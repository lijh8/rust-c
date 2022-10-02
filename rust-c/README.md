```

$ cd ~/hello_rust
$ find . -print | sed -e 's;[^/]*/;|____;g;s;____|; |;g'

.
|____Cargo.toml
|____build.rs
|____src
| |____main.rs
| |____hello
| | |____hello.h
| | |____Makefile
| | |____hello.c
| | |____libhello.so
$


$ cat src/main.rs
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

        strncpy2(dest.as_ptr() as *mut c_char,
                 src.as_ptr() as *const c_char, LEN - 1);
        let s = str::from_utf8_unchecked(
            slice::from_raw_parts(dest.as_mut_ptr() as *mut u8, src.len()));
        println!("{}: |{}|", line!(), s.to_string());

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


$ cat build.rs
fn main() {
    // ./src/hello/libhello.so
    let path = "./src/hello";
    let name = "hello";
    println!("cargo:rustc-link-search={}", path);
    println!("cargo:rustc-link-lib={}", name);
}
$


$ cat src/hello/hello.c
#include <string.h>
#include "hello.h"

char *strncpy2(char *dest, const char *src, size_t n)
{
    strncpy(dest, src, n);
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
