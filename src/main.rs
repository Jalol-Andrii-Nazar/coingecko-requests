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

    async fn simple_supported_vs_currencies(&self) -> Result<Vec<String>, reqwest::Error> {
        reqwest::get(format!("{}/simple/supported_vs_currencies", API_URL))
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
    let coins_list = client.coins_list().await?;
    for coin in coins_list {
        println!("{:?}", coin);
    }
    let currencies = client.simple_supported_vs_currencies().await?;
    for currency in currencies {
        println!("{}",currency);
    }
    Ok(())
}