use calamari::{DeribitPublicApiClient, DeribitPublicEndpoints};

#[tokio::main]
async fn main() {
    let client = DeribitPublicApiClient::default();
    println!("Server time: {}", client.time().await.unwrap());
    println!("System status: {}", client.system_status().await.unwrap());
    println!(
        "Ticker: {}",
        client.ticker("pair=XBTUSD".into()).await.unwrap()
    );
}
