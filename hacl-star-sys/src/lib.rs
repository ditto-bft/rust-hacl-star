#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(not(all(target_arch = "wasm32", not(any(target_os = "emscripten", target_os = "wasi")))))]
use libc;

#[cfg(all(target_arch = "wasm32", not(any(target_os = "emscripten", target_os = "wasi"))))]
mod libc {
    pub type c_void = u8;
    pub type c_int = i32;
    pub type c_uint = u32;
    pub type c_ulong = u32;
    pub type c_uchar = u8;
}

mod imp;
pub use imp::*;
