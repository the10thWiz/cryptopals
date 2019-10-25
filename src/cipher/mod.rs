
mod aes;
use crate::data::Bytes;
use std::collections::LinkedList;

pub const BLOCK_SIZE: usize = 16;

pub fn aes_ecb_en(input: Bytes, key: Bytes) -> Bytes {
    if input.len()%16 != 0 {
        panic!("Input is not padded correctly");
    }
    let mut output = Bytes::with_capacity(input.len());
    for part in input.split(16) {
        output+= aes::aes_block_encrypt(part, key.clone());
    }
    output
}

pub fn aes_ecb_de(input: Bytes, key: Bytes) -> Bytes {
    if input.len()%16 != 0 {
        panic!("Input is not padded correctly");
    }
    let mut output = Bytes::with_capacity(input.len());
    for part in input.split(16) {
        output+= aes::aes_block_decrypt(part, key.clone());
    }
    output
}

/**
 * Encrypts data using AES CBC mode
 */
pub fn aes_cbc_en(input: Bytes, key: Bytes, iv: Bytes) -> Bytes {
    if input.len()%16 != 0 {
        panic!("Input is not padded correctly");
    }
    let mut output = Bytes::with_capacity(input.len());
    let mut last = iv;
    for part in input.split(16) {
        last = aes::aes_block_encrypt(last ^ part, key.clone());
        output+= last.clone();
    }
    output
}
/**
 * Decrypts data using AES CBC mode
 */
pub fn aes_cbc_de(input: Bytes, key: Bytes, iv: Bytes) -> Bytes {
    if input.len()%16 != 0 {
        panic!("Input is not padded correctly");
    }
    let mut output = Bytes::with_capacity(input.len());
    let mut last = iv;
    for part in input.split(16) {
        output+= last ^ aes::aes_block_encrypt(part.clone(), key.clone());
        last = part;
    }
    output
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