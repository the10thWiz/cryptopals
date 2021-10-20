mod aes;
pub mod diffie;
pub mod rsa;
pub mod stream;

use stream::{SeekableStreamCipher, StreamCipher};

use crate::data::Bytes;
use std::collections::LinkedList;

pub const BLOCK_SIZE: usize = 16;

pub fn aes_ecb_en(input: Bytes, key: Bytes) -> Bytes {
    if input.len() % 16 != 0 {
        panic!("Input is not padded correctly");
    }
    let mut output = Bytes::with_capacity(input.len());
    for part in input.split(16) {
        output += aes::aes_block_encrypt(part, key.clone());
    }
    output
}

pub fn aes_ecb_de(input: Bytes, key: Bytes) -> Bytes {
    if input.len() % 16 != 0 {
        panic!("Input is not padded correctly");
    }
    let mut output = Bytes::with_capacity(input.len());
    for part in input.split(16) {
        output += aes::aes_block_decrypt(part, key.clone());
    }
    output
}

/**
 * Encrypts data using AES CBC mode
 */
pub fn aes_cbc_en(input: Bytes, key: Bytes, iv: Bytes) -> Bytes {
    if input.len() % 16 != 0 {
        panic!("Input is not padded correctly");
    }
    let mut output = Bytes::with_capacity(input.len());
    let mut last = iv;
    for part in input.split(16) {
        last = aes::aes_block_encrypt(last ^ part, key.clone());
        output += last.clone();
    }
    output
}
/**
 * Decrypts data using AES CBC mode
 */
pub fn aes_cbc_de(input: Bytes, key: Bytes, iv: Bytes) -> Bytes {
    if input.len() % 16 != 0 {
        panic!("Input is not padded correctly");
    }
    let mut output = Bytes::with_capacity(input.len());
    let mut last = iv;
    for part in input.split(16) {
        output += last ^ aes::aes_block_decrypt(part.clone(), key.clone());
        last = part;
    }
    output
}

//pub fn aes_gcm_en(input: Bytes, key: Bytes, iv: Bytes) -> (Bytes, Bytes) {
    //let mut output = Bytes::with_capacity(input.len());
    //let h = derive_h(&key);
    //let mut auth_tag = mult_h(Bytes::zero(BLOCK_SIZE), h);
    //let mut counter = iv + Bytes::zero(BLOCK_SIZE / 2);
    //for block in input.split(BLOCK_SIZE) {
        //let tmp = aes::aes_block_encrypt(counter.clone(), key.clone()) ^ block;
        //auth_tag = mult_h(auth_tag ^ tmp, h);
        //output += tmp;
        //count.inc();
    //}
    //auth_tag = mult_h(auth_tag ^ todo!(), h);
    //(output, auth_tag)
//}

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
        let mut ret = Bytes::zero(0);
        for block in input.split(16) {
            ret += block ^ self.get_next();
        }
        ret
    }
    //pub fn crypt(&mut self, input: Bytes) -> Bytes {
    //let mut ret = Bytes::zero(input.len());
    //for i in 0..input.len() {
    //if self.current.is_empty() {
    //let data;
    //unsafe {
    //data = Bytes::from_bytes(&self.counter.input[..]);
    //self.counter.counters[1] += 1;
    //}
    //for b in aes_ecb_en(data, self.key.clone()).to_bytes() {
    //self.current.push_back(*b);
    //}
    //}
    //ret[i] = input[i] ^ self.current.pop_front().unwrap();
    //}
    //ret
    //}
}

impl StreamCipher for CTRstream {
    fn get_next(&mut self) -> Bytes {
        unsafe {
            let data = Bytes::from_bytes(&self.counter.input);
            self.counter.counters[1] += 1;
            aes_ecb_en(data, self.key.clone())
        }
    }
}

impl SeekableStreamCipher for CTRstream {
    fn get(&self, location: usize) -> (usize, Bytes) {
        let counter_val = location as u64 / BLOCK_SIZE as u64;
        unsafe {
            let cur_counter = RunningCounter {
                counters: [self.counter.counters[0], counter_val],
            };
            let data = Bytes::from_bytes(&cur_counter.input);
            (
                counter_val as usize * BLOCK_SIZE,
                aes_ecb_en(data, self.key.clone()),
            )
        }
    }
}
