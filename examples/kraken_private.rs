use calamari::{
    ApiCredentials, KrakenPrivateApiClient, KrakenPrivateEndpoints,
    KrakenPublicEndpoints,
};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("KRAKEN_API_KEY").unwrap();
    let api_secret = std::env::var("KRAKEN_API_SECRET").unwrap();
    let credentials = ApiCredentials::new(api_key, api_secret);
    let client = KrakenPrivateApiClient::default_with_credentials(credentials);
    // Alternatively, if `client` is already a `PublicApiClient`:
    // let client = client.set_credentials(credentials);

    println!("Server time: {}", client.time().await.unwrap());
    println!("System status: {}", client.system_status().await.unwrap());
    println!(
        "Ticker: {}",
        client.ticker("pair=XBTUSD".into()).await.unwrap()
    );

    println!("Account balance: {}", client.balance().await.unwrap());
    println!(
        "Open orders: {}",
        client.open_orders("trades=true".into()).await.unwrap()
    );
}
