
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::io::{self, Result, Error, ErrorKind};

use winapi::shared::winerror::WAIT_TIMEOUT;
use winapi::um::winbase::{INFINITE, WAIT_OBJECT_0, WAIT_ABANDONED};
use winapi::um::synchapi::{OpenMutexW, CreateMutexW, ReleaseMutex, WaitForSingleObject};

use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::winnt::{HANDLE, MUTANT_ALL_ACCESS};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::minwinbase::SECURITY_ATTRIBUTES;

pub const NULL: HANDLE = 0 as HANDLE;

/// Returns the last error from the Windows socket interface.
fn last_error() -> io::Error {
    io::Error::from_raw_os_error(unsafe { GetLastError() as i32 })
}

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

/// Checks if the signed integer is the Windows constant `SOCKET_ERROR` (-1)
/// and if so, returns the last error from the Windows socket interface. This
/// function must be called before another call to the socket API is made.
pub fn cvt<T: IsMinusOne>(t: T) -> io::Result<T> {
    if t.is_minus_one() {
        Err(last_error())
    } else {
        Ok(t)
    }
}


#[derive(Debug)]
pub struct LockGuard {
    id: HANDLE,
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        let _ = ProcessLock::unlock_by_handle(self.id);
    }
}

#[derive(Debug)]
pub struct ProcessLock {
    id: HANDLE,
}

impl ProcessLock {

    pub fn new(mut name: String, _path_name: Option<String>) -> Result<ProcessLock> {
        if name.len() == 0 {
            name = String::from("process_lock_for_windows_temp");
        }

        unsafe {
            let mut name: Vec<u16> = OsStr::new(&name).encode_wide().chain(once(0)).collect();
            let mut handle;
            handle = OpenMutexW(MUTANT_ALL_ACCESS, 0, name.as_mut_ptr());
            if handle == NULL {
                handle = CreateMutexW(0 as *mut SECURITY_ATTRIBUTES, 0, name.as_mut_ptr());
                if handle == NULL {
                    return Err(last_error());
                }
            }
            return Ok(ProcessLock {
                id: handle,
            })
        }
    }

    pub fn trylock(&self) -> Result<Option<LockGuard>> {
        unsafe {
            match WaitForSingleObject(self.id, 0) {
                WAIT_OBJECT_0 | WAIT_ABANDONED => {
                    Ok(Some(LockGuard {
                        id: self.id,
                    }))
                },
                WAIT_TIMEOUT => {
                    Ok(None)
                },
                _ => {
                    Err(last_error())
                }
            }
        }
    }

    pub fn lock(&self) -> Result<LockGuard> {
        unsafe {
            match WaitForSingleObject(self.id, INFINITE) {
                //另外的进程或者线程被异常退出关闭, 这边收到通常也是算成功得到锁
                WAIT_OBJECT_0 | WAIT_ABANDONED => {
                    Ok(LockGuard {
                        id: self.id,
                    })
                },
                _ => {
                    Err(last_error())
                }
            }
        }
    }

    pub fn unlock(&self) -> Result<()> {
        Self::unlock_by_handle(self.id)?;
        Ok(())
    }

    pub fn unlock_by_handle(id: HANDLE) -> Result<()> {
        unsafe {
            cvt(ReleaseMutex(id))?;
            Ok(())
        }
    }

    pub fn destory(&mut self) -> Result<()> {
        self.check_vaild()?;
        Ok(())
    }

    pub fn check_vaild(&self) -> Result<bool> {
        if self.id == INVALID_HANDLE_VALUE {
            return Err(Error::new(ErrorKind::InvalidData, "no vaild"));
        }
        return Ok(true);
    }
}

impl Drop for ProcessLock {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.id);
        }
    }
}