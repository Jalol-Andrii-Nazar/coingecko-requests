use std::collections::HashMap;

use serde::{Deserialize, de::Visitor};

#[derive(Debug, Deserialize)]
pub struct Coin {
    id: String,
    symbol: String,
    name: String,
}

#[derive(Debug)]
pub struct CoinRange {
    prices: Vec<(u64, f64)>,
    market_caps: Vec<(u64, f64)>,
    total_volumes: Vec<(u64, f64)>,
}

struct CoinRangeVisitor;

impl <'de> Visitor<'de> for CoinRangeVisitor {
    type Value = CoinRange;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "CoinGecko input")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let prices = map.next_entry::<String, Vec<Vec<f64>>>()?.unwrap().1.into_iter().map(|vec| (vec[0] as u64, vec[1])).collect();
        let market_caps = map.next_entry::<String, Vec<Vec<f64>>>()?.unwrap().1.into_iter().map(|vec| (vec[0] as u64, vec[1])).collect();
        let total_volumes = map.next_entry::<String, Vec<Vec<f64>>>()?.unwrap().1.into_iter().map(|vec| (vec[0] as u64, vec[1])).collect();
        Ok(CoinRange { prices, market_caps, total_volumes })
    }
}

impl <'de> Deserialize<'de> for CoinRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        deserializer.deserialize_map(CoinRangeVisitor {})
    }
}
