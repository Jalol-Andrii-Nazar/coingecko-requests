use std::collections::HashMap;

use directories::ProjectDirs;
use sqlx::{Connection, Row, SqliteConnection};
use futures::TryStreamExt;
use crate::{data, api_client};

pub struct Client {
    api_client: api_client::Client,
    conn: SqliteConnection
}

impl Client {
    pub async fn new(api_client: api_client::Client) -> Result<Self, Box<dyn std::error::Error>> {
        let project_dirs = ProjectDirs::from("org", "jna", "coingecko_requests")
            .ok_or::<Box<dyn std::error::Error>>(From::from("Failed to get project_dirs!"))?;
        let data_dir = project_dirs.data_dir().to_path_buf();
        tokio::fs::create_dir_all(&data_dir).await?;
        let mut db_path = data_dir;
        db_path.push("data");
        db_path.set_extension("db");
        tokio::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(&db_path)
            .await?;
        let db_path_str = db_path.to_str()
            .ok_or::<Box<dyn std::error::Error>>(From::from("db_path cannot be converted to str!"))?;
        let connection_url = format!("sqlite:{}", db_path_str);
        let mut conn: SqliteConnection = SqliteConnection::connect(&connection_url).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS vs_currencies (name TEXT, favourite BOOL)")
            .execute(&mut conn)
            .await?;
        sqlx::query("CREATE TABLE IF NOT EXISTS coins (id TEXT, symbol TEXT, name TEXT, favourite BOOL)")
            .execute(&mut conn)
            .await?;
        sqlx::query("CREATE TABLE IF NOT EXISTS market_chart_range_meta (id TEXT, currency TEXT, from_timestamp INTEGER, to_timestamp INTEGER)")
            .execute(&mut conn)
            .await?;
        sqlx::query("CREATE TABLE IF NOT EXISTS market_chart_range_prices (parent_rowid INTEGER, timestamp INTEGER, value REAL, CONSTRAINT parent_fk FOREIGN KEY (parent_rowid) REFERENCES market_chart_range_meta (rowid))")
            .execute(&mut conn)
            .await?;
        sqlx::query("CREATE TABLE IF NOT EXISTS market_chart_range_market_caps (parent_rowid INTEGER, timestamp INTEGER, value REAL, CONSTRAINT parent_fk FOREIGN KEY (parent_rowid) REFERENCES market_chart_range_meta (rowid))")
            .execute(&mut conn)
            .await?;
        sqlx::query("CREATE TABLE IF NOT EXISTS market_chart_range_total_volumes (parent_rowid INTEGER, timestamp INTEGER, value REAL, CONSTRAINT parent_fk FOREIGN KEY (parent_rowid) REFERENCES market_chart_range_meta (rowid))")
            .execute(&mut conn)
            .await?;

        Ok(Self {
            api_client,
            conn
        })
    }

    pub async fn ping(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(self.api_client.ping().await?)
    }

    pub async fn price(&self, ids: &[&str], vs_currencies: &[&str]) -> Result<HashMap<String, HashMap<String, f64>>, Box<dyn std::error::Error>> {
        Ok(self.api_client.price(ids, vs_currencies).await?)
    }

    pub async fn vs_currencies(&mut self) -> Result<Vec<data::VsCurrency>, Box<dyn std::error::Error>> {
        let count: i64 = sqlx::query("SELECT COUNT(*) FROM vs_currencies")
            .fetch_one(&mut self.conn)
            .await?
            .try_get(0)?;
        if count <= 0 {
            self.populate_vs_currencies().await?;
        }
        let mut rows = sqlx::query("SELECT rowid, name, favourite FROM vs_currencies")
            .fetch(&mut self.conn);
        let mut vec = Vec::new();
        while let Some(row) = rows.try_next().await? {
            let rowid: i64 = row.try_get("rowid")?;
            let name: String = row.try_get("name")?;
            let favourite: bool = row.try_get("favourite")?;
            vec.push(data::VsCurrency {
                rowid,
                raw: data::RawVsCurrency {
                    name,
                },
                favourite
            });
        }
        Ok(vec)
    }

    pub async fn favourite_vs_currencies(&mut self) -> Result<Vec<data::VsCurrency>, Box<dyn std::error::Error>> {
        Ok(self.vs_currencies()
            .await?
            .into_iter()
            .filter(|vs_currency| vs_currency.favourite)
            .collect())
    }

    pub async fn set_favourite_vs_currency(&mut self, id: i64, is_favourite: bool) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query("UPDATE vs_currencies SET favourite = ? WHERE rowid = ?")
            .bind(is_favourite)
            .bind(id)
            .execute(&mut self.conn)
            .await?;
        Ok(())
    }

    pub async fn coins(&mut self) -> Result<Vec<data::Coin>, Box<dyn std::error::Error>> {
        let count: i64 = sqlx::query("SELECT COUNT(*) FROM coins")
            .fetch_one(&mut self.conn)
            .await?
            .try_get(0)?;
        if count <= 0 {
            self.populate_coins().await?;
        }
        let mut rows = sqlx::query("SELECT rowid, id, symbol, name, favourite FROM coins")
            .fetch(&mut self.conn);
        let mut vec = Vec::new();
        while let Some(row) = rows.try_next().await? {
            let rowid: i64 = row.try_get("rowid")?;
            let id: String = row.try_get("id")?;
            let symbol: String = row.try_get("symbol")?;
            let name: String = row.try_get("name")?;
            let favourite: bool = row.try_get("favourite")?;
            vec.push(data::Coin {
                rowid,
                raw: data::RawCoin {
                    id,
                    symbol,
                    name,
                },
                favourite
            });
        }
        Ok(vec)
    }

    pub async fn favourite_coins(&mut self) -> Result<Vec<data::Coin>, Box<dyn std::error::Error>> {
        Ok(self.coins()
            .await?
            .into_iter()
            .filter(|coin| coin.favourite)
            .collect())
    }

    pub async fn set_favourite_coin(&mut self, id: i64, is_favourite: bool) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query("UPDATE coins SET favourite = ? WHERE rowid = ?")
            .bind(is_favourite)
            .bind(id)
            .execute(&mut self.conn)
            .await?;
        Ok(())
    }

    pub async fn market_chart(&mut self, _id: &str, _currency: &str, _from: u64, _to: u64) -> Result<data::RawMarketChart, Box<dyn std::error::Error>> {
        todo!()        
    }

    async fn populate_vs_currencies(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Receiving vs currencies data from the CoinGecko API...");
        let default_favourite_vs_currencies = vec!["btc", "eth", "ltc", "usd", "eur", "cad", "aud", "jpy", "pln", "rub", "uah"];
        let data = self.api_client.vs_currencies().await?;
        for vs_currency in data.iter() {
            let name = &vs_currency.name;
            let is_favourite = default_favourite_vs_currencies.contains(&name.as_str());
            sqlx::query("INSERT INTO vs_currencies (name, favourite) VALUES (?, ?)")
                .bind(name)
                .bind(is_favourite)
                .execute(&mut self.conn)
                .await?;
        }
        Ok(())
    }

    async fn populate_coins(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Receiving coins data from the CoinGecko API...");
        let default_favourite_coins = vec!["btc", "ltc", "eth"];
        let data = self.api_client.coins().await?;
        for coin in data.iter() {
            let is_favourite = default_favourite_coins.contains(&coin.symbol.as_str());
            sqlx::query("INSERT INTO coins (id, symbol, name, favourite) VALUES (?, ?, ?, ?)")
                .bind(&coin.id)
                .bind(&coin.symbol)
                .bind(&coin.name)
                .bind(is_favourite)
                .execute(&mut self.conn)
                .await?;
        }
        Ok(())
    }
}