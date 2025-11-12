use reqwest::Error;
use serde::Deserialize;
use tokio::time::sleep;
use std::time::{Duration, Instant};
use serde_json;

#[derive(Deserialize, Debug)]
struct TestResponse {
    url: String,
    origin: String,
    // Note from me: don't use serde_json::Map<String, String>
    headers: serde_json::Map<String, serde_json::Value>,
    args: serde_json::Value,
}


async fn get_data(seconds: u8) -> Result<TestResponse, Error> {
    let request_url = format!("https://httpbin.org/delay/{}", seconds);
    let res = reqwest::get(&request_url).await?;
    let res: TestResponse = res.json().await?;
    return Ok(res)
}

async fn delay_time() {
    sleep(Duration::from_secs(2)).await;
    println!("Finish from delay time");
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let start_time = Instant::now();
    let data = get_data(2);
    let delayed = delay_time();
    let (data, _) = tokio::join!(
        data, delayed
    );
    println!("Data is: {:?}", data);
    println!("Total time is: {:?}", start_time.elapsed());
    Ok(())
}
