use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Coin {
    id: String,
    symbol: String,
    name: String,
}

#[derive(Debug)]
pub struct CoinRange {
    price: Vec<(u64, f64)>,
    market_caps: Vec<(u64, u64)>,
    total_volumes: Vec<(u64, u64)>,
}

impl <'de> Deserialize<'de> for CoinRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        todo!()
    }
}
