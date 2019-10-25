use crate::data::Bytes;
use openssl::symm::{Cipher, Crypter, Mode};
use std::collections::LinkedList;

pub const BLOCK_SIZE: usize = 16;

/**
 * Encrypts data using AES ECB mode
 */
pub fn aes_ecb_en(input: Bytes, key: Bytes) -> Bytes {
    let mut encrypter =
        Crypter::new(Cipher::aes_128_ecb(), Mode::Encrypt, key.to_bytes(), None).unwrap();
    encrypter.pad(false);
    let mut ret = Bytes::zero(0);
    for i in 0..input.len() / BLOCK_SIZE {
        let mut output = [0u8; BLOCK_SIZE * 2];
        let len = encrypter
            .update(
                &input.to_bytes()[i * BLOCK_SIZE..i * BLOCK_SIZE + BLOCK_SIZE],
                &mut output,
            )
            .unwrap();
        ret += Bytes::from_bytes(&output).truncate(len);
    }
    let mut output = [0u8; BLOCK_SIZE * 2];
    let len = encrypter.finalize(&mut output).unwrap();
    ret += Bytes::from_bytes(&output).truncate(len);
    ret
}

/**
 * Decrypts data using AES ECB mode
 */
pub fn aes_ecb_de(input: Bytes, key: Bytes) -> Bytes {
    let mut decrypter =
        Crypter::new(Cipher::aes_128_ecb(), Mode::Decrypt, key.to_bytes(), None).unwrap();
    decrypter.pad(false);
    let mut ret = Bytes::zero(0);
    for i in 0..(input.len() as f64 / BLOCK_SIZE as f64).ceil() as usize {
        let mut output = [0u8; BLOCK_SIZE * 2];
        let len = decrypter
            .update(
                &input[i * BLOCK_SIZE..i * BLOCK_SIZE + BLOCK_SIZE],
                &mut output,
            )
            .unwrap();
        ret += Bytes::from_bytes(&output).truncate(len);
    }
    let mut output = [0u8; BLOCK_SIZE * 2];
    let len = decrypter.finalize(&mut output).unwrap();
    ret += Bytes::from_bytes(&output).truncate(len);
    ret
}

/**
 * Encrypts data using AES CBC mode
 */
pub fn aes_cbc_en(input: Bytes, key: Bytes, iv: Bytes) -> Bytes {
    let blocks = input
        .pad_pkcs7(BLOCK_SIZE - (input.len() % BLOCK_SIZE))
        .split(BLOCK_SIZE);
    let mut last = iv;
    let mut ret = Bytes::zero(0);
    for block in blocks {
        last = aes_ecb_en(last ^ block.clone(), key.clone());
        ret += last.clone();
    }
    ret
}

/**
 * Decrypts data using AES CBC mode
 */
pub fn aes_cbc_de(input: Bytes, key: Bytes, iv: Bytes) -> Bytes {
    let blocks = input.split(BLOCK_SIZE);
    let mut last = iv;
    let mut ret = Bytes::zero(0);
    for block in blocks {
        ret += last ^ aes_ecb_de(block.clone(), key.clone());
        last = block;
    }
    ret
}

union RunningCounter {
    counters: [u64; 2],
    input: [u8; 16],
}

pub struct CTRstream {
    counter: RunningCounter,
    key: Bytes,
    current: LinkedList<u8>,
}

impl CTRstream {
    pub fn new(nonce: u64, key: Bytes) -> Self {
        Self {
            counter: RunningCounter {
                counters: [nonce, 0u64],
            },
            key: key,
            current: LinkedList::new(),
        }
    }
    pub fn crypt(&mut self, input: Bytes) -> Bytes {
        let mut ret = Bytes::zero(input.len());
        for i in 0..input.len() {
            if self.current.is_empty() {
                let data;
                unsafe {
                    data = Bytes::from_bytes(&self.counter.input[..]);
                    self.counter.counters[1] += 1;
                }
                for b in aes_ecb_en(data, self.key.clone()).to_bytes() {
                    self.current.push_back(*b);
                }
            }
            ret[i] = input[i] ^ self.current.pop_front().unwrap();
        }
        ret
    }
}
