# rust_ib_api

This is a native Rust client for the Interactive Brokers TWS API (currently IB Gateway 10.19). It does not depend on any of the official API wrappers provided by IB.

The client is multithreaded and uses the tokio runtime. Requests are either blocking (REST like) or streaming, depending on what makes more sense. Upon connection, the client will automatically subscribe to account updates.

# Usage

Here is how you would request contract details for a specific contract:

```rust
let mut client = match IBClient::connect(4002, 1, "").await {
        Ok(client) => client,
        Err(_error) => panic!("Connection not successful!")
    };
    let contract = Contract {
        symbol: Some("AAPL".to_string()),
        exchange: Some("SMART".to_string()),
        sec_type: Some(SecType::Stock),
        currency: Some("USD".to_string()),
        ..Default::default()
    }; 
    match client.req_contract_details(&contract).await {
        Ok(details) => for detail in &details {
            match &detail.contract {
                Some(contract) => assert_eq!(contract.symbol, Some("AAPL".to_string())),
                None => panic!("No valid contract details returned for AAPL")
            }
        }
        Err(_) => panic!("Error requesting contract details")
    };
```
This is an example of blocking request that will return when the response is delivered. All the complications of having to deal with asynchronous socket communication are taken care of by the client.

To place an order:
```rust
let mut client = match IBClient::connect(4002, 2, "").await {
        Ok(client) => client,
        Err(_error) => panic!("Connection not successful!")
    };
    let contract = Contract {
        symbol: Some("AAPL".to_string()),
        exchange: Some("SMART".to_string()),
        sec_type: Some(SecType::Stock),
        currency: Some("USD".to_string()),
        ..Default::default()
    };
    let order = Order::market(contract, Action::Buy, Decimal::new(10,0));
    match &mut client.place_order(&order).await {
        Ok(tracker) => {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            assert_eq!(tracker.status(), Some("Filled".to_string()));
        }
        Err(err)    => panic!("Error during order submission.")
    }
```
Placing an order returns a tracker object which will be continuously updated with information on the orders status by the client.

The client internally launches three tasks, one to manage the read half of the socket connection, one to manage the write half, and a keep-alive task which periodically sends a request to the server. All communications and synchronization is done via channels.

For more usage examples, see the integration tests.

# Error handling

The client is currently refactored to avoid crashes. The `IBClient::connect` function will return an error if the initial connection is unsuccessful. After that, socket disconnects will be communicated from the reader/writer task to the client object and these tasks will then shut down. Any further request to the client will return an error. The client is not designed for reconnection. To establish a new connection, build a new client object. All detached tasks will be canceled when the client is deallocated. Errors on decoding messages from the server will be converted to Option:None for now, the client keeps running. A logger task is still to be added, as well as the communication of actual API-Errors send from the server.

#Limitations/Disclaimer

The code is largely ported from the official API projects, the message parsing should therefore be complete. This does not mean, however, that all potential functionality of the API has been tested. Managed accounts, trading Bonds/Warrants etc. have not and will not be tested by me, since I lack the corresponding trading permissions.

The code is currently actively worked on and may not always be in a functioning state.