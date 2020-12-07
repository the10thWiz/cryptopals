use crate::cipher::stream::Stream;
use crate::cipher::*;
use crate::data::Bytes;
use crate::file::File;
use rand::prelude::*;

/**
 * Genaric Oracle trait, for oracle challenges
 */
pub trait Oracle {
    /**
     * Encrpyts provided data
     */
    fn encrypt(&self, input: Bytes) -> Bytes;
    /**
     * Decrypts encrypted data, and prints it to console
     *
     * (For debugging)
     */
    fn decrypt(&self, input: Bytes);
}

/**
 * Oracle for testing ECB/CBC mode detection
 */
pub fn encryption_oracle(input: Bytes) -> (Bytes, bool) {
    let mut rng = thread_rng();

    let plain = (Bytes::rand(rng.gen_range(5, 10)) + input + Bytes::rand(rng.gen_range(5, 10)))
        .pad_pkcs7(BLOCK_SIZE);
    if rng.gen() {
        (
            aes_cbc_en(plain, Bytes::rand(BLOCK_SIZE), Bytes::rand(BLOCK_SIZE)),
            true,
        )
    } else {
        (aes_ecb_en(plain, Bytes::rand(BLOCK_SIZE)), false)
    }
}

/**
 * Simple Oracle for 2.12
 */
pub struct OracleSimple {
    key: Bytes,
}
impl OracleSimple {
    pub fn new() -> Self {
        Self {
            key: Bytes::rand(BLOCK_SIZE),
        }
    }
}
impl Oracle for OracleSimple {
    fn encrypt(&self, input: Bytes) -> Bytes {
        aes_ecb_en(
            (input
                + Bytes::read_64(
                    "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXk\
    gaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IH\
    N0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK",
                ))
            .pad_pkcs7(BLOCK_SIZE),
            self.key.clone(),
        )
    }
    fn decrypt(&self, input: Bytes) {
        println!("{:16?}", aes_ecb_de(input, self.key.clone()));
    }
}

/**
 * CTR orable for 4.26
 */
pub struct CTROracle {
    key: Bytes,
    nonce: u64,
}
impl CTROracle {
    pub fn new() -> Self {
        Self {
            key: Bytes::rand(BLOCK_SIZE),
            nonce: rand::random(),
        }
    }
}
impl Oracle for CTROracle {
    fn encrypt(&self, input: Bytes) -> Bytes {
        CTRstream::new(self.nonce, self.key.clone()).crypt(
            input
                + Bytes::read_64(
                    "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXk\
    gaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IH\
    N0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK",
                ),
        )
    }
    fn decrypt(&self, input: Bytes) {
        println!("{:16?}", self.encrypt(input));
    }
}
/**
 * Profile Oracle for 4.26
 */
pub struct CTRProfileOracle {
    key: Bytes,
    nonce: u64,
}

impl CTRProfileOracle {
    pub fn new() -> Self {
        Self {
            key: Bytes::rand(BLOCK_SIZE),
            nonce: rand::random(),
        }
    }
    pub fn encode_profile(&self, email: Bytes) -> Bytes {
        CTRstream::new(self.nonce, self.key.clone()).crypt(self.create_profile(email))
    }
    pub fn create_profile(&self, email: Bytes) -> Bytes {
        Bytes::read_utf8("email=")
            + email.remove('&' as u8).remove('=' as u8)
            + Bytes::read_utf8("&uid=10&role=user")
    }
    pub fn get_role(&self, profile: Bytes) -> Role {
        for p in CTRstream::new(self.nonce, self.key.clone())
            .crypt(profile)
            .to_utf8()
            .split("&")
        {
            let kv: Vec<&str> = p.split("=").collect();
            if kv[0] == "role" {
                if kv[1] == "user" {
                    return Role::USER;
                } else if kv[1] == "admin" {
                    return Role::ADMIN;
                }
            }
        }
        Role::USER
    }
    pub fn print_raw(&self, profile: Bytes) {
        println!(
            "{}",
            CTRstream::new(self.nonce, self.key.clone()).crypt(profile)
        );
    }
}

/**
 * Profile Oracle for 2.13
 */
pub struct ProfileOracle {
    key: Bytes,
}

#[derive(Debug, PartialEq)]
pub enum Role {
    ADMIN,
    USER,
}

impl ProfileOracle {
    pub fn new() -> Self {
        ProfileOracle {
            key: Bytes::rand(BLOCK_SIZE),
        }
    }
    pub fn encode_profile(&self, email: Bytes) -> Bytes {
        aes_ecb_en(
            self.create_profile(email).pad_pkcs7(BLOCK_SIZE),
            self.key.clone(),
        )
    }
    pub fn create_profile(&self, email: Bytes) -> Bytes {
        Bytes::read_utf8("email=")
            + email.remove('&' as u8).remove('=' as u8)
            + Bytes::read_utf8("&uid=10&role=user")
    }
    pub fn get_role(&self, profile: Bytes) -> Role {
        for p in aes_ecb_de(profile, self.key.clone())
            .trim_pkcs7()
            .to_utf8()
            .split("&")
        {
            let kv: Vec<&str> = p.split("=").collect();
            if kv[0] == "role" {
                if kv[1] == "user" {
                    return Role::USER;
                } else if kv[1] == "admin" {
                    return Role::ADMIN;
                }
            }
        }
        Role::USER
    }
    pub fn print_raw(&self, profile: Bytes) {
        println!("{0:16?}{0:16X}", aes_ecb_de(profile, self.key.clone()));
    }
}

/**
 * Random Oracle for 2.14
 */
pub struct RandomOracle {
    key: Bytes,
    target: Bytes,
    prefix: Bytes,
}

impl RandomOracle {
    pub fn new() -> Self {
        RandomOracle {
            key: Bytes::rand(BLOCK_SIZE),
            target: Bytes::read_64(
                "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXk\
gaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IH\
N0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK",
            ),
            prefix: Bytes::rand(random::<u8>() as usize),
        }
    }
}
impl Oracle for RandomOracle {
    fn encrypt(&self, input: Bytes) -> Bytes {
        aes_ecb_en(
            (self.prefix.clone() + input + self.target.clone()).pad_pkcs7(BLOCK_SIZE),
            self.key.clone(),
        )
    }
    fn decrypt(&self, input: Bytes) {
        println!("{:16?}", aes_ecb_de(input, self.key.clone()));
    }
}

impl std::fmt::Display for RandomOracle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Prefix: {}, target: {}",
            self.prefix.len(),
            self.target.len()
        )
    }
}

/**
 * Profile Oracle (CBC mode) for 2.16
 */
pub struct ProfileCBCOracle {
    key: Bytes,
    iv: Bytes,
}

impl ProfileCBCOracle {
    pub fn new() -> Self {
        Self {
            key: Bytes::rand(BLOCK_SIZE),
            iv: Bytes::rand(BLOCK_SIZE),
        }
    }
    pub fn key_as_iv() -> Self {
        let iv = Bytes::rand(BLOCK_SIZE);
        Self {
            key: iv.clone(),
            iv,
        }
    }
    pub fn encode_profile(&self, email: Bytes) -> Bytes {
        aes_cbc_en(
            self.create_profile(email).pad_pkcs7(BLOCK_SIZE),
            self.key.clone(),
            self.iv.clone(),
        )
    }
    pub fn create_profile(&self, email: Bytes) -> Bytes {
        Bytes::read_utf8("comment1=cooking%20MCs;userdata=")
            + email.remove(';' as u8).remove('=' as u8)
            + Bytes::read_utf8(";comment2=%20like%20a%20pound%20of%20bacon")
    }
    pub fn get_role(&self, profile: Bytes) -> Result<Role, Bytes> {
        for p in aes_cbc_de(profile, self.key.clone(), self.iv.clone())
            .trim_pkcs7()
            .to_ascii()?
            .split(";")
        {
            let kv: Vec<&str> = p.split("=").collect();
            if kv[0] == "admin" {
                if kv[1] == "false" {
                    return Ok(Role::USER);
                } else if kv[1] == "true" {
                    return Ok(Role::ADMIN);
                }
            }
        }
        Ok(Role::USER)
    }
    pub fn get_raw_key(&self) -> &Bytes {
        &self.key
    }
    #[allow(dead_code)]
    pub fn print_raw(&self, profile: Bytes) {
        println!(
            "{0:16?}{0:16X}",
            aes_cbc_de(profile, self.key.clone(), self.iv.clone())
        );
    }
}

/**
 * Padding Oracle (CBC mode) for 2.17
 */
pub struct CBCPaddingOracle {
    key: Bytes,
}

fn get_rand() -> Bytes {
    let r = match thread_rng().gen_range(0, 10) {
        0 => Bytes::read_64("MDAwMDAwTm93IHRoYXQgdGhlIHBhcnR5IGlzIGp1bXBpbmc="),
        1 => Bytes::read_64(
            "MDAwMDAxV2l0aCB0aGUgYmFzcyBraWNrZWQgaW4gYW5kIHRoZSBWZWdhJ3MgYXJlIHB1bXBpbic=",
        ),
        2 => Bytes::read_64("MDAwMDAyUXVpY2sgdG8gdGhlIHBvaW50LCB0byB0aGUgcG9pbnQsIG5vIGZha2luZw=="),
        3 => Bytes::read_64("MDAwMDAzQ29va2luZyBNQydzIGxpa2UgYSBwb3VuZCBvZiBsiYWNvbg=="),
        4 => Bytes::read_64("MDAwMDA0QnVybmluZyAnZW0sIGlmIHlvdSBhaW4ndCBxdWljayBhbmQgbmltYmxl"),
        5 => Bytes::read_64("MDAwMDA1SSBnbyBjcmF6eSB3aGVuIEkgaGVhciBhIGN5bWJhbA=="),
        6 => Bytes::read_64("MDAwMDA2QW5kIGEgaGlnaCBoYXQgd2l0aCBhIHNvdXBlZCB1cCB0ZW1wbw=="),
        7 => Bytes::read_64("MDAwMDA3SSdtIG9uIGEgcm9sbCwgaXQncyB0aW1lIHRvIGdvIHNvbG8="),
        8 => Bytes::read_64("MDAwMDA4b2xsaW4nIGluIG15IGZpdmUgcG9pbnQgb2g="),
        9 => Bytes::read_64("MDAwMDA5aXRoIG15IHJhZy10b3AgZG93biBzbyBteSBoYWlyIGNhbiBibG93"),
        _ => panic!("The thread_rng() generated a value outside the given range"),
    };
    // println!("{:16X}", r);
    r
}

impl CBCPaddingOracle {
    pub fn new() -> Self {
        Self {
            key: Bytes::rand(BLOCK_SIZE),
        }
    }
    pub fn encrypt(&self) -> (Bytes, Bytes) {
        let iv = Bytes::rand(16);
        (iv.clone(), aes_cbc_en(get_rand(), self.key.clone(), iv))
    }
    pub fn check_padding(&self, enc: (Bytes, Bytes)) -> bool {
        let dec = aes_cbc_de(enc.1, self.key.clone(), enc.0);
        dec.trim_pkcs7().len() < dec.len()
    }
    pub fn print_raw(&self, enc: (Bytes, Bytes)) {
        println!(
            "{}",
            aes_cbc_de(enc.1, self.key.clone(), enc.0).trim_pkcs7()
        );
    }
}

pub fn gen_ctr_tests_3_19() -> Vec<Bytes> {
    let mut ret = Vec::new();
    let file = File::read_64_file("data_3_19");
    let key = Bytes::rand(16);
    for b in file {
        let mut ctr = Stream::new(CTRstream::new(0, key.clone()));
        ret.push(ctr.encrypt(&b));
    }
    ret
}

pub fn gen_ctr_tests_3_20() -> Vec<Bytes> {
    let mut ret = Vec::new();
    let file = File::read_64_file("data_3_20");
    let key = Bytes::rand(16);
    for b in file {
        let mut ctr = Stream::new(CTRstream::new(0, key.clone()));
        ret.push(ctr.encrypt(&b));
    }
    ret
}
