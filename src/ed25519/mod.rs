use hacl_star_sys::ffi;

pub const EXPANDED_SECRET_LENGTH: usize = 64;
pub const SECRET_LENGTH: usize = 32;
pub const PUBLIC_LENGTH: usize = 64;
pub const SIG_LENGTH: usize = 64;

#[derive(Clone)]
pub struct ExpandedSecretKey(pub [u8; EXPANDED_SECRET_LENGTH]);

#[derive(Clone)]
pub struct SecretKey(pub [u8; SECRET_LENGTH]);

#[derive(Clone)]
pub struct PublicKey(pub [u8; PUBLIC_LENGTH]);

#[derive(Clone)]
pub struct Signature(pub [u8; SIG_LENGTH]);

impl ExpandedSecretKey {
    pub fn signature(&self, msg: &[u8]) -> Signature {
        let ExpandedSecretKey(ref sk) = self;
        let mut sig = [0; SIG_LENGTH];

        unsafe {
            ffi::Hacl_Ed25519_sign_expanded(
                sig.as_mut_ptr(),
                sk.as_ptr(),
                msg.len() as u32,
                msg.as_ptr()
            );
        }

        Signature(sig)
    }
}

impl SecretKey {
    pub fn get_public(&self) -> PublicKey {
        let SecretKey(sk) = self;
        let mut pk = [0; PUBLIC_LENGTH];

        unsafe {
            ffi::Hacl_Ed25519_secret_to_public(
                pk.as_mut_ptr(),
                sk.as_ptr()
            );
        }

        PublicKey(pk)
    }

    pub fn signature(&self, msg: &[u8]) -> Signature {
        let SecretKey(ref sk) = self;
        let mut sig = [0; SIG_LENGTH];

        unsafe {
            ffi::Hacl_Ed25519_sign(
                sig.as_mut_ptr(),
                sk.as_ptr(),
                msg.len() as u32,
                msg.as_ptr()
            );
        }

        Signature(sig)
    }

    pub fn expand(&self) -> ExpandedSecretKey {
        let SecretKey(ref sk) = self;
        let mut ks = [0; EXPANDED_SECRET_LENGTH];

        unsafe {
            ffi::Hacl_Ed25519_expand_keys(
                ks.as_mut_ptr(),
                sk.as_ptr()
            );
        }

        ExpandedSecretKey(ks)
    }
}

impl PublicKey {
    pub fn verify(&self, msg: &[u8], sig: &Signature) -> bool {
        let PublicKey(ref pk) = self;
        let Signature(ref sig_ref) = sig;
        unsafe {
            ffi::Hacl_Ed25519_verify(
                pk.as_ptr(),
                msg.len() as u32,
                msg.as_ptr(),
                sig_ref.as_ptr()
            )
        }
    }
}
