//! A Rust client for the Interactive Brokers TWS API.
//! 
//! Currently tested against TWS Gateway Version 10.19.
//! The client wraps the asynchronous socket communication with the TWS/Gateway and exposes an easy to use API.
//! 
//! # How to use
//! In order to connect to the Gateway/TWS, create a `IBClient` object:
//!```
//!use rs_ib_api::ib_client::IBClient;
//! 
//!let mut client = match IBClient::connect(4002, 1, "").await?;
//!```
//! If the connection is successful, you receive a client object that is already subscribed to streaming account updates
//! and periodically polls the Gatway to maintain an open connection. If the initial connection fails, an error is returned.
//! Make sure that the port and client ID are correct and that the socket API is correctly configured in the TWS/Gateway.
//! Next, you might request contract details for Apple (AAPL) stock as follows:
//!```
//!use rs_ib_api::ib_contract::*;
//!use rs_ib_api::ib_enums::*;
//! 
//!let contract = Contract::stock_us_smart("AAPL");
//!let details = client.req_contract_details(&contract).await?; 
//!for detail in &details {
//!     match &detail.contract {
//!         Some(contract) => assert_eq!(contract.symbol, Some("AAPL".to_string())),
//!         None => _
//!     };
//!};
//!```
//! The client provides a variety of such requests that will block until the response is received.
//! Requests for real-time market data and placing orders are special in the sense that they return structs which
//! provide streaming updates.
//! When placing an order, for example, the returned `OrderTracker` can be used to monitor the status of the order:
//! ```
//! use rs_ib_api::order::Order;
//! //buy 10 shares of Apple
//! let order = Order::market(contract, Action::Buy, Decimal::new(10,0));
//! let mut tracker = client.place_order(&order).await? 
//! tokio::time::sleep(std::time::Duration::from_secs(1)).await;
//! assert_eq!(tracker.status(), Some("Filled".to_string()));
//! ```
pub mod enums;
mod utils;
pub mod client;
mod account;
mod frame;
pub mod contract;
pub mod order;
pub mod ticker;
pub mod bars;