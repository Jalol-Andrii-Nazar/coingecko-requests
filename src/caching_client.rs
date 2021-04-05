use sqlx::{Connection, SqliteConnection, Row};
use futures::TryStreamExt;
use crate::data;

pub struct CachingClient {
    api_client: crate::client::Client,
    conn: SqliteConnection
}

impl CachingClient {
    pub async fn new(api_client: crate::client::Client) -> Result<Self, Box<dyn std::error::Error>> {
        let mut conn: SqliteConnection = SqliteConnection::connect("sqlite::memory:").await?;
        sqlx::query("CREATE TABLE IF NOT EXISTS supported_vs_currencies (name TEXT)")
            .execute(&mut conn)
            .await?;
        sqlx::query("CREATE TABLE IF NOT EXISTS coins (id TEXT, symbol TEXT, name TEXT)")
            .execute(&mut conn)
            .await?;
        Ok(CachingClient {
            api_client,
            conn
        })
    }

    pub async fn supported_vs_currencies(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let query = sqlx::query("SELECT name FROM supported_vs_currencies");
        let mut rows = query.fetch(&mut self.conn);
        let mut vec = Vec::new();
        while let Some(row) = rows.try_next().await? {
            let name: &str = row.try_get("name")?;
            vec.push(name.to_string());
        }
        drop(rows); //Required to take a mutable borrow to self later
        if !vec.is_empty() {
            println!("Getting the supported vs currencies data from DB!");
            Ok(vec)
        } else {
            println!("Getting the supported vs currencies data from API!");
            let data = self.api_client.supported_vs_currencies().await?;
            for name in data.iter() {
                let insert_query = sqlx::query("INSERT INTO supported_vs_currencies (name) VALUES (?)")
                    .bind(name)
                    .execute(&mut self.conn)
                    .await?;
            }
            Ok(data)
        }
    }

    pub async fn coins_list(&mut self) -> Result<Vec<data::Coin>, Box<dyn std::error::Error>> {
        let query = sqlx::query("SELECT id, symbol, name FROM coins");
        let mut rows = query.fetch(&mut self.conn);
        let mut vec = Vec::new();
        while let Some(row) = rows.try_next().await? {
            let id: &str = row.try_get("id")?;
            let symbol: &str = row.try_get("symbol")?;
            let name: &str = row.try_get("name")?;
            vec.push(data::Coin {
                id: id.to_string(),
                symbol: symbol.to_string(),
                name: name.to_string()
            });
        }
        drop(rows); //Required to take a mutable borrow to self later
        if !vec.is_empty() {
            println!("Getting the coins data from DB!");
            Ok(vec)
        } else {
            println!("Getting the coins data from API!");
            let data = self.api_client.coins_list().await?;
            for coin in data.iter() {
                let insert_query = sqlx::query("INSERT INTO coins (id, symbol, name) VALUES (?, ?, ?)")
                    .bind(&coin.id)
                    .bind(&coin.symbol)
                    .bind(&coin.name)
                    .execute(&mut self.conn)
                    .await?;
            }
            Ok(data)
        }
    }
}