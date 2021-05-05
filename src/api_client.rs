const API_URL: &'static str = "https://api.coingecko.com/api/v3";

use std::collections::HashMap;

use crate::data;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Client {}

impl Client {
    pub fn new() -> Self {
        Self {}
    }

    //ping

    pub async fn ping(&self) -> Result<String, reqwest::Error> {
        reqwest::get(format!("{}/ping", API_URL))
            .await?
            .text()
            .await
    }

    //simple

    pub async fn price(&self, ids: &[&str], vs_currencies: &[&str]) -> Result<HashMap<String, HashMap<String, f64>>, reqwest::Error> {
        let ids = ids.join(",");
        let vs_currencies = vs_currencies.join(",");
        reqwest::get(format!("{}/simple/price?ids={}&vs_currencies={}", API_URL, ids, vs_currencies))
            .await?
            .json()
            .await
    }

    pub async fn vs_currencies(&self) -> Result<Vec<data::RawVsCurrency>, reqwest::Error> {
        reqwest::get(format!("{}/simple/supported_vs_currencies", API_URL))
            .await?
            .json::<Vec<String>>()
            .await
            .map(|vec| vec
                .into_iter()
                .map(|name| data::RawVsCurrency {
                    name
                })
                .collect())
    }

    //coins

    pub async fn coins(&self) -> Result<Vec<data::RawCoin>, reqwest::Error> {
        reqwest::get(format!("{}/coins/list", API_URL))
            .await?
            .json()
            .await
    }

    //WARNING: a big pitfall is that input data is in SECONDS but the output data is in MILLISECONDS
    pub async fn market_chart(&self, id: &str, currency: &str, from: u64, to: u64) -> Result<data::RawMarketChart, reqwest::Error> {
        reqwest::get(format!("{}/coins/{}/market_chart/range?vs_currency={}&from={}&to={}", API_URL, id, currency, from, to))
            .await?
            .json()
            .await
    }
}