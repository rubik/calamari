<div align="center">
  <img alt="calamari logo" src="https://github.com/rubik/calamari/raw/master/images/logo.png" height="130" />
</div>

<div align="center">
  <h1>Calamari</h1>
  <p>Minimal and elegant async REST API client for Kraken</p>
  <a target="_blank" href="https://coveralls.io/github/rubik/calamari">
    <img src="https://img.shields.io/coveralls/github/rubik/calamari?style=for-the-badge" alt="Code Coverage">
  </a>
  <a target="_blank" href="https://crates.io/crates/calamari">
   <img src="https://img.shields.io/crates/d/calamari?style=for-the-badge" alt="Downloads (all time)">
  <a>
  <a href="https://github.com/rubik/calamari/blob/master/LICENSE">
    <img src="https://img.shields.io/crates/l/calamari?style=for-the-badge" alt="ISC License">
  </a>
  <br>
  <br>
</div>

Calamari is a REST API client for [Kraken](https://kraken.com).

# Quickstart
The API client comes in two flavors: `PublicApiClient` and `PrivateApiClient`.
The former has access to the public methods only, the latter to all endpoints.
This is enforced at compile-time, as all the endpoints are defined statically
in the traits `PublicEndpoints` and `PrivateEndpoints`.

```rust
use calamari::{PublicApiClient, PublicEndpoints};

// Note: to run this example you will need to add Tokio to your dependencies:
// tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }

#[tokio::main]
async fn main() {
    let client = PublicApiClient::default();
    println!("Server time: {}", client.time().await.unwrap());
    println!("System status: {}", client.system_status().await.unwrap());
    println!("Ticker: {}", client.ticker("pair=XBTUSD".into()).await.unwrap());
}
```

Each endpoint accepts either zero arguments or a single argument containing all
the request parameters in urlencode format. All endpoints return a `String`
containing the JSON response from the server, leaving the user with complete
freedom in how they want to handle it.

A `PrivateApiClient` can be instantiated directly, or created from an existing
`PublicApiClient` by supplying the API credentials with the `set_credentials`
method.

```rust
use calamari::{ApiCredentials, PrivateApiClient, PublicEndpoints, PrivateEndpoints};

#[tokio::main]
async fn main() {
    let credentials = ApiCredentials::new(
        "YOUR_API_KEY".into(),
        "YOUR_API_SECRET".into(),
    );
    let client = PrivateApiClient::default_with_credentials(credentials);
    // Alternatively, if `client` is already a `PublicApiClient`:
    // let client = client.set_credentials(credentials);

    println!("Server time: {}", client.time().await.unwrap());
    println!("System status: {}", client.system_status().await.unwrap());
    println!("Ticker: {}", client.ticker("pair=XBTUSD".into()).await.unwrap());

    println!("Account balance: {}", client.balance().await.unwrap());
    println!("Open orders: {}", client.open_orders("trades=true".into()).await.unwrap());
}
```

# Documentation
The complete documentation is available on
[docs.rs](https://docs.rs/calamari/latest/calamari/).

<div>
  <small>
    Logo made by <a href="https://www.flaticon.com/authors/freepik" title="Freepik">Freepik</a> from <a href="https://www.flaticon.com" title="Flaticon">www.flaticon.com</a>
  </small>
</div>
