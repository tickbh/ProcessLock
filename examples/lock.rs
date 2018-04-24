extern crate process_lock;
use std::time::{Duration, Instant};
use process_lock::*;

fn main () {
    let lock = ProcessLock::new(String::from(".process_lock"), None).unwrap();
    
    for i in 0..100 {
        let now = Instant::now();
        {
            let _guard = lock.lock().unwrap();
            println!("success get the {} lock lock all use time ===== {}", i, now.elapsed().as_secs());
            let ten_millis = ::std::time::Duration::from_millis(2000);
            ::std::thread::sleep(ten_millis);
        }
        let ten_millis = ::std::time::Duration::from_millis(100);
        ::std::thread::sleep(ten_millis);
    }
}
