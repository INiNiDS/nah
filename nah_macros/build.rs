fn main() {
    println!("cargo:rerun-if-env-changed=DEPLOY");
    let val = std::env::var("DEPLOY").unwrap_or_default();
    println!("cargo:rustc-env=NAH_DEPLOY={}", val);
}
