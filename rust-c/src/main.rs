extern crate libc;
use libc::{c_char, c_int};
use std::{slice, str, ptr};

extern "C" {
    fn strncpy2(dest: *mut c_char, src: *const c_char, n: usize) -> *const c_char;
    fn intcpy(dest: *mut c_int, src: *const c_int) -> *const c_int;
}

fn main() {
    const LEN: usize = 1024;
    let src = "hello world".to_string();
    let mut dest: [i8; LEN] = [0; LEN];

    let mut d1 = 0;
    let d2 = 123;

    unsafe {

        //string
        strncpy2(dest.as_mut_ptr(), src.as_ptr() as *const i8, LEN - 1);
        let s = str::from_utf8_unchecked(
            slice::from_raw_parts(dest.as_ptr() as *const u8, LEN - 1));
        println!("{}: [{}]", line!(), s.to_string());

        //int
        intcpy(ptr::addr_of_mut!(d1), ptr::addr_of!(d2));
        println!("{}: [{}]", line!(), d1);

    };
}
