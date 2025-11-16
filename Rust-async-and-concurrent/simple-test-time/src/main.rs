use std::time::{Instant, Duration};
use tokio::time::sleep;
use tokio::io::Error;

async fn task_01() {
    println!("Getting JLoka data-01...");
    sleep(Duration::from_secs(2)).await;
    println!("Getting JLoka data-02...");
    sleep(Duration::from_secs(2)).await;
}

async fn task_02() {
    println!("Getting JLoka data-03...");
    sleep(Duration::from_secs(2)).await;
    println!("Getting JLoka data-04...");
    sleep(Duration::from_secs(2)).await;
}

async fn task_03() {
    println!("Getting JLoka data-05...");
    sleep(Duration::from_secs(2)).await;
    println!("Getting JLoka data-06...");
    sleep(Duration::from_secs(2)).await;
}

#[tokio::main(flavor = "multi_thread", worker_threads=1)]
async fn main() -> Result<(), Error> {
    let start_time = Instant::now();
    // tokio::join!(task_01(), task_02(), task_03());
    let _ = tokio::task::spawn(async {
        let t1 = task_01();
        let t2 = task_02();
        let t3 = task_03();
        tokio::join!(t1, t2, t3);
    }).await;

    println!("Execution finished in: {:?}", start_time.elapsed());
    Ok(())
}
