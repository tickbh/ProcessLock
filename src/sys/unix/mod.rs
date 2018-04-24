use libc;

use std::io::{self, Result, Error, ErrorKind};

#[allow(dead_code)]
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(target_os = "linux")]
pub enum Sem {
  GETPID  = 11,
  GETVAL  = 12,
  GETALL  = 13,
  GETZCNT = 15,
  SETVAL  = 16,
  SETALL  = 17,
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(target_os = "macos")]
pub enum Sem {
  GETPID  = 4,
  GETVAL  = 5,
  GETALL  = 6,
  GETZCNT = 7,
  SETVAL  = 8,
  SETALL  = 9,
}

pub const SEM_NUM: u16 = 0;
pub const NSOPS: usize = 1;
pub const SEM_UNDO: i16 = 0x1000;

#[doc(hidden)]
pub trait IsMinusOne {
    fn is_minus_one(&self) -> bool;
}

macro_rules! impl_is_minus_one {
    ($($t:ident)*) => ($(impl IsMinusOne for $t {
        fn is_minus_one(&self) -> bool {
            *self == -1
        }
    })*)
}

impl_is_minus_one! { i8 i16 i32 i64 isize }

pub fn cvt<T: IsMinusOne>(t: T) -> io::Result<T> {
    if t.is_minus_one() {
        Err(io::Error::last_os_error())
    } else {
        Ok(t)
    }
}

#[derive(Debug)]
pub struct LockGuard {
    id: libc::c_int,
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        let _ = ProcessLock::unlock_by_id(self.id);
    }
}

pub struct ProcessLock {
    id: libc::c_int,
}


impl ProcessLock {
    fn hash_code(name: &String) -> i32 {
        let bytes = name.as_bytes();
        let mut h = 0 as i32;
        for byte in bytes {
            h = h.wrapping_mul(31).wrapping_add(*byte as i32);
        }
        return h;
    }

    pub fn new(name: String, path_name: Option<String>) -> Result<ProcessLock> {
        let path = path_name.clone().unwrap_or(String::from("."));
        let code = Self::hash_code(&name);
        unsafe {
            let key = cvt(libc::ftok(path.as_bytes().as_ptr() as *mut i8, code))?;
            let (id, is_create) = match cvt(libc::semget(key, 1024, 0o0666)) {
                Ok(id) => {
                    (id, false)
                }
                Err(_) => {
                    (cvt(libc::semget(key, 1024, 0o0666 | libc::IPC_CREAT | libc::IPC_EXCL))?, true)
                }
            };
            if is_create {
                // 如果要得到信号量的值, 直接用Sem::GETVAL, 但是GETVAL是直接做为返回值返回的
                cvt(libc::semctl(id, SEM_NUM as i32, Sem::SETVAL as libc::c_int, 1))?;
            }
            return Ok(ProcessLock {
                id: id,
            })
        }
    }

    pub fn trylock(&self) -> Result<Option<LockGuard>> {
        let mut op = libc::sembuf {
            sem_num: SEM_NUM,
            sem_op: -1,
            sem_flg: SEM_UNDO | libc::IPC_NOWAIT as i16,
        };
        unsafe {
            let ret = libc::semop(self.id, &mut op as *mut libc::sembuf, NSOPS);
            let err = io::Error::last_os_error();
            if err.raw_os_error() == Some(libc::EAGAIN) {
                return Ok(None);
            } else if ret == -1 {
                return Err(err);
            }
            Ok(Some(LockGuard {
                id: self.id
            }))
        }
    }

    pub fn lock(&self) -> Result<LockGuard> {
        unsafe {
            let mut op = libc::sembuf {
                sem_num: SEM_NUM,
                sem_op: -1,
                sem_flg: SEM_UNDO,
            };
            cvt(libc::semop(self.id, &mut op as *mut libc::sembuf, NSOPS))?;
            Ok(LockGuard {
                id: self.id
            })
        }
    }

    pub fn unlock(&self) -> Result<()> {
        Self::unlock_by_id(self.id)
    }

    pub fn unlock_by_id(id: libc::c_int) -> Result<()> {
        unsafe {
            let mut op = libc::sembuf {
                sem_num: SEM_NUM,
                sem_op: 1,
                sem_flg: SEM_UNDO,
            };
            cvt(libc::semop(id, &mut op, NSOPS))?;
            Ok(())
        }
    }

    pub fn destory(&mut self) -> Result<()> {
        self.check_vaild()?;
        unsafe {
            cvt(libc::semctl(self.id, SEM_NUM as i32, libc::IPC_RMID))?;
            self.id = -1;
        }
        Ok(())
    }

    pub fn check_vaild(&self) -> Result<bool> {
        if self.id == -1 {
            return Err(Error::new(ErrorKind::InvalidData, "no vaild"));
        }
        return Ok(true);
    }
}
