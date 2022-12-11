//! A Rust client for the Interactive Brokers TWS API. Currently tested against TWS Gateway Version 10.19.
//! The client wraps the asynchronous socket communication with the TWS/Gateway and exposes an easy to use API.
//! In order to connect to the Gateway/TWS, create a client object:
//! ```
//!use rs_ib_api::ib_client::IBClient;
//!let mut client = match IBClient::connect(4002, 1, "").await?;
//! ```
//! If the connection is successful, you receive a client object that is already subscribed to streaming account updates
//! and periodically polls the Gatway to maintain an open connection. If the initial connection fails, an error is returned.
//! Make sure that the port and client ID are correct and that the socket API is correctly configured in the TWS/Gatway.
//! Next, you might request contract details for Apple (APPL) stock as follows:
//! ```
//!use rs_ib_api::ib_contract::*;
//!let contract = Contract {
//!                 symbol: Some("AAPL".to_string()),
//!                 exchange: Some("SMART".to_string()),
//!                 sec_type: Some(SecType::Stock),
//!                 currency: Some("USD".to_string()),
//!                 ..Default::default()
//!                 }; 
//!let details = client.req_contract_details(&contract).await?; 
//!for detail in &details {
//!     match &detail.contract {
//!         Some(contract) => assert_eq!(contract.symbol, Some("AAPL".to_string())),
//!         None => _
//!};
//! ```
//! The client provides a variety of such requests that will block if awaited until the response is received.
//! Requests for real-time market data and placing orders are special in the sense that they return structs which
//! provide streaming updates.
//! When placing an order, for example, the returned `OrderTracker` can be used to monitor the status of the order:
pub mod ib_enums;
mod utils;
pub mod ib_client;
mod account;
mod frame;
pub mod ib_contract;
pub mod order;
pub mod ticker;
pub mod bars;