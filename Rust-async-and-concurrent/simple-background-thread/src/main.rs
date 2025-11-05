use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;

fn main() {

    let shared_data = Arc::new((Mutex::new(false), Condvar::new()));
    let shared_data_clone = Arc::clone(&shared_data);
    let STOP = Arc::new(AtomicBool::new(false));
    let STOP_CLONE = Arc::clone(&STOP);

    println!("Hello, world!");
}
