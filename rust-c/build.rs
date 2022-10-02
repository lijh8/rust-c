fn main() {
    // ./src/hello/libhello.so
    let path = "./src/hello";
    let name = "hello";
    println!("cargo:rustc-link-search={}", path);
    println!("cargo:rustc-link-lib={}", name);
}
