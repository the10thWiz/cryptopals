
use crate::data::Bytes;
use openssl::symm::{Cipher, Crypter, Mode};

pub const BLOCK_SIZE : usize = 16;

/**
 * Encrypts data using AES ECB mode
 */
pub fn encrypt_ecb(input: Bytes, key: Bytes) -> Bytes {
    let mut encrypter = Crypter::new(Cipher::aes_128_ecb(), Mode::Encrypt, key.to_bytes(), None).unwrap();
    encrypter.pad(false);
    let mut ret = Bytes::zero(0);
    for i in 0..input.len()/BLOCK_SIZE {
        let mut output = [0u8; BLOCK_SIZE*2];
        let len = encrypter.update(&input.to_bytes()[i*BLOCK_SIZE..i*BLOCK_SIZE+BLOCK_SIZE], &mut output).unwrap();
        ret+= Bytes::from_bytes(&output).truncate(len);
    }
    let mut output = [0u8; BLOCK_SIZE*2];
    let len = encrypter.finalize(&mut output).unwrap();
    ret+= Bytes::from_bytes(&output).truncate(len);
    ret
}

/**
 * Decrypts data using AES ECB mode
 */
pub fn decrypt_ecb(input: Bytes, key: Bytes) -> Bytes {
    let mut decrypter = Crypter::new(Cipher::aes_128_ecb(), Mode::Decrypt, key.to_bytes(), None).unwrap();
    decrypter.pad(false);
    let mut ret = Bytes::zero(0);
    for i in 0..(input.len() as f64/BLOCK_SIZE as f64).ceil() as usize {
        let mut output = [0u8; BLOCK_SIZE*2];
        let len = decrypter.update(&input[i*BLOCK_SIZE..i*BLOCK_SIZE+BLOCK_SIZE], &mut output).unwrap();
        ret+= Bytes::from_bytes(&output).truncate(len);
    }
    let mut output = [0u8; BLOCK_SIZE*2];
    let len = decrypter.finalize(&mut output).unwrap();
    ret+= Bytes::from_bytes(&output).truncate(len);
    ret
}

/**
 * Encrypts data using AES CBC mode
 */
pub fn encrypt_cbc(input: Bytes, key: Bytes, iv: Bytes) -> Bytes {
    let blocks = input.pad_pkcs7(BLOCK_SIZE-(input.len()%BLOCK_SIZE)).split(BLOCK_SIZE);
    let mut last = iv;
    let mut ret = Bytes::zero(0);
    for block in blocks {
        last = encrypt_ecb(last ^ block.clone(), key.clone());
        ret+= last.clone();
    }
    ret
}

/**
 * Decrypts data using AES CBC mode
 */
pub fn decrypt_cbc(input: Bytes, key: Bytes, iv: Bytes) -> Bytes {
    let blocks = input.split(BLOCK_SIZE);
    let mut last = iv;
    let mut ret = Bytes::zero(0);
    for block in blocks {
        ret+= last ^ decrypt_ecb(block.clone(), key.clone());
        last = block;
    }
    ret
}
