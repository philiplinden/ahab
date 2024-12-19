fn main() {
    // https://github.com/embassy-rs/embassy/blob/main/examples/stm32f4/build.rs
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
