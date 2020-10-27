use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // change directory to hacl-c
    let hacl_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("hacl-c");
    env::set_current_dir(&hacl_dir).unwrap();

    // compile code...
    Command::new("./configure")
        .arg("--disable-ocaml")
        .spawn()
        .map(|mut ch| ch.wait().unwrap())
        .unwrap();
    Command::new("make")
        .spawn()
        .map(|mut ch| ch.wait().unwrap())
        .unwrap();

    // link
    println!("cargo:rustc-link-lib=static={}", "evercrypt");
    println!("cargo:rustc-link-search=native={}", hacl_dir.to_str().unwrap());
}
