```
$ cd ~/hello_rust
$ find . -print | sed -e 's;[^/]*/;|____;g;s;____|; |;g'

.
|____Cargo.toml
|____build-rs-cargo-config
|____.cargo
| |____config.toml
|____src
| |____main.rs
| |____hello
| | |____hello.c
| | |____Makefile
| | |____hello.h
| | |____libhello.so
$


$ cat src/main.rs
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
$


$ cat Cargo.toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2021"

[dependencies]
libc = "0.2"
$


$ cat .cargo/config.toml
[build]

# https://doc.rust-lang.org/cargo/reference/config.html
# https://doc.rust-lang.org/rustc/command-line-arguments.html

rustflags = [
    "-Lnative=/home/ljh/Documents/hello_rust/src/hello", 
    "-ldylib=hello",
]
$


$ cat build-rs-cargo-config
// in favor of .cargo/config.toml

/*
fn main() {
    // ./src/hello/libhello.so
    let path = "./src/hello";
    let name = "hello";
    println!("cargo:rustc-link-search={}", path);
    println!("cargo:rustc-link-lib={}", name);
}
*/
$


$ cat src/hello/hello.c
#include <string.h>
#include "hello.h"

char *strncpy2(char *dest, const char *src, size_t n)
{
    strncpy(dest, src, n);
    return dest;
}

int *intcpy(int *dest, const int *src)
{
    *dest = *src;
    return dest;
}
$


$ cat src/hello/Makefile
# build shared library with -fPIC, -shared
CFLAGS   = -g -fPIC # -O3 # CXXFLAGS for .cpp
LDFLAGS  =  -shared # -L../hello
LDLIBS   = # -lhello
CPPFLAGS = -MMD -MP # -I../hello
#CC      = $(CXX)  # link with CXX for .cpp

# target name is basename of one of the source files
hello: $(patsubst %.c,%.o,$(wildcard *.c))  # .cpp
-include *.d
clean : ; -rm -fr *.o *.d
.PHONY : clean
$


$ cd src/hello
$ make
$ mv hello libhello.so


$ cd ~/hello_rust
$ LD_LIBRARY_PATH=./src/hello/ cargo run
$

```
