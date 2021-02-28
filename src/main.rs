mod ib_enums;
mod utils;
mod ib_client;
mod account;
mod frame;
mod ib_contract;
mod order;
mod ticker;
mod bars;
use ib_client::IBClient;
use tokio::time;
use ib_contract::Contract;
use ib_enums::*;
use rust_decimal::prelude::*;

#[tokio::main]
async fn main() {
    // Connect to a peer
    {
        let mut client = match IBClient::connect(4002, 1, "").await {
            Ok(client) => client,
            Err(error) => panic!("Connection not successful!")
        };
        let contract = Contract {
            symbol: Some("AAPL".to_string()),
            exchange: Some("SMART".to_string()),
            sec_type: Some(SecType::Stock),
            currency: Some("USD".to_string()),
            ..Default::default()
        };
        match client.req_contract_details(&contract).await {
            Ok(details) => for detail in details {
                println!("{:?}", detail);
            }
            Err(_) => panic!("Error requesting contract details")
        };
        let order = order::BasicOrder::market(Action::Buy, &Decimal::new(10,0));
        let mut tracker = client.place_basic_order(&contract, &order).await;
        
        match &mut tracker {
            Ok(tracker) => println!("{:?}", tracker.status()),
            Err(err) => println!("{:?}", err)
        }
        
        println!("Net liquidation value is {:?}", client.net_liquidation_value());
    }
    time::sleep(time::Duration::from_secs(10)).await;
}
