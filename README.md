<div align="center">
  <img alt="squid logo" src="https://github.com/rubik/squid/raw/master/images/logo.png" height="130" />
</div>

<div align="center">
  <h1>squid</h1>
  <p>Minimal and elegant async REST API client for Kraken</p>
  <a target="_blank" href="https://travis-ci.org/rubik/squid">
    <img src="https://img.shields.io/travis/rubik/squid?style=for-the-badge" alt="Build">
  </a>
  <a target="_blank" href="https://coveralls.io/github/rubik/squid">
    <img src="https://img.shields.io/coveralls/github/rubik/squid?style=for-the-badge" alt="Code Coverage">
  </a>
  <a target="_blank" href="https://crates.io/crates/squid">
   <img src="https://img.shields.io/crates/d/squid?style=for-the-badge" alt="Downloads (all time)">
  <a>
  <a href="https://github.com/rubik/squid/blob/master/LICENSE">
    <img src="https://img.shields.io/crates/l/squid?style=for-the-badge" alt="ISC License">
  </a>
  <br>
  <br>
</div>

squid is an API client for [Kraken](https://kraken.com).

# Quickstart
The API client comes in two flavors: `PublicApiClient` and `PrivateApiClient`.
The former has access to the public methods only, the latter to all endpoints.
This is enforced at compile-time, as all the endpoints are defined statically
in the traits `PublicEndpoints` and `PrivateEndpoints`.

```rust
use squid::{PublicApiClient, PublicEndpoints};

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

<div>
  <small>
    Logo made by <a href="https://www.flaticon.com/authors/freepik" title="Freepik">Freepik</a> from <a href="https://www.flaticon.com" title="Flaticon">www.flaticon.com</a>
  </small>
</div>
