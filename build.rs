use std::env;

fn main() {
    let path = env::current_dir().unwrap();
    println!("cargo:rustc-link-lib=dylib=Pentane");
    println!("cargo:rustc-link-search={}/lib", path.display());
}
