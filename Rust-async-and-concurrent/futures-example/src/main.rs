use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread::sleep;
use std::time::{Duration, Instant};
use tokio::task::JoinHandle;

struct CounterExample {
    count: u32,
}

impl Future for CounterExample {
    type Output = u32;

    /**
     * Here the future are moved from the address of main to address of tokio module,
     * so we use Pin
     */
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.count += 1;
        println!("Count is: {}", self.count);
        sleep(Duration::from_secs(1));
        if self.count < 3 {
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(self.count)
        }
    }
}

/**
 *  In Rust, the compiler often moves values around in memory. For instance, if we move
 *  a variable into a function, the memory may be moved.
 *  Most normal primitives such as number, string, bool, structs, and enum implement
 *  the Unpin trait, enabling them to be moved around.
 */

#[tokio::main]
async fn main() {
    let t1 = CounterExample { count: 0 }; 
    let t2 = CounterExample { count: 0 };
    let start = Instant::now();
    let simple_01: JoinHandle<u32> = tokio::task::spawn(async { t1.await });
    let simple_02: JoinHandle<u32> = tokio::task::spawn(async { t2.await });

    let (res1, res2) = tokio::join!(simple_01, simple_02);
    println!("Res1 is: {}, Res2 is: {}", res1.unwrap(), res2.unwrap());
    println!("The time is: {:?}", start.elapsed());
}
