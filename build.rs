fn main() {
    println!("cargo:rerun-if-changed=icon.rc");
    println!("cargo:rustc-link-arg=icon.res");
}
