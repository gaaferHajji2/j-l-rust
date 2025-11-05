use std::time::Instant;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://jsonplaceholder.typicode.com/todos/1";
    let start_time = Instant::now();

    let _ = reqwest::get(url).await?;
    let total_time = start_time.elapsed();
    println!("total time is: {}", total_time.as_millis());

    Ok(())
}
