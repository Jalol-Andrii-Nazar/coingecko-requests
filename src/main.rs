use serde::{Deserialize};
const API_URL: &'static str = "https://api.coingecko.com/api/v3";

struct Client {}

#[derive(Debug, Deserialize)]
struct Coin {
    id: String,
    symbol: String,
    name: String,
}

#[derive(Debug, Deserialize)]
struct CoinRange {
    price: Vec<(u64, f64)>,
    market_caps: Vec<(u64, u64)>,
    total_volumes: Vec<(u64, u64)>,
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

    async fn coins_id_market_chart_range(&self, id: &str, currency: &str, to_: u64, from_:u64) -> Result<CoinRange, reqwest::Error> {
        reqwest::get(format!("{}coins/{}/market_chart/range?vs_currency={}&from={}&to={}", API_URL, id, currency, from_, to_))
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

    //let cryptoRange = client.coins_id_market_chart_range("btc", "usd", 1392577232, 1392677232);


    Ok(())
}