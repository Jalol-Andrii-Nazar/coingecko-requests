use std::fs::OpenOptions;

use directories::ProjectDirs;
use sqlx::{Connection, SqliteConnection, Row};
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
        std::fs::create_dir_all(&data_dir)?;
        let mut db_path = data_dir;
        db_path.push("data");
        db_path.set_extension("db");
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(&db_path)?;
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

        Ok(Self {
            api_client,
            conn
        })
    }

    pub async fn supported_vs_currencies(&mut self) -> Result<Vec<data::VsCurrency>, Box<dyn std::error::Error>> {
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

    pub async fn coins_list(&mut self) -> Result<Vec<data::Coin>, Box<dyn std::error::Error>> {
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

    pub async fn market_chart_range(&mut self, _id: &str, _currency: &str, _from: u64, _to: u64) -> Result<data::RawMarketChart, Box<dyn std::error::Error>> {
        todo!()        
    }

    async fn populate_vs_currencies(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Receiving vs currencies data from the CoinGecko API...");
        let default_favourite_vs_currencies = vec!["btc", "eth", "ltc", "usd", "eur", "cad", "aud", "jpy", "pln", "rub", "uah"];
        let data = self.api_client.supported_vs_currencies().await?;
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
        let data = self.api_client.coins_list().await?;
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