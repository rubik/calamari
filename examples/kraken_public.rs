use calamari::{KrakenPublicApiClient, KrakenPublicEndpoints};

#[tokio::main]
async fn main() {
    let client = KrakenPublicApiClient::default();
    println!("Server time: {}", client.time().await.unwrap());
    println!("System status: {}", client.system_status().await.unwrap());
    println!(
        "Ticker: {}",
        client.ticker("pair=XBTUSD".into()).await.unwrap()
    );
}
