use std::env;
fn main() {
    let project_dir = env::current_dir().unwrap();
    println!("cargo:rustc-link-search={}/lib", project_dir.display());
    // println!("cargo:rustc-link-lib=static=gateway");
}
