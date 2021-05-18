use std::fmt::Display;

use serde::{Deserialize, de::Visitor};

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct RawVsCurrency {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct RawCoin {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct RawMarketChart {
    pub prices: Vec<(u128, f64)>,
    pub market_caps: Vec<(u128, f64)>,
    pub total_volumes: Vec<(u128, f64)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VsCurrency {
    pub rowid: i64,
    pub raw: RawVsCurrency,
    pub favourite: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coin {
    pub rowid: i64,
    pub raw: RawCoin,
    pub favourite: bool,
}

#[derive(Debug, Clone)]
pub struct MarketChart {
    pub meta_rowid: i64,
    pub raw: RawMarketChart,
}

struct RawMarketChartVisitor;

impl <'de> Visitor<'de> for RawMarketChartVisitor {
    type Value = RawMarketChart;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Correct JSON from the CoinGecko API")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        fn parse_next_map_entry<'de, A>(map: &mut A) -> Result<Vec<(u128, f64)>, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            Ok(map
                .next_entry::<String, Vec<[f64; 2]>>()?
                .unwrap()
                .1
                .into_iter()
                .map(|[timestamp, value]| (timestamp as u128, value))
                .collect())
        }
        let prices = parse_next_map_entry(&mut map)?;
        let market_caps = parse_next_map_entry(&mut map)?;
        let total_volumes = parse_next_map_entry(&mut map)?;
        Ok(RawMarketChart { prices, market_caps, total_volumes })
    }
}

impl <'de> Deserialize<'de> for RawMarketChart {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        deserializer.deserialize_map(RawMarketChartVisitor {})
    }
}

impl Display for RawVsCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Display for RawCoin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Display for VsCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.favourite {
            write!(f, "[{}!] ", self.rowid)?;
        } else {
            write!(f, "[{}] ", self.rowid)?;
        }
        write!(f, "{}", self.raw.to_string())
    }
}

impl Display for Coin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.favourite {
            write!(f, "[{}!] ", self.rowid)?;
        } else {
            write!(f, "[{}] ", self.rowid)?;
        }
        write!(f, "{}", self.raw.to_string())
    }
}
