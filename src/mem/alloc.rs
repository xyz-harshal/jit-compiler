use libc;
use std::ptr;

pub struct ExecutableMem {
    ptr: *mut u8, //raw pointer to the first u8
    size: usize, //size of the executable memory
}

impl ExecutableMem {
    //It takes how many bytes you want, tries to allocate them via mmap
    pub fn new(capacity: usize) -> Result<Self, String> {
        unsafe {
            let res = libc::mmap(
                std::ptr::null_mut(),
                capacity,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
                -1,
                0
            );
            if res == libc::MAP_FAILED {
                Err("Failed to allocate Executable Memory".to_string())
            }else {
                Ok(Self { ptr: res as *mut u8, size: capacity })
            }
        }
    }

    pub fn write_code(&self, buf: Vec<u8>) -> Result<(), String> {
        if buf.len() > self.size {
            return Err("buffer size exceeds the allocated memory size".to_string());
        }
        unsafe {
            ptr::copy_nonoverlapping(buf.as_ptr(), self.ptr, buf.len());
        }
        Ok(())
    }

    pub fn make_executable(&self) -> Result<(), String> {
        unsafe {
            //mempory protect (mprotect) is a function that changes the permission flags of the given memory mapped region
            let result = libc::mprotect(
                self.ptr as *mut c_void,
                self.size,
                libc::PROT_READ | libc::PROT_EXEC
            );
            match result {
                0 => Ok(()),
                _ => Err("Couldn't make the buffer executable".to_string()),
            }
        }
    }

    pub fn execute_code() -> {

    }
}
