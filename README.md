this is process lock impl multi process for rust 
=====================

[![Build Status](https://travis-ci.org/tickbh/ProcessLock.svg?branch=master)](https://travis-ci.org/tickbh/ProcessLock) [![Crates.io](https://img.shields.io/crates/v/process_lock.svg)](https://crates.io/crates/process_lock)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
process_lock = "0.1"
```

and this to your crate root:

```rust
extern crate process_lock;
```

How to use
```rust
extern crate process_lock;
use std::time::{Duration, Instant};
use process_lock::*;
fn main () {
    let mut lock = ProcessLock::new(String::from(".process_lock"), None).unwrap();
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
```
each process will get the lock in other process release the lock

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
