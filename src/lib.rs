//! Calamari is a REST API client for [Kraken](https://kraken.com).
//!
//! # Quickstart
//! The API client comes in two flavors: `PublicApiClient` and `PrivateApiClient`.
//! The former has access to the public methods only, the latter to all endpoints.
//! This is enforced at compile-time, as all the endpoints are defined statically
//! in the traits `PublicEndpoints` and `PrivateEndpoints`.
//!
//! ```rust
//! use calamari::{PublicApiClient, PublicEndpoints};
//!
//! // Note: to run this example you will need to add Tokio to your dependencies:
//! // tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = PublicApiClient::default();
//!     println!("Server time: {}", client.time().await.unwrap());
//!     println!("System status: {}", client.system_status().await.unwrap());
//!     println!("Ticker: {}", client.ticker("pair=XBTUSD".into()).await.unwrap());
//! }
//! ```
//!
//! Each endpoint accepts either zero arguments or a single argument containing all
//! the request parameters in urlencode format. All endpoints return a `String`
//! containing the JSON response from the server, leaving the user with complete
//! freedom in how they want to handle it.
//!
//! A `PrivateApiClient` can be instantiated directly, or created from an existing
//! `PublicApiClient` by supplying the API credentials with the `set_credentials`
//! method.
//!
//! ```rust
//! use calamari::{ApiCredentials, PrivateApiClient, PublicEndpoints, PrivateEndpoints};
//!
//! #[tokio::main]
//! async fn main() {
//!     let credentials = ApiCredentials::new(
//!         "YOUR_API_KEY".into(),
//!         "YOUR_API_SECRET".into(),
//!     );
//!     # let credentials = ApiCredentials::new(
//!     #     "YOUR_API_KEY".into(),
//!     #     "c2VjcmV0".into(),
//!     # );
//!     let client = PrivateApiClient::default_with_credentials(credentials);
//!     // Alternatively, if `client` is already a `PublicApiClient`:
//!     // let client = client.set_credentials(credentials);
//!
//!     println!("Server time: {}", client.time().await.unwrap());
//!     println!("System status: {}", client.system_status().await.unwrap());
//!     println!("Ticker: {}", client.ticker("pair=XBTUSD".into()).await.unwrap());
//!
//!     println!("Account balance: {}", client.balance().await.unwrap());
//!     println!("Open orders: {}", client.open_orders("trades=true".into()).await.unwrap());
//! }
//! ```
//!
//! All endpoints can be found under the [`PublicEndpoints`] and
//! [`PrivateEndpoints`] traits, which contain all the endpoints listed in
//! [the Kraken documentation](https://docs.kraken.com/rest/).

mod client;
mod endpoints;
mod models;
mod request;

pub use client::{PrivateApiClient, PublicApiClient};
pub use endpoints::{PrivateEndpoints, PublicEndpoints};
pub use models::{ApiCredentials, ApiParams};
