struct Client {
    api_url: String,
}

impl Client {
    async fn new(&self) -> Self {
        Client { api_url: String::from("https://www.coingecko.com/api/documentations/v3/") }
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