# rust_ib_api

This is a native Rust client for the Interactive Brokers TWS API (currently IB Gateway 10.19). It does not depend on any of the official API wrappers provided by IB.

The client is multithreaded and uses the tokio runtime. Requests are either blocking (REST like) or streaming, depending on what makes more sense. Upon connection, the client will automatically subscribe to account updates.

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

For usage examples, see the integration tests.