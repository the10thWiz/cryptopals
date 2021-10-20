use std::{collections::HashMap, hash::Hasher};

use crate::data::Bytes;
use sha::sha256;

//
// passwd.rs
// Copyright (C) 2021 matthew <matthew@WINDOWS-05HIC4F>
// Distributed under terms of the MIT license.
//

#[derive(Debug)]
pub struct Passwd {
    pub salt: Bytes,
    pub hash: Bytes,
}

impl Passwd {
    fn new(pepper: &Bytes, passwd: impl AsRef<str>) -> Self {
        let salt = Bytes::rand(16);
        let mut hasher = sha256::Sha256::default();
        //hasher.write(pepper);
        hasher.write(&salt);
        hasher.write(passwd.as_ref().as_bytes());
        Self {
            salt,
            hash: Bytes::from_bytes(&hasher.finish().to_be_bytes()),
        }
    }

    fn verify(&self, pepper: &Bytes, passwd: impl AsRef<str>) -> bool {
        let mut hasher = sha256::Sha256::default();
        //hasher.write(pepper);
        hasher.write(&self.salt);
        hasher.write(passwd.as_ref().as_bytes());
        &hasher.finish().to_be_bytes() == self.hash.to_bytes()
    }
}

#[derive(Debug)]
pub struct PasswdStore {
    pub pepper: Bytes,
    pub db: HashMap<String, Passwd>,
}

impl PasswdStore {
    pub fn new() -> Self {
        Self {
            pepper: Bytes::rand(16),
            db: HashMap::default(),
        }
    }

    pub fn add_user(&mut self, username: impl Into<String>, passwd: impl AsRef<str>) {
        self.db
            .insert(username.into(), Passwd::new(&self.pepper, passwd));
    }

    pub fn verify(&mut self, username: impl AsRef<str>, pass: impl AsRef<str>) -> bool {
        if let Some(passwd) = self.db.get(username.as_ref()) {
            passwd.verify(&self.pepper, pass)
        } else {
            false
        }
    }
}
