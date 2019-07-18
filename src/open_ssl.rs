
use crate::data::Bytes;
use openssl::symm::{encrypt, decrypt, Cipher};

#[allow(dead_code)]
pub fn encrypt_ecb(input: Bytes, key: Bytes) -> Bytes {
    let cipher = Cipher::aes_128_ecb();
    let key = b"YELLOW SUBMARINE";
    Bytes::from_vec(encrypt(cipher, key, None, input.to_bytes()).unwrap())
}

#[allow(dead_code)]
pub fn decrypt_ecb(input: Bytes, key: Bytes) -> Bytes {
    let cipher = Cipher::aes_128_ecb();
    Bytes::from_vec(decrypt(cipher, key.to_bytes(), None, input.to_bytes()).unwrap())
}

#[allow(dead_code)]
pub fn encrypt_cbc(input: Bytes, key: Bytes, iv: Bytes) -> Bytes {
    let blocks = input.pad_pkcs7(16-(input.size()%16)).split(16);
    let mut last = iv;
    let mut ret = Bytes::zero(0);
    for block in blocks {
        ret+= encrypt_ecb(last ^ block.clone(), key.clone());
        last = block;
    }
    ret
}

#[allow(dead_code)]
pub fn decrypt_cbc(input: Bytes, key: Bytes, iv: Bytes) -> Bytes {
    let blocks = input.pad_pkcs7(16-(input.size()%16)).split(16);
    let mut last = iv;
    let mut ret = Bytes::zero(0);
    for block in blocks {
        ret+= last ^ encrypt_ecb(block.clone(), key.clone());
        last = block;
    }
    ret
}
