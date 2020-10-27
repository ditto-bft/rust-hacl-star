extern "C" {
    pub fn Hacl_Ed25519_sign(signature: *mut u8, priv_: *mut u8, len: u32, msg: *mut u8);
    pub fn Hacl_Ed25519_verify(pub_: *mut u8, len: u32, msg: *mut u8, signature: *mut u8) -> bool;
    pub fn Hacl_Ed25519_secret_to_public(pub_: *mut u8, priv_: *mut u8);
}
