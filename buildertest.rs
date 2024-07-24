fn main() {
    println!("cargo:rerun-if-changed=add.o");
    println!("cargo:rustc-link-search=.");
    println!("cargo:rustc-link-arg=add.o");
}
