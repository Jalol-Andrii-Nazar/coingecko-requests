use sqlx::{Connection, SqliteConnection, Row};
use futures::TryStreamExt;

pub struct CachingClient {
    api_client: crate::client::Client,
    conn: SqliteConnection
}

impl CachingClient {
    pub async fn new(api_client: crate::client::Client) -> Result<Self, Box<dyn std::error::Error>> {
        let mut conn: SqliteConnection = SqliteConnection::connect("sqlite::memory:").await?;
        let create_supported_vs_currencies_query = sqlx::query("CREATE TABLE IF NOT EXISTS supported_vs_currencies (name TEXT)");
        create_supported_vs_currencies_query.execute(&mut conn).await?;
        let add_test_currency = sqlx::query("INSERT INTO supported_vs_currencies VALUES ('my test currency')");
        add_test_currency.execute(&mut conn).await?;
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
        Ok(vec)
    }
}