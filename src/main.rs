const API_URL: &'static str = "https://api.coingecko.com/api/v3";

struct Client {}

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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client.ping().await?;
    println!("Ping result: {}", res);
    Ok(())
}