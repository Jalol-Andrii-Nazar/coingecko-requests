const API_URL: &'static str = "https://api.coingecko.com/api/v3";

use crate::data;

pub struct Client {}

impl Client {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn ping(&self) -> Result<String, reqwest::Error> {
        reqwest::get(format!("{}/ping", API_URL))
            .await?
            .text()
            .await
    }

    pub async fn coins_list(&self) -> Result<Vec<data::Coin>, reqwest::Error> {
        reqwest::get(format!("{}/coins/list", API_URL))
            .await?
            .json()
            .await
    }

    pub async fn simple_supported_vs_currencies(&self) -> Result<Vec<String>, reqwest::Error> {
        reqwest::get(format!("{}/simple/supported_vs_currencies", API_URL))
            .await?
            .json()
            .await
    }

    pub async fn coins_id_market_chart_range(&self, id: &str, currency: &str, from: u64, to: u64) -> Result<data::MarketChart, reqwest::Error> {
        reqwest::get(format!("{}/coins/{}/market_chart/range?vs_currency={}&from={}&to={}", API_URL, id, currency, from, to))
            .await?
            .json()
            .await
    }
}