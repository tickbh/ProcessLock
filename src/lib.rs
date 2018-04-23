
extern crate libc;
extern crate winapi;
extern crate share_memory;

use std::io::Result;

mod sys;
pub use sys::LockGuard;
pub struct ProcessLock(sys::ProcessLock);
/// 提供进程间的共享内存模块
/// 其中unix使用sem_init来实现
/// 其实windows使用CreateMutex来实现
/// # Examples
///
/// ```no_run
/// ```
impl ProcessLock {
    /// 创建一个进程锁, 或者打开已存在的进程锁
    pub fn new_create(name: String, path_name: Option<String>) -> Result<ProcessLock> {
        Ok(ProcessLock(sys::ProcessLock::new_create(name, path_name)?))
    }

    /// 创建一个进程锁, 或者打开已存在的进程锁
    pub fn new_open(name: String, path_name: Option<String>) -> Result<ProcessLock> {
        Ok(ProcessLock(sys::ProcessLock::new_open(name, path_name)?))
    }

    pub fn trylock(&self) -> Result<Option<LockGuard>> {
        self.0.trylock()
    }

    pub fn lock(&self) -> Result<LockGuard> {
        self.0.lock()
    }

    pub fn unlock(&self) -> Result<()> {
        self.0.unlock()
    }

    pub fn destory(&mut self) -> Result<()> {
        self.0.destory()
    }
}