use std::time::Instant;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://jsonplaceholder.typicode.com/todos/1";
    let start_time = Instant::now();

    let response = reqwest::get(url).await?;
    let total_time = start_time.elapsed();
    println!("total time is: {}", total_time.as_millis());

    // let start_time = Instant::now();
    // let (_, _, _, _) = tokio::join!(
    //     reqwest::get(url),
    //     reqwest::get(url),
    //     reqwest::get(url),
    //     reqwest::get(url),
    // );
    // let total_time = start_time.elapsed();
    // println!("total time is: {}", total_time.as_millis());

    if response.status().is_success() {
        let t1 = response.text().await?;
        println!("{}", t1)
    } else {
        println!("Response not ok with status: {}", response.status())
    }

    Ok(())
}
