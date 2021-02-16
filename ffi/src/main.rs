use std::os::raw::{c_char, c_uint};
use std::ffi::CString;

extern "C" {
    fn mystrlen(str: *const c_char) -> c_uint;
}

fn main() {
    let c_string = CString::new("C From Rust").expect("failed");
    let count = unsafe {
        mystrlen(c_string.as_ptr())
    };
    println!("c_string's length is {}", count);
}

fn safe_mystrlen(str: &str) -> Option<u32> {
    let c_string = match CString::new(str) {
        Ok(c) => c,
        Err(_) => return None
    };
    unsafe {
        Some(mystrlen(c_string.as_ptr()))
    }
}