const API_URL: &'static str = "https://www.coingecko.com/api/documentations/v3";

struct Client {}

impl Client {
    fn new(&self) -> Self {
        Self {}
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("ping")
        .await?
        .text()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}