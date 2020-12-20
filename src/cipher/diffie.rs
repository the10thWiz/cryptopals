//
// diffie.rs
// Copyright (C) 2020 matt <matt@mattlaptop>
// Distributed under terms of the MIT license.
//

//! Diffie-Hellman algo:
//! ```
//! let (a, A) = diffie_helman_weak_a();
//! let B = ();// send A and get B (the other side's A)
//! let shared_key = diffie_helman_weak_key(a, B);
//! ```
use lazy_static::lazy_static;

use num_bigint::BigUint;
use num_bigint::{RandBigInt, ToBigUint};

lazy_static! {
    static ref WEAK_P: BigUint = 37usize.to_biguint().unwrap();
    static ref WEAK_G: BigUint = 5usize.to_biguint().unwrap();
    static ref NIST_P: BigUint = BigUint::parse_bytes(b"ffffffffffffffffc90fdaa22168c234c4c6628b80dc1cd129024e088a67cc74020bbea63b139b22514a08798e3404ddef9519b3cd3a431b302b0a6df25f14374fe1356d6d51c245e485b576625e7ec6f44c42e9a637ed6b0bff5cb6f406b7edee386bfb5a899fa5ae9f24117c4b1fe649286651ece45b3dc2007cb8a163bf0598da48361c55d39a69163fa8fd24cf5f83655d23dca3ad961c62f356208552bb9ed529077096966d670c354e4abc9804f1746c08ca237327ffffffffffffffff", 16).unwrap();
    static ref NIST_G: BigUint = 2usize.to_biguint().unwrap();
}

/// Generates a and A
///
/// returns (a, A)
pub fn diffie_hellman_weak_a() -> (BigUint, BigUint) {
    let mut rng = rand::thread_rng();
    let a = rng.gen_biguint(10) % &*WEAK_P;
    let A = WEAK_G.modpow(&a, &*WEAK_P);
    (a, A)
}
pub fn diffie_hellman_weak_key(a: BigUint, B: BigUint) -> BigUint {
    B.modpow(&a, &*WEAK_P)
}

/// Generates a and A
///
/// returns (a, A)
pub fn diffie_hellman_a() -> (BigUint, BigUint) {
    let mut rng = rand::thread_rng();
    let a = rng.gen_biguint(10) % &*NIST_P;
    let A = WEAK_G.modpow(&a, &*NIST_P);
    (a, A)
}
pub fn diffie_hellman_key(a: BigUint, B: BigUint) -> BigUint {
    B.modpow(&a, &*NIST_P)
}
