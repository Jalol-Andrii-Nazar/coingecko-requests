use serde::{Deserialize};

const API_URL: &'static str = "https://api.coingecko.com/api/v3";

struct Client {}

#[derive(Debug, Deserialize)]
struct Coin {
    id: String,
    symbol: String,
    name: String,
}

impl Client {
    fn new() -> Self {
        Self {}
    }

    async fn ping(&self) -> Result<String, reqwest::Error> {
        reqwest::get(format!("{}/ping", API_URL))
            .await?
            .text()
            .await
    }

    async fn coins_list(&self) -> Result<Vec<Coin>, reqwest::Error> {
        reqwest::get(format!("{}/coins/list", API_URL))
            .await?
            .json()
            .await
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client.ping().await?;
    println!("Ping result: {}", res);
    Ok(())
}