extern crate process_lock;
extern crate time;
use process_lock::*;

fn main () {
    let now = time::precise_time_ns() / 1000 as u64;
    println!("now = {:?}", now);
    let mut lock = ProcessLock::new_open(String::from(".1211"), None);

    println!("fkkkkkkkkkkkkkkkkkkkkkk");
    // println!("lock = {:?}", lock);
    let mut lock = lock.unwrap();
    if now % 2 == 0 {
        println!("exec lock()");
        let _guard = lock.lock();
        println!("success get lock");
        loop {

        }
    } else {
        println!("exec trylock()");
        let _guard = lock.trylock();
        println!("_guard = {:?}", _guard);
        loop {

        }
    };
}
