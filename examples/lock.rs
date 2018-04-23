extern crate process_lock;
extern crate time;
use process_lock::*;

fn main () {
    let now = time::precise_time_ns() / 1000 as u64;
    println!("now = {:?}", now);
    let mut lock = ProcessLock::new_create(String::from(".xxx1"), None);
    

    println!("fkkkkkkkkkkkkkkkkkkkkkk");
    // println!("lock = {:?}", lock);
    let mut lock = lock.unwrap();
    if now % 2 == 0 {
        println!("exec lock()");
        let _guard = lock.lock().unwrap();
        println!("success get lock");
        // drop(_guard);
        lock.destory();
        loop {

        }
    } else {
        println!("exec trylock()");
        let _guard = lock.trylock().unwrap();
        println!("_guard = {:?}", _guard);
        if _guard.is_some() {
            println!("success get lock");
        }
        loop {

        }
    };
}
