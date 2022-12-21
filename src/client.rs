
use crate::contract::ContractDetails;
use crate::enums::*;
use crate::contract;
//use crate::utils::ib_message;
use crate::utils::ib_stream;
use crate::utils::ib_stream::AsyncResult;
use crate::utils::ib_message::Encodable;
use crate::account;
use crate::order;
use crate::ticker;
use crate::bars;
use crate::opt_params;
use crate::frame::IBFrame;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::{error::Error, fmt};

use rust_decimal::prelude::*;

use std::str;
use chrono::{TimeZone, DateTime};
//use chrono::format::ParseError;
//use tokio::task;
use tokio::time;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::sync::watch;
use crossbeam::channel::{self};
//use std::sync::atomic::{AtomicUsize,AtomicI32};
use futures::future::{Abortable, AbortHandle};
use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
    use log4rs::{
        append::{
            console::{ConsoleAppender, Target},
            file::FileAppender,
        },
        config::{Appender, Config, Root},
        encode::pattern::PatternEncoder,
        filter::threshold::ThresholdFilter,
};

enum Request {
    OrderID(oneshot::Sender<i32>),
    ReqWithID{id: i32, sender: oneshot::Sender<Response>},
}
enum Response {
    ContractDetails(Vec<contract::ContractDetails>),
    Order(order::OrderTracker),
    Ticker(ticker::Ticker),
    Bars(bars::BarSeries),
    OptParams(opt_params::OptParams),
    TWSError(TWSError),
    Empty
}

enum TaskState {
    Running,
    Dead
}

#[derive(Debug)]
struct ResponseError;

impl Error for ResponseError {}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid response!") // user-facing output
    }
}

#[derive(Debug)]
struct TWSError {
    code: Option<i32>,
    message: Option<String>
}

impl Error for TWSError {}

impl fmt::Display for TWSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TWS Error code: {}, message: {}", &self.code.unwrap_or(-1), &self.message.as_ref().unwrap_or(&String::from("")))
    }
}

impl TWSError {
    fn new(code: Option<i32>, message: Option<String>) -> TWSError {
        TWSError {code, message}
    }
}


#[derive(Debug)]
struct HandShakeError;

impl Error for HandShakeError {}
impl fmt::Display for HandShakeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hand shake to establish server connection failed") // user-facing output
    }
}
#[derive(Debug)]
struct SocketError;
impl Error for SocketError {}
impl fmt::Display for SocketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Socket connection to TWS/Gateway is dead.") // user-facing output
    }
}
/// The client is used to make requests and receive account updates.
pub struct IBClient
{
    client_id: i32,
    writer_abort_handle: AbortHandle,
    reader_abort_handle: AbortHandle,
    keep_alive_abort_handle: AbortHandle,
    write_tx: mpsc::Sender<String>,
    req_tx: crossbeam::channel::Sender<Request>,
    reader_state_rx: watch::Receiver<Option<TaskState>>,
    writer_state_rx: watch::Receiver<Option<TaskState>>,
    server_version: i32,
    account: account::AccountReceiver,
    next_req_id: i32,
    next_order_id: i32,
    mkt_data_setting: MarketDataType,
    log_handle: log4rs::Handle
}

impl IBClient
{
    fn init_logging(file_path: Option<&str>) -> Result<log4rs::Handle,Box<dyn Error>> {
        let level = log::LevelFilter::Info;
    
        // Build a stderr logger.
        let stderr = ConsoleAppender::builder().target(Target::Stderr).build();
    
        // Logging to log file.
        if let Some(fp) = file_path {
            let logfile = FileAppender::builder()
            // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
            .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}\n")))
            .build(fp)?;
            let config = Config::builder()
            .appender(Appender::builder().build("logfile", Box::new(logfile)))
            .appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(level)))
                    .build("stderr", Box::new(stderr)),
            )
            .build(
                Root::builder()
                    .appender("logfile")
                    .appender("stderr")
                    .build(LevelFilter::Trace),
            )?;
            Ok(log4rs::init_config(config)?)
        } else {
            let config = Config::builder()
            .appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(level)))
                    .build("stderr", Box::new(stderr)),
            )
            .build(
                Root::builder()
                    .appender("stderr")
                    .build(LevelFilter::Trace),
            )?;
            Ok(log4rs::init_config(config)?)
        }
    }

    async fn connect_socket(writer: &mut ib_stream::IBWriter, reader: &mut ib_stream::IBReader) -> AsyncResult<i32> {
        //initiate handshake
        info!("Initialize handshake.");
        writer.write_raw(b"API\0").await?;
        let mut valid_versions = constants::MIN_CLIENT_VER.to_string();
        valid_versions.push_str("..");
        valid_versions.push_str(&constants::MAX_CLIENT_VER.to_string());
        writer.write(&valid_versions).await?;
        //await handshake response
        //according to official python api, some junk messages might arrive before the server version
        //we attempt to read the valid message 10 times
        let mut reads = 10;
        loop {
            let msg = reader.read().await?;
            let msg = String::from_utf8_lossy(&msg);
            let fields: Vec<&str> = msg.split("\0").collect();
            if fields.len() == 3 {
                match fields[0].parse() {
                    Ok(v) => {info!("Handshake successful! Server version: {v}"); return Ok(v)},
                    Err(_) => return Err(Box::new(HandShakeError))
                }
            }
            reads -= 1;
            if reads <= 0 {
                error!("More than 10 messages read without handshake response.");
                return Err(Box::new(HandShakeError));
            }
        }

    }

    async fn start_api(writer: &mut ib_stream::IBWriter, client_id: i32, optional_capabilities: &str) ->AsyncResult<()> {
        let mut msg = Outgoing::StartApi.encode();
        let version : i32 = 2;
        //start API
        msg.push_str(&version.encode());
        msg.push_str(&client_id.encode());
        msg.push_str(&optional_capabilities.to_string().encode());
        info!("Sending start API message.");
        writer.write(&msg).await?;
        Ok(())
    }
    
    fn is_connected(&self) -> bool {
        if let Some(reader_state) = &*self.reader_state_rx.borrow() {
            if let Some(writer_state) = &*self.writer_state_rx.borrow() {
                let connected = match writer_state {
                    TaskState::Running => match reader_state {
                        TaskState::Running => true,
                        TaskState::Dead => false
                    }
                    TaskState::Dead => false
                };
                return connected;
            }
        }
        return false;
    }

    async fn make_request(&mut self, id: i32, msg: String) -> AsyncResult<(Response)> {
        let (resp_tx, resp_rx) = oneshot::channel();
        self.req_tx.send(Request::ReqWithID{id, sender: resp_tx})?;
        self.write_tx.send(msg).await?;
        match resp_rx.await {
            Ok(response) => 
            {
                Ok(response)
            },
            Err(err) => Err(Box::new(err))
        }
    }
    /// Connects to the TWS/Gateway on the specified port and with the specified client ID. Make sure that the port agrees
    /// with the one configured in the TWS/Gateway.
    /// Returns a connected client if the connection was successful, otherwise returns an error.
    pub async fn connect(port: i32, client_id: i32, optional_capabilities: &str, log_file: Option<&str>) -> AsyncResult<Self> {
        
        let log_handle = IBClient::init_logging(log_file)?;
        let mut addr = "127.0.0.1:".to_string();
        addr.push_str(&port.to_string());
        let stream = TcpStream::connect(addr).await?;
        let ( recv, trans) = stream.into_split();
        let mut writer = ib_stream::IBWriter::new(trans);
        let mut reader = ib_stream::IBReader::new(recv);
        let server_version = IBClient::connect_socket(&mut writer, &mut reader).await?;
        
        //println!("{:?}", server_version);
        IBClient::start_api(&mut writer, client_id, optional_capabilities).await?;

        //set up required channels
        let (tx, mut rx) = mpsc::channel(64);
        let write_tx: mpsc::Sender<String> = tx.clone();
        let (req_tx, req_rx) = channel::bounded(100);
        let (account_tx, account) = account::init_account_channel();
        let (reader_state_tx, reader_state_rx) = watch::channel(Some(TaskState::Running));
        let (writer_state_tx, writer_state_rx) = watch::channel(Some(TaskState::Running));
        //the server will send the next order ID unsolicited, just put a request on the channel to receive it
        //when the reader task starts working
        let (order_id_tx, order_id_rx) = oneshot::channel();
        req_tx.send(Request::OrderID(order_id_tx))?;

        //start the reader task
        let (reader_abort_handle, reader_abort_registration) = AbortHandle::new_pair();
        let reader_fut = Abortable::new(async move {
            //caches
            let mut positions_cache= Vec::new();
            let mut contract_details_cache: HashMap<i32,Vec<ContractDetails>> = HashMap::new();
            let mut executions_cache = HashMap::new();
            //pending requests
            let mut order_id_reqs = VecDeque::new();
            let mut requests = HashMap::new();
            //open order trackers
            let mut order_trackers = HashMap::new();
            //open tickers
            let mut tickers = HashMap::new();


            loop {
                let msg;
                match reader.read().await {
                    Ok(m) => msg = m,
                    Err(_) => {
                        //on reader error, the socket is either disconnected or the message head could not
                        //be parsed, which is also non-recoverable -> signal closure of the reader and shut
                        //down the task
                        error!("Socket read half disconnected, shutting down reader.");
                        let _ = reader_state_tx.send(Some(TaskState::Dead));
                        return;
                    }
                }
                info!("Received message: {}",String::from_utf8_lossy(&msg));
                loop {
                    match req_rx.try_recv() {
                        Ok(req) => match req {
                            Request::OrderID(sender) => {
                                order_id_reqs.push_back(sender)},
                            Request::ReqWithID{id,sender} => {
                                requests.insert(id, sender);}
                        },
                        Err(_) => break
                    }
                };
                
                if let Some(frame) = IBFrame::parse(&msg) {
                    match frame {
                        //all account channels are tied directly to the client, if these channels are closed, the client is deallocated,
                        //so we shut down the reader thread. Since the client is gone, there is no use in signaling the shutdown of the reader thread.
                        //Since the client kills the reader thread on Drop(), this should actually never happen
                        //and the result of 'send' could probably just as well be ignored.
                        IBFrame::AccountCode(code) => match account_tx.account_code.send(code) {
                            Err(_) => return,
                            _ => ()
                        },
                        IBFrame::AccountType(typ) => match account_tx.account_type.send(typ){
                            Err(_) => return,
                            _ => ()
                        },
                        IBFrame::AccountUpdateTime(time) => match account_tx.update_time.send(time){
                            Err(_) => return,
                            _ => ()
                        },
                        IBFrame::CashBalance(cash) => match account_tx.cash_balance.send(cash){
                            Err(_) => return,
                            _ => ()
                        },
                        IBFrame::EquityWithLoanValue(loan) => match account_tx.equity_with_loan_value.send(loan){
                            Err(_) => return,
                            _ => ()
                        },
                        IBFrame::ExcessLiquidity(liquidity) => match account_tx.excess_liquidity.send(liquidity){
                            Err(_) => return,
                            _ => ()
                        },
                        IBFrame::NetLiquidation(nav) => match account_tx.net_liquidation.send(nav){
                            Err(_) => return,
                            _ => ()
                        },
                        IBFrame::UnrealizedPnL(u_pnl) => match account_tx.unrealized_pnl.send(u_pnl){
                            Err(_) => return,
                            _ => ()
                        },
                        IBFrame::RealizedPnL(pnl) => match account_tx.realized_pnl.send(pnl){
                            Err(_) => return,
                            _ => ()
                        },
                        IBFrame::TotalCashBalance(balance) => match account_tx.total_cash_balance.send(balance){
                            Err(_) => return,
                            _ => ()
                        },
                        IBFrame::PortfolioValue(position) => positions_cache.push(position),
                        IBFrame::AccountUpdateEnd(_) => {
                            match account_tx.portfolio.send(Some(positions_cache)){
                                Err(_) => return,
                                _ => ()
                            };
                            positions_cache = Vec::new();},
                        IBFrame::CurrentTime(dtime) => println!("Heartbeat at {}", dtime),
                        IBFrame::OrderID(id) => {
                            match order_id_reqs.pop_front() {
                                //ignore potential closure of the channel, as it just means the requestor is dead
                                //potentially just log this event, once a logger is implemented
                                Some(sender) => { let _ = sender.send(id);},
                                None => println!("No pending order id request.")
                            }
                        },
                        IBFrame::ContractDetails{req_id: id,contract_details: details} => {
                            //contract_details_cache.entry(id).or_insert(Vec::new());
                            match contract_details_cache.get_mut(&id){
                                Some(v) => v.push(details),
                                None => {let _ = contract_details_cache.insert(id, vec![details]);}}
                        },
                        IBFrame::ContractDetailsEnd(req_id) => {
                            match requests.remove_entry(&req_id) {
                                Some((_, sender)) => {
    
                                    let _res = match contract_details_cache.remove_entry(&req_id) {
                                        Some((_, details)) => sender.send(Response::ContractDetails(details)),
                                        None => sender.send(Response::Empty)
                                    };
    
                                },
                                None => println!("No pending contract details request for req_id {}", req_id)
                            };
                        },
                        IBFrame::OpenOrder{order,order_state} => {
                            let order_id = order.order_id;
                            match requests.remove_entry(&order_id) {
                                Some((_, sender)) => {
                                    let (order_sender, order_receiver) = order::OrderTracker::new(order, order_state);
                                    match sender.send(Response::Order(order_receiver)){
                                        Ok(()) => {order_trackers.insert(order_id, order_sender);},
                                        Err(_) => ()
                                    }
                                    
                                },
                                None => {
                                    let mut tracker_dead: bool = false;
                                    if let Some(tracker) = order_trackers.get(&order_id) {
                                        match tracker.order_state_tx.send(order_state) {
                                            Err(_) => {tracker_dead = true;},
                                            _ => ()
                                        }
                                        match tracker.order_tx.send(order){
                                            Err(_) => {tracker_dead = true;},
                                            _ => ()
                                        }
                                    }
                                    if tracker_dead {
                                        order_trackers.remove(&order_id);
                                    }
                                }
                            }
                            
                        },
                        IBFrame::Execution(execution) => {
                            let mut tracker_dead: bool = false;
                            let order_id = execution.order_id;
                            let exec_id = execution.exec_id.clone();
                            if let Some(tracker) = order_trackers.get_mut(&execution.order_id) {
                                
                                match tracker.executions_tx.send(execution){
                                    Err(_) => {tracker_dead = true;},
                                    Ok(()) => {executions_cache.insert(exec_id, order_id);}
                                }
                            }
                            if tracker_dead {
                                order_trackers.remove(&order_id);
                            }
                        },
                        IBFrame::CommissionReport(report) => {
                            let mut tracker_dead = false;
                            if let Some((_,order_id)) = executions_cache.remove_entry(&report.exec_id) {
                                if let Some(tracker) = order_trackers.get_mut(&order_id) {
                                    match tracker.commission_reports_tx.send(report) {
                                        Err(_error) => tracker_dead = true,
                                        _ => ()
                                    }
                                }
                                if tracker_dead {
                                    order_trackers.remove(&order_id);
                                }
                            }
    
                        },
                        IBFrame::OrderStatus(status) => {
                            let mut tracker_dead = false;
                            let order_id = status.order_id;
                            if let Some(tracker) = order_trackers.get(&status.order_id) {
                                match tracker.order_status_tx.send(Some(status)) {
                                    Err(_error) => tracker_dead = true,
                                    _ => ()
                                }
                            }
                            if tracker_dead {
                                order_trackers.remove(&order_id);
                            }
                        }
                        IBFrame::PriceTick{id, kind, price, size, ..} => {
                            if let Some((_, req)) = requests.remove_entry(&id) {
                                let (ticker_sender, ticker) = ticker::Ticker::new();
                                if let Ok(()) = req.send(Response::Ticker(ticker)) {tickers.insert(id, ticker_sender);} else {continue}; //else: request is dead
                            }
                            if let Some(t) = tickers.get_mut(&id) {
                                let ok = match kind {
                                    TickType::Bid | TickType::DelayedBid => {
                                        if let Err(_) = t.bid.send(Some(price)) {false}
                                        else if let Err(_) = t.bid_size.send(size) {false}
                                        else {true}
                                    },
                                    TickType::Ask | TickType::DelayedAsk => {
                                        if let Err(_) = t.ask.send(Some(price)) {false}
                                        else if let Err(_) = t.ask_size.send(size) {false}
                                        else {true}
    
                                    },
                                    TickType::Last | TickType::DelayedLast => {
                                        if let Err(_) = t.last.send(Some(price)) {false}
                                        else if let Err(_) = t.last_size.send(size) {false}
                                        else {true}
                                    }
                                    _ => true
                                };
                                if !ok {tickers.remove_entry(&id);}    //ticker dead
                            };
                        },
                        IBFrame::SizeTick{id, kind, size} => {
                            if let Some((_, req)) = requests.remove_entry(&id) {
                                let (ticker_sender, ticker) = ticker::Ticker::new();
                                tickers.insert(id, ticker_sender);
                                if let Ok(()) = req.send(Response::Ticker(ticker)) {} else {continue}; //else: request is dead
                            }
                            if let Some(t) = tickers.get_mut(&id) {
                                let ok = match kind {
                                    TickType::BidSize | TickType::DelayedBidSize => {
                                        if let Err(_) = t.bid_size.send(Some(size)) {false}
                                        else {true}
                                    },
                                    TickType::AskSize | TickType::DelayedAskSize => {
                                        if let Err(_) = t.ask_size.send(Some(size)) {false}
                                        else {true}
    
                                    },
                                    TickType::LastSize | TickType::DelayedLastSize => {
                                        if let Err(_) = t.last_size.send(Some(size)) {false}
                                        else {true}
                                    }
                                    TickType::ShortableShares => {
                                        if let Err(_) = t.shortable_shares.send(Some(size)) {false}
                                        else {true}
                                    }
                                    _ => true
                                };
                                if !ok {tickers.remove_entry(&id);}    //ticker dead
                            };
                        },
                        IBFrame::GenericTick{id, kind, val} => {
                            if let Some((_, req)) = requests.remove_entry(&id) {
                                let (ticker_sender, ticker) = ticker::Ticker::new();
                                tickers.insert(id, ticker_sender);
                                if let Ok(()) = req.send(Response::Ticker(ticker)) {} else {continue}; //else: request is dead
                            }
                            if let Some(t) = tickers.get_mut(&id) {
                                let ok = match kind {
                                    TickType::Shortable => {
                                        if let Err(_) = t.short_availability.send(Some(ticker::ShortAvailability::from_f64(val))) {false}
                                        else {true}
                                    }
                                    _ => true
                                };
                                if !ok {tickers.remove_entry(&id);}    //ticker is dead
                            };
                        },
                        IBFrame::Bars{id, data} => {
                            if let Some((_, req)) = requests.remove_entry(&id) {
                                let _ = req.send(Response::Bars(data));
                            }
                        }
                        IBFrame::Error{id, code, msg} => {
                            if let Some(idval) = id {
                                match requests.remove_entry(&idval) {
                                    Some((_, tx)) => {let _ = tx.send(Response::TWSError(TWSError::new(code,msg)));},
                                    None => ()
                                };
                            };
                            
                        }
                        _ => warn!("Message ignored by client because not currently implemented.")
                    };
                } else {warn!("Message could not be parsed and is ignored!");}
                
            }
        }, reader_abort_registration);
        let _reader_task = tokio::spawn(reader_fut);
        //now await receipt of the next order id before anything else happens (ensures that the API is ready)
        let mut next_order_id = 0;
        match order_id_rx.await {
            Ok(id) => next_order_id = id,
            Err(err) => return Err(Box::new(err))
        }
        //start the writer task managing the write half of the socket
        let (writer_abort_handle, writer_abort_registration) = AbortHandle::new_pair();
        let writer_fut = Abortable::new(async move {
            loop {
                match rx.recv().await {
                    Some(msg) => match writer.write(&msg).await {
                        Err(_) => {let _ = writer_state_tx.send(Some(TaskState::Dead)); return;}
                        _ => info!("Message send: {msg}")
                    },
                    None => {let _ = writer_state_tx.send(Some(TaskState::Dead)); return;}
                }
            }
        }, writer_abort_registration);
        let _writer_task = tokio::spawn(writer_fut);

        //start the keep alive task to send a message across the socket every minute
        let (keep_alive_abort_handle, keep_alive_abort_registration) = AbortHandle::new_pair();
        let keep_alive_fut = Abortable::new(async move{
            let mut msg = Outgoing::ReqCurrentTime.encode();
            msg.push_str(&1i32.encode());
            loop{
                if let Err(_) = tx.send(msg.clone()).await {
                    return;
                }
                time::sleep(time::Duration::from_secs(60)).await;
            }
        }, keep_alive_abort_registration);
        let _keep_alive_task = tokio::spawn(keep_alive_fut);
        let mut client = IBClient {
            client_id,
            writer_abort_handle,
            reader_abort_handle,
            keep_alive_abort_handle,
            write_tx,
            req_tx,
            reader_state_rx,
            writer_state_rx,
            server_version,
            account,
            next_req_id: 0,
            next_order_id,
            mkt_data_setting: MarketDataType::RealTime,
            log_handle
        };
        //subscribe to account updates
        let mut msg = Outgoing::ReqAcctData.encode();
        msg.push_str(&2i32.encode());
        msg.push_str(&true.encode());
        msg.push_str("\0");
        client.write_tx.send(msg).await?;
        
        Ok(client)
    }
    /// Returns the current net liquidation value of the account. If no value was received yet, returns `None`.
    pub fn net_liquidation_value(&self) -> Option<Decimal> {
        *self.account.net_liquidation.borrow()
    }
    /// Returns the current cash balance of the account. If no value was received yet, returns `None`.
    pub fn cash_balance(&self) -> Option<Decimal> {
        *self.account.cash_balance.borrow()
    }
    /// Returns the excess liquidity of the account. If no value was received yet, returns `None`.
    pub fn excess_liquidity(&self) -> Option<Decimal> {
        *self.account.excess_liquidity.borrow()
    }

    fn get_next_req_id(&mut self) -> i32 {
        self.next_req_id += 1;
        self.next_req_id
    }

    fn get_next_order_id(&mut self) -> i32 {
        self.next_order_id += 1;
        self.next_order_id
    }
    /// Requests contract details for the given `Contract`.
    pub async fn req_contract_details(&mut self, contract: &contract::Contract) -> AsyncResult<Vec<contract::ContractDetails>> {
        if !self.is_connected() {
            return Err(Box::new(SocketError));
        }
        let mut msg = Outgoing::ReqContractData.encode();
        msg.push_str(&8i32.encode());
        let id = self.get_next_req_id();
        msg.push_str(&id.encode());
        msg.push_str(&contract.encode());
        match self.make_request(id, msg).await? {
            Response::ContractDetails(contracts) => Ok(contracts),
            Response::TWSError(error) => Err(Box::new(error)),
            _ => Err(Box::new(ResponseError{}))
        }
    }
    /// Places an order. An `OrderTracker` is returned which can be used to monitor the order execution.
    pub async fn place_order(&mut self, order: &order::Order) -> AsyncResult<order::OrderTracker> {
        if !self.is_connected() {
            return Err(Box::new(SocketError));
        }
        let mut msg = Outgoing::PlaceOrder.encode();
        let id = self.get_next_order_id();
        msg.push_str(&id.encode());
        msg.push_str(&order.encode());
        match self.make_request(id, msg).await? {
            Response::Order(tracker) => Ok(tracker),
            Response::TWSError(error) => Err(Box::new(error)),
            _ => Err(Box::new(ResponseError{}))
        }
    }
    /// Requests real-time or delayed market data. A `Ticker` is returned which will receive streaming market data.
    pub async fn req_market_data(&mut self, contract: &contract::Contract, snapshot: bool, regulatory: bool, 
        additional_data: Option<Vec<GenericTickType>>) -> AsyncResult<ticker::Ticker> {
        if !self.is_connected() {
            return Err(Box::new(SocketError));
        }
        let mut msg = Outgoing::ReqMktData.encode();
        msg.push_str("11\0"); //version
        let id = self.get_next_req_id();
        msg.push_str(&id.encode());
        msg.push_str(&contract.encode_for_ticker());
        
        if let Some(add_data) = additional_data {
            for i in 0..add_data.len()-1 {
                msg.push_str(&add_data[i].encode());
                msg.push_str(",");
            }
            if let Some(gen_tick) = add_data.last() {
                msg.push_str(&gen_tick.encode());
            }
        }
        
        msg.push_str("\0"); //generic tick data
        msg.push_str(&snapshot.encode());
        msg.push_str(&regulatory.encode());
        msg.push_str("\0"); //market data options
        match self.make_request(id, msg).await? {
            Response::Ticker(ticker) => Ok(ticker),
            Response::TWSError(error) => Err(Box::new(error)),
            _ => Err(Box::new(ResponseError{}))
        }
    }
    /// Requests historical price bar data.
    pub async fn req_historical_data<Tz: TimeZone> (&mut self, contract: &contract::Contract, end_date_time: &DateTime<Tz>, 
        duration: HistoricalDataDuration, bar_period: HistoricalDataBarSize, what_to_show: HistoricalDataType, use_rth: bool) -> AsyncResult<bars::BarSeries>
        where
        <Tz as TimeZone>::Offset: std::fmt::Display
        {
        if !self.is_connected() {
            return Err(Box::new(SocketError));
        }
        let mut msg = Outgoing::ReqHistoricalData.encode();
        let id = self.get_next_req_id();
        msg.push_str(&id.encode());
        msg.push_str(&contract.encode_for_hist_data());
        msg.push_str(&end_date_time.format("%Y%m%d %H:%M:%S").to_string().encode());
        msg.push_str(&bar_period.encode());
        msg.push_str(&duration.encode());
        msg.push_str(&use_rth.encode());
        msg.push_str(&what_to_show.encode());
        msg.push_str("1\00\0\0");
        match self.make_request(id, msg).await? {
            Response::Bars(bars) => Ok(bars),
            Response::TWSError(error) => Err(Box::new(error)),
            _ => Err(Box::new(ResponseError{}))
        }
    }
    /// Requests historical price bar data adjusted for dividends and splits (stocks only).
    pub async fn req_adj_historical_data(&mut self, contract: &contract::Contract, duration: HistoricalDataDuration, bar_period: HistoricalDataBarSize, use_rth: bool) -> AsyncResult<bars::BarSeries> {
        if !self.is_connected() {
            return Err(Box::new(SocketError));
        }
        let mut msg = Outgoing::ReqHistoricalData.encode();
        let id = self.get_next_req_id();
        msg.push_str(&id.encode());
        msg.push_str(&contract.encode_for_hist_data());
        msg.push_str("\0");
        msg.push_str(&bar_period.encode());
        msg.push_str(&duration.encode());
        msg.push_str(&use_rth.encode());
        msg.push_str("ADJUSTED_LAST\0");
        msg.push_str("1\00\0\0");
        match self.make_request(id, msg).await? {
            Response::Bars(bars) => Ok(bars),
            Response::TWSError(error) => Err(Box::new(error)),
            _ => Err(Box::new(ResponseError{}))
        }
    }
    /// Configures market data type as delayed data (no real-time subscription required).
    pub async fn set_mkt_data_delayed(&mut self) -> AsyncResult<()> {
        if !self.is_connected() {
            return Err(Box::new(SocketError));
        }
        let mut msg = Outgoing::ReqMarketDataType.encode();
        msg.push_str("1\0");
        msg.push_str(&MarketDataType::Delayed.encode());
        self.write_tx.send(msg).await?;
        self.mkt_data_setting = MarketDataType::Delayed;
        Ok(())
    }
    /// Configures market data type as real-time (subscription required).
    pub async fn set_mkt_data_real_time(&mut self) -> AsyncResult<()> {
        if !self.is_connected() {
            return Err(Box::new(SocketError));
        }
        let mut msg = Outgoing::ReqMarketDataType.encode();
        msg.push_str("1\0");
        msg.push_str(&MarketDataType::RealTime.encode());
        self.write_tx.send(msg).await?;
        self.mkt_data_setting = MarketDataType::RealTime;
        Ok(())
    }

    pub async fn req_options_metadata(&mut self, contract: &contract::Contract, exchange: Option<&str>) ->AsyncResult<()> {
        if !self.is_connected() {
            return Err(Box::new(SocketError));
        }
        
        if contract.symbol.is_some() && contract.sec_type.is_some() && contract.con_id.is_some() {
            let mut msg = Outgoing::ReqSecDefOptParams.encode();
            let id = self.get_next_req_id();
            msg.push_str(&id.encode());
            msg.push_str(&contract.symbol.encode());
            msg.push_str(&exchange.encode());
            msg.push_str(&contract.con_id.encode());
            match self.make_request(id, msg).await? {
                _ => Err(Box::new(ResponseError{}))
            }
        }
        else {Ok(())}
    }

}
/// The `IBClient` shuts down all detached tasks used to manage the socket connection on `Drop`.
impl Drop for IBClient {
    fn drop(&mut self) {
        self.keep_alive_abort_handle.abort();
        self.writer_abort_handle.abort();
        self.reader_abort_handle.abort();
    }
}