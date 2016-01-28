extern crate libc;

use libc::c_char;
use std::ffi::CStr;
use std::str;

#[no_mangle]
pub extern "C" fn test_hello_world() {
    println!("Hello world from rust - #2")
}

#[no_mangle]
pub extern "C" fn test_get_num() -> i32 {
    52
}

#[no_mangle]
pub extern "C" fn test_get_array(func : extern "C" fn(i32, *const u8) -> i32) -> i32 {
    let xs: [u8; 5] = [1, 2, 3, 4, 5];
    func(xs.len() as i32, xs.as_ptr())
}

#[no_mangle]
pub extern "C" fn test_combine(func : extern "C" fn(i32,i32) -> i32) -> i32 {
    func(2,3)
}

#[no_mangle]
pub extern "C" fn test_send_string(c_buf : *const c_char) {
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let buf: &[u8] = c_str.to_bytes();
    let str_slice: &str = str::from_utf8(buf).unwrap();
    let str_buf: String = str_slice.to_owned();
    println!("Rust received a string: {}", str_buf)
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}
