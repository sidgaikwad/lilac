use std::sync::OnceLock;

use jsonwebtoken::{DecodingKey, EncodingKey};

pub static KEYS: OnceLock<Keys> = OnceLock::new();

pub struct Keys {
    pub(crate) encoding: EncodingKey,
    pub(crate) decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
