use reqwest::Error;
use serde::Deserialize;
use tokio::time::sleep;
use std::time::{Duration, Instant}
use serde_json;

#[derive(Deserialize, Debug)]
struct TestResponse {
    url: String,
    args: serde_json::Value,
}

async fn get_data() -> Result<TestResponse, Error> {
    let request_url = "https://example.com";
    let res = reqwest::get(request_url).await?;
    let res: TestResponse = res.json().await?;
    return Ok(res)
}

fn main() {
    println!("Hello, world!");
}
