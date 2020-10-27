use std::fs;
use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::default::Default;

const LIB: &str = "libevercrypt.a";

fn main() {
    // create the destiny folder for the resulting bindings
    let mod_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("src")
        .join("ffi");

    if let Err(e) = fs::create_dir(&mod_dir) {
        if e.kind() != std::io::ErrorKind::AlreadyExists {
            panic!(format!("{}", e));
        }
    }

    // change directory to hacl-c
    let hacl_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("hacl-c");
    env::set_current_dir(&hacl_dir).unwrap();

    // compile code...
    Command::new("./configure")
        .spawn()
        .map(|mut ch| ch.wait().unwrap())
        .unwrap();
    Command::new("make")
        .spawn()
        .map(|mut ch| ch.wait().unwrap())
        .unwrap();

    // link
    println!("cargo:rustc-link-lib=static={}", LIB);
    println!("cargo:rustc-link-search=native={}", hacl_dir.to_str().unwrap());

    // generate binds
    bindgen::Builder::default()
        .header(mod_dir.to_str().unwrap())
        .ctypes_prefix("crate::libc")
        .use_core()
        //.whitelist_type($white)
        //.whitelist_function($white)
        //.whitelist_var($white)
        .generate().unwrap()
        .write_to_file(mod_dir.join("mod.rs")).unwrap();
}
