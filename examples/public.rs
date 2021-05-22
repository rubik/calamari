use squid::{PublicApiClient, PublicEndpoints};

#[tokio::main]
async fn main() {
    let client = PublicApiClient::default();
    println!("Server time: {}", client.time().await.unwrap());
    println!("System status: {}", client.system_status().await.unwrap());
    println!("Ticker: {}", client.ticker("pair=XBTUSD".into()).await.unwrap());
}
