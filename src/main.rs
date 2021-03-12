const API_URL: &'static str = "https://www.coingecko.com/api/documentations/v3";

struct Client {
    api_url: String,
}

impl Client {
    fn new(&self) -> Self {
        Client { api_url: API_URL.to_string(), }
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