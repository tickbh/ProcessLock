
extern crate libc;
extern crate winapi;

use std::io::Result;

mod sys;
pub use sys::LockGuard;
pub struct ProcessLock(sys::ProcessLock);
/// 提供进程间的共享内存模块
/// 其中unix使用semget, semctl, semop, 其中用特殊的标识位SEM_UNDO来实现, 防止进程意外退出导致的死锁
/// 其实windows使用CreateMutex, ReleaseMutex, WaitForSingleObject来实现
/// # Examples, Open Double Process for same
///
/// ```no_run
/// extern crate process_lock;
/// use std::time::{Duration, Instant};
/// use process_lock::*;
/// fn main () {
///     let mut lock = ProcessLock::new(String::from(".process_lock"), None).unwrap();
///     for i in 0..100 {
///        let now = Instant::now();
///        {
///            let _guard = lock.lock().unwrap();
///            println!("success get the {} lock lock all use time ===== {}", i, now.elapsed().as_secs());
///            let ten_millis = ::std::time::Duration::from_millis(2000);
///            ::std::thread::sleep(ten_millis);
///        }
///        let ten_millis = ::std::time::Duration::from_millis(100);
///        ::std::thread::sleep(ten_millis);
///     }
/// }
/// ```
impl ProcessLock {
    /// 首先打开命名的进程锁, 如果打开失败, 则创建一个新的进程锁, 并赋与初始变量
    pub fn new(name: String, path_name: Option<String>) -> Result<ProcessLock> {
        Ok(ProcessLock(sys::ProcessLock::new(name, path_name)?))
    }

    /// 立即返回获取进程锁, 如果成功则返回LockGuard, 如果失败则返回Ok(None), 如果发生错误则返回Error
    pub fn trylock(&self) -> Result<Option<LockGuard>> {
        self.0.trylock()
    }

    /// 无限等待获取进程锁, 如果成功返回LockGuard, 如果被打断则系统发生错误, 可查看Error处理具体的错误
    /// LockGuard析构会自动解锁
    pub fn lock(&self) -> Result<LockGuard> {
        self.0.lock()
    }

    /// 释放进程锁, 通常情况不主动调用, 由LockGuard的生存周期来控制锁的占用周期
    pub fn unlock(&self) -> Result<()> {
        self.0.unlock()
    }

    /// 销毁进程锁, 把handle或者sem_id主动删除
    pub fn destory(&mut self) -> Result<()> {
        self.0.destory()
    }
}