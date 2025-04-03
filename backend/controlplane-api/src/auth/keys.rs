use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;
use rand::distr::{Alphanumeric, SampleString};

pub(crate) static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = Alphanumeric.sample_string(&mut rand::rng(), 60);
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
