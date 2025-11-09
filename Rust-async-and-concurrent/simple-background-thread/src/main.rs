use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed; // here we use Relaxed to avoid using condvar with wait 
// every time

fn main() {
    let shared_data = Arc::new((Mutex::new(false), Condvar::new()));
    // we create clones for each thread, so we can't change the state of app by accident
    let shared_data_clone = Arc::clone(&shared_data);
    let STOP = Arc::new(AtomicBool::new(false));
    let STOP_CLONE = Arc::clone(&STOP);

    let background_thread = thread::spawn(move || {
        let (lock, cvar) = &*shared_data_clone;
        let mut received_value = lock.lock().unwrap();
        while !STOP.load(Relaxed) {
            //  we wait on the Condvar notification. 
            received_value = cvar.wait(received_value).unwrap();
            println!("Received value: {}", *received_value);
        }
    });

    let updater_thread = thread::spawn(move || {
        let (lock, cvar) = &*shared_data;
        let values = [false, true, false, true];
        for i in 0..4 {
            let value = values[i as usize];
            println!("Updating value to {}", value);
            *lock.lock().unwrap() = value;
            cvar.notify_one(); // also if we have multiple threads we have notify all
            thread::sleep(Duration::from_secs(2));
        }

        // The Relaxed ordering, used with AtomicBool,
        // ensures that the operations on the atomic variable are visible to all threads but does
        // not enforce any particular order on the surrounding operations.
        STOP_CLONE.store(true, Relaxed);
        println!("update the value of stop");
        cvar.notify_one();
    });

    updater_thread.join().unwrap();

    println!("Hello, world!");
}
