
use crate::data::Bytes;
use openssl::symm::{Cipher, Crypter, Mode};

#[allow(dead_code)]
pub fn encrypt_ecb(input: Bytes, key: Bytes) -> Bytes {
    let mut encrypter = Crypter::new(Cipher::aes_128_ecb(), Mode::Encrypt, key.to_bytes(), None).unwrap();
    encrypter.pad(false);
    let mut ret = Bytes::zero(0);
    for i in 0..input.size()/16 {
        let mut output = [0u8; 32];
        encrypter.update(&input.to_bytes()[i..i+16], &mut output).unwrap();
        ret+= Bytes::from_bytes(&output).truncate(16);
    }
    ret
}

#[allow(dead_code)]
pub fn decrypt_ecb(input: Bytes, key: Bytes) -> Bytes {
    let mut decrypter = Crypter::new(Cipher::aes_128_ecb(), Mode::Decrypt, key.to_bytes(), None).unwrap();
    decrypter.pad(false);
    let mut ret = Bytes::zero(0);
    for i in 0..(input.size() as f64/16.0).ceil() as usize {
        let mut output = [0u8; 32];
        let len = decrypter.update(&input.to_bytes()[i..i+16], &mut output).unwrap();
        ret+= Bytes::from_bytes(&output).truncate(len);
    }
    let mut output = [0u8; 32];
    let len = decrypter.finalize(&mut output).unwrap();
    ret+= Bytes::from_bytes(&output).truncate(len);
    ret
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
    let blocks = input.split(16);
    let mut last = iv;
    let mut ret = Bytes::zero(0);
    for block in blocks {
        ret+= last ^ decrypt_ecb(block.clone(), key.clone());
        last = block;
    }
    ret
}
