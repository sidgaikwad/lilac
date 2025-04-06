use std::env;

use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;

pub(crate) static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = env::var("SECRET_KEY").expect("secret key to be set");
    Keys::new(secret.as_bytes())
});

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
