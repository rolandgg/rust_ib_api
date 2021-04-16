# rust_ib_api

This is a native Rust client for the Interactive Brokers TWS API (IB Gateway 978+). It does not depend on any of the official API wrappers provided by IB.

The client is multithreaded and uses the tokio runtime. Requests are either blocking (REST like) or streaming, depending on what makes more sense. Upon connection, the client will automatically subscribe to account updates.

For usage examples, see the integration tests.