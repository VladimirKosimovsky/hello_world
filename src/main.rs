#[tokio::main]
async fn main() -> Result<(), Box<(dyn std::error::Error)>> {
    let client = reqwest::Client::builder().build()?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    let data = r#"{
    "model": "llama3.2",
    "prompt": "Why is the sky blue?",
    "stream": false
}"#;

    let json: serde_json::Value = serde_json::from_str(&data)?;

    let request = client
        .request(reqwest::Method::POST, "http://localhost:11434/api/generate")
        .headers(headers)
        .json(&json);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}
