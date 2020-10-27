use std::fs;
use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::default::Default;

const LIB: &str = "evercrypt";

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
        .arg("--disable-ocaml")
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

    // add kremlin to include path
    env::set_var(
        "C_INCLUDE_PATH",
        env::join_paths(&[
            hacl_dir.join("kremlin").join("include"),
            hacl_dir.join("kremlin").join("kremlib").join("dist").join("minimal"),
        ]).unwrap()
    );

    // generate binds
    bindgen::Builder::default()
        .header(hacl_dir.join("wrapper.h").to_str().unwrap())
        .ctypes_prefix("crate::libc")
        .use_core()
        //.whitelist_type($white)
        //.whitelist_function($white)
        //.whitelist_var($white)
        .generate().unwrap()
        .write_to_file(mod_dir.join("mod.rs")).unwrap();
}
