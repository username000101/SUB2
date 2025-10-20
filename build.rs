use std::env::var;
use std::path::PathBuf;

fn main() {
    let tdjson_path = PathBuf::from(var("TDJSON_PATH").expect("Need to specify TDJSON_PATH environment variable"));
    if !tdjson_path.exists() {
        panic!("TDJSON_PATH points to a non-existent directory(TDJSON_PATH: {})", tdjson_path.display());
    }

    println!("cargo:rustc-link-search=native={}", tdjson_path.display());
}