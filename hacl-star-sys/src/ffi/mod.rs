extern "C" {
    pub fn Hacl_Ed25519_expand_keys(ks: *mut u8, priv_: *const u8);
    pub fn Hacl_Ed25519_sign_expanded(signature: *mut u8, ks: *const u8, len: u32, msg: *const u8);
    pub fn Hacl_Ed25519_sign(signature: *mut u8, priv_: *const u8, len: u32, msg: *const u8);
    pub fn Hacl_Ed25519_verify(pub_: *const u8, len: u32, msg: *const u8, signature: *const u8) -> bool;
    pub fn Hacl_Ed25519_secret_to_public(pub_: *mut u8, priv_: *const u8);
}
