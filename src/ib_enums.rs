use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::str::FromStr;
use crate::utils::ib_message::Encodable;
use crate::utils::ib_message::Decodable;

use enum_ordinalize;

pub mod constants {
    pub const CLIENT_VERSION: i32 = 66;
    pub const MIN_SERVER_VER_PRICE_MGMT_ALGO: i32 = 151;
    pub const MIN_CLIENT_VER: i32 = 100;
    pub const MAX_CLIENT_VER: i32 = MIN_SERVER_VER_PRICE_MGMT_ALGO;
}

#[derive(FromPrimitive)]
pub enum Incoming {
    TickPrice                                = 1,
    TickSize                                 = 2,
    OrderStatus                              = 3,
    ErrMsg                                   = 4,
    OpenOrder                                = 5,
    AcctValue                                = 6,
    PortfolioValue                           = 7,
    AcctUpdateTime                          = 8,
    NextValidId                             = 9,
    ContractData                             = 10,
    ExecutionData                            = 11,
    MarketDepth                              = 12,
    MarketDepthL2                           = 13,
    NewsBulletins                            = 14,
    ManagedAccts                             = 15,
    ReceiveFa                                = 16,
    HistoricalData                           = 17,
    BondContractData                        = 18,
    ScannerParameters                        = 19,
    ScannerData                              = 20,
    TickOptionComputation                   = 21,
    TickGeneric                              = 45,
    TickString                               = 46,
    TickEfp                                  = 47,
    CurrentTime                              = 49,
    RealTimeBars                            = 50,
    FundamentalData                          = 51,
    ContractDataEnd                         = 52,
    OpenOrderEnd                            = 53,
    AcctDownloadEnd                         = 54,
    ExecutionDataEnd                        = 55,
    DeltaNeutralValidation                  = 56,
    TickSnapshotEnd                         = 57,
    MarketDataType                          = 58,
    CommissionReport                         = 59,
    PositionData                             = 61,
    PositionEnd                              = 62,
    AccountSummary                           = 63,
    AccountSummaryEnd                       = 64,
    VerifyMessageApi                        = 65,
    VerifyCompleted                          = 66,
    DisplayGroupList                        = 67,
    DisplayGroupUpdated                     = 68,
    VerifyAndAuthMessageApi               = 69,
    VerifyAndAuthCompleted                 = 70,
    PositionMulti                            = 71,
    PositionMultiEnd                        = 72,
    AccountUpdateMulti                      = 73,
    AccountUpdateMultiEnd                  = 74,
    SecurityDefinitionOptionParameter      = 75,
    SecurityDefinitionOptionParameterEnd  = 76,
    SoftDollarTiers                         = 77,
    FamilyCodes                              = 78,
    SymbolSamples                            = 79,
    MktDepthExchanges                       = 80,
    TickReqParams                           = 81,
    SmartComponents                          = 82,
    NewsArticle                              = 83,
    TickNews                                 = 84,
    NewsProviders                            = 85,
    HistoricalNews                           = 86,
    HistoricalNewsEnd                       = 87,
    HeadTimestamp                            = 88,
    HistogramData                            = 89,
    HistoricalDataUpdate                    = 90,
    RerouteMktDataReq                      = 91,
    RerouteMktDepthReq                     = 92,
    MarketRule                               = 93,
    PnL                                       = 94,
    PnlSingle                                = 95,
    HistoricalTicks                          = 96,
    HistoricalTicksBidAsk                  = 97,
    HistoricalTicksLast                     = 98,
    TickByTick                              = 99,
    OrderBound                               = 100,
    CompletedOrder                            = 101,
    CompletedOrdersEnd                        = 102
}

impl FromStr for Incoming {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ord = match s.parse::<i32>() {
            Ok(n) => n,
            Err(_) => return Err(ParseEnumError),
        };
        match FromPrimitive::from_i32(ord) {
            Some(en_type) => return Ok(en_type),
            None => return Err(ParseEnumError),
        };
    }
}

#[derive(enum_ordinalize::Ordinalize)]
pub enum Outgoing {
    // outgoing message IDs
    ReqMktData                  = 1, 
    CancelMktData               = 2, 
    PlaceOrder                   = 3, 
    CancelOrder                  = 4, 
    ReqOpenOrders               = 5, 
    ReqAcctData                 = 6, 
    ReqExecutions                = 7, 
    ReqIds                       = 8, 
    ReqContractData             = 9, 
    ReqMktDepth                 = 10, 
    CancelMktDepth              = 11, 
    ReqNewsBulletins            = 12, 
    CancelNewsBulletins         = 13, 
    SetServerLoglevel           = 14, 
    ReqAutoOpenOrders          = 15, 
    ReqAllOpenOrders           = 16, 
    ReqManagedAccts             = 17, 
    ReqFa                        = 18, 
    ReplaceFa                    = 19, 
    ReqHistoricalData           = 20, 
    ExerciseOptions              = 21, 
    ReqScannerSubscription      = 22, 
    CancelScannerSubscription   = 23, 
    ReqScannerParameters        = 24, 
    CancelHistoricalData        = 25, 
    ReqCurrentTime              = 49, 
    ReqRealTimeBars            = 50, 
    CancelRealTimeBars         = 51, 
    ReqFundamentalData          = 52, 
    CancelFundamentalData       = 53, 
    ReqCalcImpliedVolat        = 54, 
    ReqCalcOptionPrice         = 55, 
    CancelCalcImpliedVolat     = 56, 
    CancelCalcOptionPrice      = 57, 
    ReqGlobalCancel             = 58, 
    ReqMarketDataType          = 59, 
    ReqPositions                 = 61, 
    ReqAccountSummary           = 62, 
    CancelAccountSummary        = 63, 
    CancelPositions              = 64, 
    VerifyRequest                = 65, 
    VerifyMessage                = 66, 
    QueryDisplayGroups          = 67, 
    SubscribeToGroupEvents     = 68, 
    UpdateDisplayGroup          = 69, 
    UnsubscribeFromGroupEvents = 70, 
    StartApi                     = 71, 
    VerifyAndAuthRequest       = 72, 
    VerifyAndAuthMessage       = 73, 
    ReqPositionsMulti           = 74, 
    CancelPositionsMulti        = 75, 
    ReqAccountUpdatesMulti     = 76, 
    CancelAccountUpdatesMulti  = 77, 
    ReqSecDefOptParams        = 78, 
    ReqSoftDollarTiers         = 79, 
    ReqFamilyCodes              = 80, 
    ReqMatchingSymbols          = 81, 
    ReqMktDepthExchanges       = 82, 
    ReqSmartComponents          = 83, 
    ReqNewsArticle              = 84, 
    ReqNewsProviders            = 85, 
    ReqHistoricalNews           = 86, 
    ReqHeadTimestamp            = 87, 
    ReqHistogramData            = 88, 
    CancelHistogramData         = 89, 
    CancelHeadTimestamp         = 90, 
    ReqMarketRule               = 91, 
    ReqPnl                       = 92, 
    CancelPnl                    = 93, 
    ReqPnlSingle                = 94, 
    CancelPnlSingle             = 95, 
    ReqHistoricalTicks          = 96, 
    ReqTickByTickData         = 97, 
    CancelTickByTickData      = 98, 
    ReqCompletedOrders          = 99 
}

impl Encodable for Outgoing {
    fn encode(&self) -> String {
        let ord = self.ordinal();
        ord.to_string() + "\0"
    }
}

#[derive(Debug)]
pub struct ParseEnumError;
// Some enums are only for decoding and implement the FromStr trait
// Some enums are only for encoding and implement the encode method (might make it a trait)

#[derive(FromPrimitive,Debug,Clone)]
pub enum TickType {
    BidSize,
    Bid,
    Ask,
    AskSize,
    Last,
    LastSize,
    High,
    Low,
    Volume,
    Close,
    BidOptionComputation,
    AskOptionComputation,
    LastOptionComputation,
    ModelOption,
    Open,
    Low13Week,
    High13Week,
    Low26Week,
    High26Week,
    Low52Week,
    High52Week,
    AvgVolume,
    OpenInterest,
    OptionHistoricalVol,
    OptionImpliedVol,
    OptionBidExch,
    OptionAskExch,
    OptionCallOpenInterest,
    OptionPutOpenInterest,
    OptionCallVolume,
    OptionPutVolume,
    IndexFuturePremium,
    BidExch,
    AskExch,
    AuctionVolume,
    AuctionPrice,
    AuctionImbalance,
    MarkPrice,
    BidEfpComputation,
    AskEfpComputation,
    LastEfpComputation,
    OpenEfpComputation,
    HighEfpComputation,
    LowEfpComputation,
    CloseEfpComputation,
    LastTimestamp,
    Shortable,
    FundamentalRatios,
    RtVolume,
    Halted,
    BidYield,
    AskYield,
    LastYield,
    CustOptionComputation,
    TradeCount,
    TradeRate,
    VolumeRate,
    LastRthTrade,
    RtHistoricalVol,
    IbDividends,
    BondFactorMultiplier,
    RegulatoryImbalance,
    NewsTick,
    ShorttermVolume3min,
    ShorttermVolume5min,
    ShorttermVolume10min,
    DelayedBid,
    DelayedAsk,
    DelayedLast,
    DelayedBidSize,
    DelayedAskSize,
    DelayedLastSize,
    DelayedHigh,
    DelayedLow,
    DelayedVolume,
    DelayedClose,
    DelayedOpen,
    RtTrdVolume,
    CreditManMarkPrice,
    CreditManSlowMarkPrice,
    DelayedBidOptionComputation,
    DelayedAskOptionComputation,
    DelayedLastOptionComputation,
    DelayedModelOptionComputation,
    LastExch,
    LastRegTime,
    FuturesOpenInterest,
    AvgOptVolume,
    DelayedLastTimestamp,
    ShortableShares,
    NotSet,
}

impl FromStr for TickType {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ord = match s.parse::<i32>() {
            Ok(n) => n,
            Err(_) => return Err(ParseEnumError),
        };
        match FromPrimitive::from_i32(ord) {
            Some(en_type) => return Ok(en_type),
            None => return Err(ParseEnumError),
        };
    }
}

impl Decodable for TickType {}

#[derive(Debug,Clone)]
pub enum GenericTickType {
    ShortableData,
    HistoricData,
    OptionHistoricalVol,
    OptionImpliedVol,
    OptionOpenInterest,
    AuctionData,
    OptionVolume,
}

impl Encodable for GenericTickType {
    fn encode(&self) -> String {
        match self {
            GenericTickType::ShortableData => "236",
            GenericTickType::HistoricData => "165",
            GenericTickType::OptionHistoricalVol => "10",
            GenericTickType::OptionImpliedVol => "106",
            GenericTickType::OptionOpenInterest => "101",
            GenericTickType::AuctionData => "225",
            GenericTickType::OptionVolume => "100",
        }.to_string()
    }
}

#[derive(Debug,Clone)]
pub enum MarketDataType {
    RealTime = 1,
    Frozen = 2,
    Delayed = 3,
    FrozenDelayed = 4
}

impl Encodable for MarketDataType {
    fn encode(&self) -> String {
        match self {
            MarketDataType::RealTime => "1\0",
            MarketDataType::Frozen => "2\0",
            MarketDataType::Delayed => "3\0",
            MarketDataType::FrozenDelayed => "4\0"
        }.to_string()
    }
}

impl FromStr for MarketDataType {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "1" => MarketDataType::RealTime,
            "2" => MarketDataType::Frozen,
            "3" => MarketDataType::Delayed,
            "4" => MarketDataType::FrozenDelayed,
            &_ => return Err(ParseEnumError)
        };
        Ok(res)
    }
}

impl Decodable for MarketDataType {}

#[derive(Debug,Clone)]
pub enum FundamentalDataType {
    Snapshot,
    FinSummary,
    Ratios,
    FinStatements,
    Estimates,
}

impl Encodable for FundamentalDataType {
    fn encode(&self) -> String {
        match self {
            FundamentalDataType::Snapshot => "ReportSnapShot\0",
            FundamentalDataType::FinSummary => "ReportsFinSummary\0",
            FundamentalDataType::Ratios => "ReportRatios\0",
            FundamentalDataType::FinStatements => "ReportsFinStatements\0",
            FundamentalDataType::Estimates => "RESC\0",
        }.to_string()
    }
}
#[derive(Debug,PartialEq,Eq,Clone)]
pub enum SecType {
    Stock,
    Option,
    Future,
    OptionOnFuture,
    Index,
    Forex,
    Combo,
    Warrant,
    Bond,
    Commodity,
    News,
    MutualFund,
}

impl Encodable for SecType {
    fn encode(&self) -> String {
        match self {
            SecType::Stock => "STK\0",
            SecType::Option => "OPT\0",
            SecType::Future => "FUT\0",
            SecType::OptionOnFuture => "FOP\0",
            SecType::Index => "IND\0",
            SecType::Forex => "CASH\0",
            SecType::Combo => "BAG\0",
            SecType::Warrant => "WAR\0",
            SecType::Bond => "BOND\0",
            SecType::Commodity => "CMDTY\0",
            SecType::News => "NEWS\0",
            SecType::MutualFund => "FUND\0",
        }.to_string()
    }
}

impl FromStr for SecType {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "STK" => SecType::Stock,
            "OPT" => SecType::Option,
            "FUT" => SecType::Future,
            "FOP" => SecType::OptionOnFuture,
            "IND" => SecType::Index,
            "CASH" => SecType::Forex,
            "BAG" => SecType::Combo,
            "WAR" => SecType::Warrant,
            "BOND" => SecType::Bond,
            "CMDTY" => SecType::Commodity,
            "NEWS" => SecType::News,
            "FUND" => SecType::MutualFund,
            &_ => return Err(ParseEnumError)
        };
        Ok(res)
    }
}

impl Decodable for SecType {}
#[derive(Debug,Clone)]
pub enum OptionRight {
    Undefined,
    Put,
    Call,
}

impl Encodable for  OptionRight {
    fn encode(&self) -> String {
        match self {
            OptionRight::Undefined => "0\0",
            OptionRight::Put => "PUT\0",
            OptionRight::Call => "CALL\0",
        }.to_string()
    }
}

impl FromStr for OptionRight {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "0" => OptionRight::Undefined,
            "?" => OptionRight::Undefined,
            "PUT" => OptionRight::Put,
            "CALL" => OptionRight::Call,
            "P" => OptionRight::Put,
            "C" => OptionRight::Call,
            &_ => return Err(ParseEnumError)
        };
        Ok(res)
    }
}

impl Decodable for OptionRight {}
#[derive(Debug,Clone)]
pub enum SecIdType {
    Isin,
    Cusip,
}

impl Encodable for SecIdType {
    fn encode(&self) -> String {
        match self {
            SecIdType::Isin => "ISIN\0",
            SecIdType::Cusip => "CUSIP\0",
        }.to_string()
    }
}

#[derive(Debug,Clone)]
pub enum ComboAction {
    Buy,
    Sell,
    ShortSell,
}

impl Encodable for ComboAction {
    fn encode(&self) -> String {
        match self {
            ComboAction::Buy => "BUY\0",
            ComboAction::Sell => "SELL\0",
            ComboAction::ShortSell => "SSELL\0",
        }.to_string()
    }
}

impl FromStr for ComboAction {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BUY" => Ok(ComboAction::Buy),
            "SELL" => Ok(ComboAction::Sell),
            "SSELL" => Ok(ComboAction::ShortSell),
            &_ => Err(ParseEnumError)
        }
    }
}

impl Decodable for ComboAction { }

#[derive(Debug,Clone)]
pub enum OptionOpenClose {
    Same,
    Open,
    Close,
    Unknown,
}

impl Encodable for OptionOpenClose {
    fn encode(&self) -> String {
        match self {
            OptionOpenClose::Same => "0\0",
            OptionOpenClose::Open => "1\0",
            OptionOpenClose::Close => "2\0",
            OptionOpenClose::Unknown => "3\0",
        }.to_string()
    }
}

impl FromStr for OptionOpenClose {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(OptionOpenClose::Same),
            "1" => Ok(OptionOpenClose::Open),
            "2" => Ok(OptionOpenClose::Close),
            "3" => Ok(OptionOpenClose::Unknown),
            &_ => Err(ParseEnumError)
        }
    }
}

impl Decodable for OptionOpenClose {}

#[derive(Debug,Clone)]
pub enum ShortSaleSlot {
    NoSlot,
    Broker,
    ThirdParty,
}

impl Encodable for ShortSaleSlot {
    fn encode(&self) -> String {
        match self {
            ShortSaleSlot::NoSlot => "0\0",
            ShortSaleSlot::Broker => "1\0",
            ShortSaleSlot::ThirdParty => "2\0"
        }.to_string()
    }
}

impl FromStr for ShortSaleSlot {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(ShortSaleSlot::NoSlot),
            "1" => Ok(ShortSaleSlot::Broker),
            "2" => Ok(ShortSaleSlot::ThirdParty),
            &_ => Err(ParseEnumError)
        }
    }
}

impl Decodable for ShortSaleSlot {}

#[derive(Debug,Clone)]
pub enum Action {
    Buy,
    Sell,
    SellShort,
    SellLong,
}

impl Encodable for Action {
    fn encode(&self) -> String {
        match self {
            Action::Buy => "BUY\0",
            Action::Sell => "SELL\0",
            Action::SellShort => "SSELL\0",
            Action::SellLong => "SLONG\0",
        }.to_string()
    }
}

impl Default for Action {
    fn default() -> Self {
        Action::Buy
    }
}

impl FromStr for Action {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BUY" => Ok(Action::Buy),
            "SELL" => Ok(Action::Sell),
            "SSELL" => Ok(Action::SellShort),
            "SLONG" => Ok(Action::SellLong),
            &_ => Err(ParseEnumError)
        }
    }
}

impl Decodable for Action {}

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum OrderType {
    NoOrderType, //only legit for deltaNeutralOrderType
    Limit,
    Market,
    MarketIfTouched,
    MarketOnClose,
    MarketOnOpen,
    PeggedToMarket,
    PeggedToStock,
    PeggedToPrimary,
    BoxTop,
    LimitIfTouched,
    LimitOnClose,
    PassiveRelative,
    PeggedToMidpoint,
    MarketToLimit,
    MarketWithProtection,
    Stop,
    StopLimit,
    StopWithProtection,
    TrailingStop,
    TrailingStopLimit,
    RelativeLimit,
    RelativeMarket,
    Volatility,
    PeggedToBenchmark,
}

impl Encodable for OrderType {
    fn encode(&self) -> String {
        match self {
            OrderType::NoOrderType => "None\0",
            OrderType::Limit => "LMT\0",
            OrderType::Market => "MKT\0",
            OrderType::MarketIfTouched => "MIT\0",
            OrderType::MarketOnClose => "MOC\0",
            OrderType::MarketOnOpen => "MOO\0",
            OrderType::PeggedToMarket => "PEG MKT\0",
            OrderType::PeggedToStock => "PEG STK\0",
            OrderType::PeggedToPrimary => "REL\0",
            OrderType::BoxTop => "BOX TOP\0",
            OrderType::LimitIfTouched => "LIT\0",
            OrderType::LimitOnClose => "LOC\0",
            OrderType::PassiveRelative => "PASSV REL\0",
            OrderType::PeggedToMidpoint => "PEG MID\0",
            OrderType::MarketToLimit => "MTL\0",
            OrderType::MarketWithProtection => "MKT PRT\0",
            OrderType::Stop => "STP\0",
            OrderType::StopLimit => "STP LMT\0",
            OrderType::StopWithProtection => "STP PRT\0",
            OrderType::TrailingStop => "TRAIL\0",
            OrderType::TrailingStopLimit => "TRAIL LIMIT\0",
            OrderType::RelativeLimit => "Rel + LMT\0",
            OrderType::RelativeMarket => "Rel + MKT\0",
            OrderType::Volatility => "VOL\0",
            OrderType::PeggedToBenchmark => "PEG BENCH\0",
        }.to_string()
    }
}

impl FromStr for OrderType {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(OrderType::NoOrderType),
            "LMT" => Ok(OrderType::Limit),
            "MKT" => Ok(OrderType::Market),
            "MIT" => Ok(OrderType::MarketIfTouched),
            "MOC" => Ok(OrderType::MarketOnClose),
            "MOO" => Ok(OrderType::MarketOnOpen),
            "PEG MKT" => Ok(OrderType::PeggedToMarket),
            "PEG STK" => Ok(OrderType::PeggedToStock),
            "REL" => Ok(OrderType::PeggedToPrimary),
            "BOX TOP" => Ok(OrderType::BoxTop),
            "LIT" => Ok(OrderType::LimitIfTouched),
            "LOC" => Ok(OrderType::LimitOnClose),
            "PASSV REL" => Ok(OrderType::PassiveRelative),
            "PEG MID" => Ok(OrderType::PeggedToMidpoint),
            "MTL" => Ok(OrderType::MarketToLimit),
            "MKT PRT" => Ok(OrderType::MarketWithProtection),
            "STP" => Ok(OrderType::Stop),
            "STP LMT" => Ok(OrderType::StopLimit),
            "STP PRT" => Ok(OrderType::StopWithProtection),
            "TRAIL" => Ok(OrderType::TrailingStop),
            "TRAIL LIMIT" => Ok(OrderType::TrailingStopLimit),
            "REL + LMT" => Ok(OrderType::RelativeLimit),
            "REL + MKT" => Ok(OrderType::RelativeMarket),
            "VOL" => Ok(OrderType::Volatility),
            "PEG BENCH" => Ok(OrderType::PeggedToBenchmark),
            &_ => Err(ParseEnumError)
        }
    }
}

impl Decodable for OrderType {}

impl Default for OrderType {
    fn default() -> Self {
        OrderType::Market
    }
}

#[derive(Debug,Clone)]
pub enum TriggerMethod {
    Default,
    DoubleBidAsk,
    Last,
    DoubleLast,
    BidAsk,
    LastOrBidAsk,
    MidPoint,
}

impl Encodable for TriggerMethod {
    fn encode(&self) -> String {
        match self {
            TriggerMethod::Default => "0\0",
            TriggerMethod::DoubleBidAsk => "1\0",
            TriggerMethod::Last => "2\0",
            TriggerMethod::DoubleLast => "3\0",
            TriggerMethod::BidAsk => "4\0",
            TriggerMethod::LastOrBidAsk => "7\0",
            TriggerMethod::MidPoint => "8\0"
        }.to_string()
    }
}

impl FromStr for TriggerMethod {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(TriggerMethod::Default),
            "1" => Ok(TriggerMethod::DoubleBidAsk),
            "2" => Ok(TriggerMethod::Last),
            "3" => Ok(TriggerMethod::DoubleLast),
            "4" => Ok(TriggerMethod::BidAsk),
            "7" => Ok(TriggerMethod::LastOrBidAsk),
            "8" => Ok(TriggerMethod::MidPoint),
            &_ => Err(ParseEnumError)
        }
    }
}

impl Decodable for TriggerMethod {}

#[derive(Debug,Clone)]
pub enum TimeInForce {
    Day,
    GoodTillCancel,
    ImmediateOrCancel,
    GoodUntilDate,
    GoodOnOpen,
    FillOrKill,
    DayUntilCancel,
}

impl Encodable for TimeInForce {
    fn encode(&self) -> String {
        match self {
            TimeInForce::Day => "DAY\0",
            TimeInForce::GoodTillCancel => "GTC\0",
            TimeInForce::ImmediateOrCancel => "IOC\0",
            TimeInForce::GoodUntilDate => "GTD\0",
            TimeInForce::GoodOnOpen => "OPG\0",
            TimeInForce::FillOrKill => "FOK\0",
            TimeInForce::DayUntilCancel => "DTC\0",
        }.to_string()
    }
}

impl FromStr for TimeInForce {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DAY" => Ok(TimeInForce::Day),
            "GTC" => Ok(TimeInForce::GoodTillCancel),
            "IOC" => Ok(TimeInForce::ImmediateOrCancel),
            "GTD" => Ok(TimeInForce::GoodTillCancel),
            "OPG" => Ok(TimeInForce::GoodOnOpen),
            "FOK" => Ok(TimeInForce::FillOrKill),
            "DTC" => Ok(TimeInForce::DayUntilCancel),
            &_ => Err(ParseEnumError)
        }
    }
}

impl Decodable for TimeInForce {}

#[derive(Debug,Clone)]
pub enum Rule80A {
    Individual,
    Agency,
    AgentOtherMember,
    IndividualPTIA,
    AgencyPTIA,
    AgentOtherMemberPTIA,
    IndividualPT,
    AgencyPT,
    AgentOtherMemberPT,
}

impl Encodable for Rule80A {
    fn encode(&self) -> String {
        match *self {
            Rule80A::Individual => "I\0",
            Rule80A::Agency => "A\0",
            Rule80A::AgentOtherMember => "W\0",
            Rule80A::IndividualPTIA => "J\0",
            Rule80A::AgencyPTIA => "U\0",
            Rule80A::AgentOtherMemberPTIA => "M\0",
            Rule80A::IndividualPT => "K\0",
            Rule80A::AgencyPT => "Y\0",
            Rule80A::AgentOtherMemberPT => "N\0",
        }.to_string()
    }
}

impl FromStr for Rule80A {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "I" => Ok(Rule80A::Individual),
            "A" => Ok(Rule80A::Agency),
            "W" => Ok(Rule80A::AgentOtherMember),
            "J" => Ok(Rule80A::IndividualPTIA),
            "U" => Ok(Rule80A::AgencyPTIA),
            "M" => Ok(Rule80A::AgentOtherMemberPTIA),
            "K" => Ok(Rule80A::IndividualPT),
            "Y" => Ok(Rule80A::AgencyPT),
            "N" => Ok(Rule80A::AgentOtherMemberPT),
            &_ => Err(ParseEnumError)
        }
    }
}

impl Decodable for Rule80A {}

#[derive(Debug,Clone)]
pub enum OrderOpenClose {
    Open,
    Close,
}

impl Encodable for OrderOpenClose {
    fn encode(&self) -> String {
        match self {
            OrderOpenClose::Open => "O\0",
            OrderOpenClose::Close => "C\0",
        }.to_string()
    }
}

impl FromStr for OrderOpenClose {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "O" => Ok(OrderOpenClose::Open),
            "C" => Ok(OrderOpenClose::Close),
            &_ => Err(ParseEnumError)
        }
    }
}

impl Decodable for OrderOpenClose {}

#[derive(Debug,Clone)]
pub enum Origin {
    Customer,
    Firm,
    Unknown
}

impl Encodable for Origin {
    fn encode(&self) -> String {
        match self {
            Origin::Customer => "0\0",
            Origin::Firm => "1\0",
            Origin::Unknown => "2\0"
        }.to_string()
    }
}

impl FromStr for Origin {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Origin::Customer),
            "1" => Ok(Origin::Firm),
            "2" => Ok(Origin::Unknown),
            &_ => Err(ParseEnumError)
        }
    }
}

impl Decodable for Origin {}

#[derive(FromPrimitive,Debug,Clone)]
pub enum AuctionStrategy {
    NoAuctionStrategy,
    Match,
    Improvement,
    Transparent
}

impl Encodable for AuctionStrategy {
    fn encode(&self) -> String {
        match self {
            AuctionStrategy::NoAuctionStrategy => "0\0",
            AuctionStrategy::Match => "1\0",
            AuctionStrategy::Improvement => "2\0",
            AuctionStrategy::Transparent => "3\0",
        }.to_string()
    }
}

impl FromStr for AuctionStrategy {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ord = match s.parse::<i32>() {
            Ok(n) => n,
            Err(_) => return Err(ParseEnumError),
        };
        match FromPrimitive::from_i32(ord) {
            Some(en_type) => return Ok(en_type),
            None => return Err(ParseEnumError),
        };
    }
}

impl Decodable for AuctionStrategy {}

#[derive(FromPrimitive,Debug,Clone)]
pub enum OCAType {
    NoOCAType,
    CancelWithBlock,
    ReduceWithBlock,
    ReduceNonBlock
}

impl Encodable for OCAType {
    fn encode(&self) -> String {
        match self {
            OCAType::NoOCAType => "0\0",
            OCAType::CancelWithBlock => "1\0",
            OCAType::ReduceWithBlock => "2\0",
            OCAType::ReduceNonBlock => "3\0",
        }.to_string()
    }
}

impl FromStr for OCAType {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ord = match s.parse::<i32>() {
            Ok(n) => n,
            Err(_) => return Err(ParseEnumError),
        };
        match FromPrimitive::from_i32(ord) {
            Some(en_type) =>  Ok(en_type),
            None => Err(ParseEnumError),
        }
    }
}

impl Decodable for OCAType {}

#[derive(Debug,Clone)]
pub enum VolatilityType {
    NoVolType,
    Daily,
    Annual
}

impl Encodable for VolatilityType {
    fn encode(&self) -> String {
        match self {
            VolatilityType::NoVolType => "0\0",
            VolatilityType::Daily => "1\0",
            VolatilityType::Annual => "2\0",
        }.to_string()
    }
}

impl FromStr for VolatilityType {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(VolatilityType::NoVolType),
            "1" => Ok(VolatilityType::Daily),
            "2" => Ok(VolatilityType::Annual),
            &_ => Err(ParseEnumError)
        }
    }
}

impl Decodable for VolatilityType {}

#[derive(Debug,Clone)]
pub enum ReferencePriceType {
    NoRefPriceType,
    Average,
    BidOrAsk
}

impl Encodable for ReferencePriceType {
    fn encode(&self) -> String {
        match self {
            ReferencePriceType::NoRefPriceType => "0\0",
            ReferencePriceType::Average => "1\0",
            ReferencePriceType::BidOrAsk => "2\0"
        }.to_string()
    }
}

impl FromStr for ReferencePriceType {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(ReferencePriceType::NoRefPriceType),
            "1" => Ok(ReferencePriceType::Average),
            "2" => Ok(ReferencePriceType::BidOrAsk),
            &_ => Err(ParseEnumError)
        }
    }
}

impl Decodable for ReferencePriceType {}

#[derive(Debug,Clone)]
pub enum BasisPointsType {
    Undefined,
}

impl Encodable for BasisPointsType {
    fn encode(&self) -> String {
        match self {
            BasisPointsType::Undefined => "?\0",
        }.to_string()
    }
}

impl FromStr for BasisPointsType {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "?" => Ok(BasisPointsType::Undefined),
            &_ => Err(ParseEnumError)
        }
    }
}

impl Decodable for BasisPointsType {}

#[derive(PartialEq,Debug,Clone)]
pub enum HedgeType {
    Undefined,
    Delta,
    Beta,
    Forex,
    Pair,
}

impl Encodable for HedgeType {
    fn encode(&self) -> String {
        match self {
            HedgeType::Undefined => "?\0",
            HedgeType::Delta => "D\0",
            HedgeType::Beta => "B\0",
            HedgeType::Forex => "F\0",
            HedgeType::Pair => "P\0",
        }.to_string()
    }
}

impl FromStr for HedgeType {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "?" => Ok(HedgeType::Undefined),
            "D" => Ok(HedgeType::Delta),
            "B" => Ok(HedgeType::Beta),
            "F" => Ok(HedgeType::Forex),
            "P" => Ok(HedgeType::Pair),
            &_ => Err(ParseEnumError)
        }
    }
}

impl Decodable for HedgeType {}

#[derive(Debug,Clone)]
pub enum ClearingIntent {
    InteractiveBrokers,
    Away,
    PTA,
}

impl Encodable for ClearingIntent {
    fn encode(&self) -> String {
        match self {
            ClearingIntent::InteractiveBrokers => "IB\0",
            ClearingIntent::Away => "Away\0",
            ClearingIntent::PTA => "PTA\0",
        }.to_string()
    }
}

impl FromStr for ClearingIntent {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "IB" => Ok(ClearingIntent::InteractiveBrokers),
            "Away" => Ok(ClearingIntent::Away),
            "PTA" => Ok(ClearingIntent::PTA),
            &_ => Err(ParseEnumError)
        }
    }
}

impl Decodable for ClearingIntent {}

#[derive(Debug,Clone)]
pub enum UsePriceMgmtAlgo {
    DontUse,
    Use
}

impl Encodable for UsePriceMgmtAlgo {
    fn encode(&self) -> String {
        match self {
            UsePriceMgmtAlgo::DontUse => "0\0",
            UsePriceMgmtAlgo::Use => "1\0",
        }.to_string()
    }
}

impl FromStr for UsePriceMgmtAlgo {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(UsePriceMgmtAlgo::DontUse),
            "1" => Ok(UsePriceMgmtAlgo::Use),
            &_ => Err(ParseEnumError)
        }
    }
}

impl Decodable for UsePriceMgmtAlgo {}

#[derive(Debug,Clone)]
pub enum Side {
    Long,
    Short,
}

impl Encodable for Side {
    fn encode(&self) -> String {
        match self {
            Side::Long => "BOT\0",
            Side::Short => "SLD\0",
        }.to_string()
    }
}

impl FromStr for Side {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BOT" => Ok(Side::Long),
            "SLD" => Ok(Side::Short),
            &_ => Err(ParseEnumError)
        }
    }
}

impl Decodable for Side {}

#[derive(FromPrimitive,Debug,Clone)]
#[derive(enum_ordinalize::Ordinalize)]
pub enum OrderConditionType {
    Price = 1,
    Time = 3,
    Margin = 4,
    Execution = 5,
    Volume = 6,
    PercentChange = 7
}

impl Encodable for OrderConditionType {
    fn encode(&self) -> String {
        let ord = self.ordinal();
        ord.to_string() + "\0"
    }
}

impl FromStr for OrderConditionType {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ord = match s.parse::<i32>() {
            Ok(n) => n,
            Err(_) => return Err(ParseEnumError),
        };
        match FromPrimitive::from_i32(ord) {
            Some(en_type) => return Ok(en_type),
            None => return Err(ParseEnumError),
        };
    }
}

impl Decodable for OrderConditionType {}

pub enum IBAccountField {
    AccountType,
    NetLiquidation,
    TotalCashValue,
    SettledCash,
    AccruedCash,
    BuyingPower,
    EquityWithLoanValue,
    PreviousEquityWithLoanValue,
    GrossPositionValue,
    ReqTEquity,
    ReqTMargin,
    SMA,
    InitMarginReq,
    MaintMarginReq,
    AvailableFunds,
    ExcessLiquidity,
    Cushion,
    FullInitMarginReq,
    FullMaintMarginReq,
    FullAvailableFunds,
    FullExcessLiquidity,
    LookAheadNextChange,
    LookAheadInitMarginReq,
    LookAheadMaintMarginReq,
    LookAheadAvailableFunds,
    LookAheadExcessLiquidity,
    HighestSeverity,
    DayTradesRemaining,
    Leverage,
}

impl FromStr for IBAccountField {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AccountType" => Ok(IBAccountField::AccountType),
            "NetLiquidation" => Ok(IBAccountField::NetLiquidation),
            "TotalCashValue" => Ok(IBAccountField::TotalCashValue),
            "SettledCash" => Ok(IBAccountField::SettledCash),
            "AccruedCash" => Ok(IBAccountField::AccruedCash),
            "BuyingPower" => Ok(IBAccountField::BuyingPower),
            "EquityWithLoanValue" => Ok(IBAccountField::EquityWithLoanValue),
            "PreviousEquityWithLoanValue" => Ok(IBAccountField::PreviousEquityWithLoanValue),
            "GrossPositionValue" => Ok(IBAccountField::GrossPositionValue),
            "ReqTEquity" => Ok(IBAccountField::ReqTEquity),
            "ReqTMargin" => Ok(IBAccountField::ReqTMargin),
            "SMA" => Ok(IBAccountField::SMA),
            "InitMarginReq" => Ok(IBAccountField::InitMarginReq),
            "MaintMarginReq" => Ok(IBAccountField::MaintMarginReq),
            "AvailableFunds" => Ok(IBAccountField::AvailableFunds),
            "ExcessLiquidity" => Ok(IBAccountField::ExcessLiquidity),
            "Cushion" => Ok(IBAccountField::Cushion),
            "FullInitMarginReq" => Ok(IBAccountField::FullInitMarginReq),
            "FullMaintMarginReq" => Ok(IBAccountField::FullMaintMarginReq),
            "FullAvailableFunds" => Ok(IBAccountField::FullAvailableFunds),
            "FullExcessLiquidity" => Ok(IBAccountField::FullExcessLiquidity),
            "LookAheadNextChange" => Ok(IBAccountField::LookAheadNextChange),
            "LookAheadInitMarginReq" => Ok(IBAccountField::LookAheadInitMarginReq),
            "LookAheadMaintMarginReq" => Ok(IBAccountField::LookAheadMaintMarginReq),
            "LookAheadAvailableFunds" => Ok(IBAccountField::LookAheadAvailableFunds),
            "LookAheadExcessLiquidity" => Ok(IBAccountField::LookAheadExcessLiquidity),
            "HighestSeverity" => Ok(IBAccountField::HighestSeverity),
            "DayTradesRemaining" => Ok(IBAccountField::DayTradesRemaining),
            "Leverage" => Ok(IBAccountField::Leverage),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for IBAccountField {}
