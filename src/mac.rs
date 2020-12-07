//
// digest.rs
// Copyright (C) 2020 matt <matt@mattlaptop>
// Distributed under terms of the MIT license.
//

use crate::data::Bytes;
use md4::Digest;
use sha::sha1::Sha1;
use sha::utils::DigestExt;
use std::io::Write;

enum Algo {
    Sha1,
    Md4,
}

pub struct SecrectDigest {
    key: Bytes,
    algo: Algo,
}

impl SecrectDigest {
    pub fn sha1() -> Self {
        Self {
            key: Bytes::rand(16),
            algo: Algo::Sha1,
        }
    }
    pub fn md4() -> Self {
        Self {
            key: Bytes::rand(0),
            algo: Algo::Md4,
        }
    }
    pub fn len(&self) -> usize {
        self.key.len()
    }
    pub fn sign(&self, message: &Bytes) -> Bytes {
        match self.algo {
            Algo::Sha1 => {
                let mut sha1 = Sha1::default();
                sha1.write_all(self.key.to_bytes()).unwrap();
                sha1.write_all(message.to_bytes()).unwrap();
                sha1.flush().unwrap();
                Bytes::from_vec(sha1.to_bytes())
            }
            Algo::Md4 => {
                let mut md4 = md4::Md4::new();
                md4.update(self.key.to_bytes());
                md4.update(message.to_bytes());
                unsafe {
                    let mut md4: md4::Md4 = std::mem::transmute_copy(&md4);
                    let mut gen_arr: () = Default::default();
                    use digest::FixedOutputDirty;
                    //md4.finalize_into_dirty(&mut gen_arr);
                    let raw: [u32; 24] = std::mem::transmute(md4);
                    println!("{:X} => {:X?}", message, &raw[20..]);
                    println!("len: {:X?}", &raw[0..2]);
                }
                Bytes::from_bytes(&md4.finalize()[..])
            }
        }
    }
    pub fn verify(&self, message: &Bytes, mac: &Bytes) -> bool {
        self.sign(message) == mac
    }
}
