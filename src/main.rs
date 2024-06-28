use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();

    let params = [
        ("api_key", "[your API key]"),
        ("url", "https://example.com"),
    ];

    let response = client.get("https://scraping.narf.ai/api/v1/")
        .query(&params)
        .send()
        .await?;

    let body = response.text().await?;
    println!("{}", body);

    Ok(())
}