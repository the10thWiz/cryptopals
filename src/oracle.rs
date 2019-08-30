
use crate::data::Bytes;
use crate::open_ssl::*;

pub struct Oracle {
    key : Bytes
}

impl Oracle {
    pub fn new() -> Oracle {
        Oracle {key: Bytes::rand(BLOCK_SIZE)}
    }
    pub fn encrypt_ecb(&self, input : Bytes) -> Bytes {
        encrypt_ecb(input.pad_pkcs7(BLOCK_SIZE), self.key.clone())
    }
    pub fn decrypt_ecb(&self, input : Bytes) -> Bytes {
        decrypt_ecb(input, self.key.clone())
    }
}
