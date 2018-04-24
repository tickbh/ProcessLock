extern crate process_lock;
extern crate time;
use std::time::{Duration, Instant};
use process_lock::*;

fn main () {
    let now = time::precise_time_ns() / 1000 as u64;
    println!("now = {:?}", now);
    let mut lock = ProcessLock::new(String::from(".xxx1"), None).unwrap();
    
    for i in 0..100 {
        let now = Instant::now();
        println!("repeat i = {:?}", i);
        let _guard = lock.lock().unwrap();
        println!("get lock all use time ===== {}", now.elapsed().as_secs());
        println!("success get lock");
        let ten_millis = ::std::time::Duration::from_millis(2000);
        ::std::thread::sleep(ten_millis);
    }
}
