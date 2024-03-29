use rs_ib_api::client::IBClient;
use rs_ib_api::contract::*;
use rs_ib_api::order::Order;
use tokio::time;
use chrono::Duration;
use chrono::{TimeZone, Utc, DateTime};
use rs_ib_api::enums::*;
use rust_decimal::prelude::*;

#[tokio::test]
async fn connection() {
    let mut client = match IBClient::connect(4002, 1, "", None).await {
        Ok(client) => client,
        Err(_error) => panic!("Connection not successful!")
    };
    time::sleep(time::Duration::from_secs(10)).await;
}

#[tokio::test]
async fn contract_details() {
    let mut client = match IBClient::connect(4002, 1, "", Some("foo.log")).await {
        Ok(client) => client,
        Err(_error) => panic!("Connection not successful!")
    };
    let contract = Contract::stock("SPY", "ARCA", "USD");
    match client.req_contract_details(&contract).await {
        Ok(details) => for detail in &details {
            match &detail.contract() {
                Some(contract) => assert_eq!(contract.symbol(), &Some("SPY".to_string())),
                None => panic!("No valid contract details returned for SPY")
            }
        }
        Err(err) => panic!("Error requesting contract details : {:?}", err)
    };
}

#[tokio::test]
async fn opt_params() {
    let mut client = match IBClient::connect(4002, 1, "", Some("foo.log")).await {
        Ok(client) => client,
        Err(_error) => panic!("Connection not successful!")
    };
    let contract = Contract::stock("SPY", "ARCA", "USD");
    match client.req_contract_details(&contract).await {
        Ok(details) => for detail in &details {
            match &detail.contract() {
                Some(contract) => {
                    match client.req_options_metadata(&contract, None).await {
                        Ok(params) => (),
                        _ => ()
                    }
                },
                None => panic!("No valid contract details returned for SPY")
            }
        }
        Err(err) => panic!("Error requesting contract details : {:?}", err)
    };
}

#[tokio::test]
async fn liquid_hours() {
    let mut client = match IBClient::connect(4002, 1, "", None).await {
        Ok(client) => client,
        Err(_error) => panic!("Connection not successful!")
    };
    let contract = Contract::stock("AAPL", "SMART", "USD");
    match client.req_contract_details(&contract).await {
        Ok(details) => for detail in &details {
            assert!(detail.liquid_hours().is_some());
        }
        Err(_) => panic!("Error requesting contract details")
    };
}

#[tokio::test]
async fn place_market_order() {
    let mut client = match IBClient::connect(4002, 2, "", Some("boo.log")).await {
        Ok(client) => client,
        Err(_error) => panic!("Connection not successful!")
    };
    let contract = Contract::stock("AAPL", "SMART", "USD");
    let order = Order::market(contract, Action::Buy, Decimal::new(10,0));
    match &mut client.place_order(&order).await {
        Ok(tracker) => {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            assert_eq!(tracker.status(), Some("Filled".to_string()));
        }
        Err(err)    => panic!("Error during order submission.")
    }
}

#[tokio::test]
async fn place_spread_market_order() {
    let mut client = match IBClient::connect(4002, 1, "", None).await {
        Ok(client) => client,
        Err(_error) => panic!("Connection not successful!")
    };
    let mut contract = Contract::combo("IBKR,MCD", "SMART", "USD");
    contract.add_leg(ComboLeg::new(43645865, 1, ComboAction::Buy, "SMART"));
    contract.add_leg(ComboLeg::new(9408, 1, ComboAction::Sell, "SMART"));

    let mut order = Order::market(contract, Action::Buy, Decimal::new(10,0)).combo();
    match &mut client.place_order(&order).await {
        Ok(tracker) => {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            assert_eq!(tracker.status(), Some("Filled".to_string()));
        }
        Err(err)    => panic!("Error during order submission.")
    }
}

#[tokio::test]
async fn market_data() {
    let mut client = match IBClient::connect(4002, 3, "", None).await {
        Ok(client) => client,
        Err(_error) => panic!("Connection not successful!")
    };
    let contract = Contract::stock("AAPL", "SMART", "USD");
    match &client.req_market_data(&contract, false, false,
         Some(vec![GenericTickType::ShortableData])).await {
        Ok(ticker) => {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            assert!(ticker.midpoint().is_some());
        }
        Err(_error) => panic!("Market data request not successful")
    }
}

#[tokio::test]
async fn delayed_market_data() {
    let mut client = match IBClient::connect(4002, 4, "", None).await {
        Ok(client) => client,
        Err(_error) => panic!("Connection not successful!")
    };
    client.set_mkt_data_delayed().await;
    let contract = Contract::stock("AAPL", "SMART", "USD");
    match &client.req_market_data(&contract, false, false,
         Some(vec![GenericTickType::ShortableData])).await {
        Ok(ticker) => {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            assert!(ticker.midpoint().is_some());
            println!("{:?}", ticker.midpoint())
        }
        Err(_error) => panic!("Market data request not successful")
    }
}

#[tokio::test]
async fn snapshot_data() {
    let mut client = match IBClient::connect(4002, 3, "", None).await {
        Ok(client) => client,
        Err(_error) => panic!("Connection not successful!")
    };
    let contract = Contract::stock("AAPL", "SMART", "USD");
    match &client.req_market_data(&contract, true, false,
         None).await {
        Ok(ticker) => {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            assert!(ticker.midpoint().is_some());
            println!("{:?}", ticker.midpoint())
        }
        Err(_error) => panic!("Market data request not successful")
    }
}

#[tokio::test]
async fn historical_data() {
    let mut client = match IBClient::connect(4002, 4, "", None).await {
        Ok(client) => client,
        Err(_error) => panic!("Connection not successful!")
    };
    let contract = Contract::stock("AAPL", "SMART", "USD");
    let end_dt = Utc.datetime_from_str("2020-03-01 00:00:00", "%Y-%m-%d %H:%M:%S");

    match &client.req_historical_data(&contract, &end_dt.unwrap(), HistoricalDataDuration::Months(1), HistoricalDataBarSize::OneDay,
    HistoricalDataType::Midpoint, true).await {
        Ok(bars) => {
            assert!(bars.n_bars.is_some());
        },
        Err(_error) => panic!("Bar series loading not successful!")
    }
}