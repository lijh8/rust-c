extern crate libc;
use libc::{c_char};
use std::{slice, str};

extern "C" {
    fn strncpy2(dest: *mut c_char, src: *const c_char, n: usize) -> *const c_char;
}

fn main() {
    const LEN: usize = 1024;
    let src = "hello world";
    let mut dest: [i8; LEN] = [0; LEN];

    unsafe {

        strncpy2(dest.as_mut_ptr() as *mut c_char,
                 src.as_ptr() as *const c_char, LEN - 1);
        let s = str::from_utf8_unchecked(
            slice::from_raw_parts(dest.as_mut_ptr() as *mut u8, src.len()));
        println!("{}: |{}|", line!(), s.to_string());

    };
}
