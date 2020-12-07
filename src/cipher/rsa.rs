use num_bigint::{BigUint, RandBigInt};
use rand::prelude::*;
use std::ops::Range;

#[derive(Debug)]
pub struct RSAKey {
    n: BigUint,
    p: BigUint,
    s: Option<BigUint>,
}

impl RSAKey {
    pub fn public_key(n: BigUint, p: BigUint) -> Self {
        Self { n, p, s: None }
    }
    pub fn private_key(n: BigUint, p: BigUint, s: BigUint) -> Self {
        Self { n, p, s: Some(s) }
    }
    //pub fn key_gen_pq(p: BigUint, q: BigUint) -> Self {
    //let n = p * q;
    //let phi = (p - 1) * (q - 1);
    //let e = 1025;
    //let d = inv_mod(e, phi); //e^{-1} mod phi
    //Self {
    //n,
    //p: e,
    //s: Some(d),
    //}
    //}
    pub fn key_gen() -> Self {
        let mut rng = ThreadRng::default();
        let p = gen_prime(BigUint::from(100usize)..BigUint::from(1000usize), &mut rng);
        let q = gen_prime(BigUint::from(100usize)..BigUint::from(1000usize), &mut rng);
        let n = &p * &q;
        let phi = (p - 1usize) * (q - 1usize);
        let e = gen_rel_prime(BigUint::from(2usize)..&phi - 2usize, &phi, &mut rng);
        let d = inv_mod(e.clone(), phi); //e^{-1} mod phi
        Self {
            n,
            p: e,
            s: Some(d),
        }
    }
    pub fn encrypt(&self, m: BigUint) -> BigUint {
        m.modpow(&self.p, &self.n)
    }
    pub fn encrypt_sign(&self, m: BigUint) -> BigUint {
        m.modpow(
            self.s.as_ref().expect("Cannot decrypt w/o secret key"),
            &self.n,
        )
    }
    pub fn decrypt(&self, c: BigUint) -> BigUint {
        // c^s mod n
        c.modpow(
            self.s.as_ref().expect("Cannot decrypt w/o secret key"),
            &self.n,
        )
    }
    pub fn decrypt_sign(&self, c: BigUint) -> BigUint {
        c.modpow(&self.p, &self.n)
    }
}

fn gen_prime(size: Range<BigUint>, rng: &mut ThreadRng) -> BigUint {
    let mut num = BigUint::from(0usize);
    while !size.contains(&num) || !is_prime(num.clone()) {
        println!("*");
        num = rng.gen_biguint_range(&size.start, &size.end);
    }
    num
}

fn gen_rel_prime(size: Range<BigUint>, to: &BigUint, rng: &mut ThreadRng) -> BigUint {
    let mut num = BigUint::from(0usize);
    while !size.contains(&num) || !is_prime_to(num.clone(), to) {
        println!("+");
        num = rng.gen_biguint_range(&size.start, &size.end);
    }
    num
}

fn is_prime(num: BigUint) -> bool {
    // TODO: use a better prime test
    let mut i = num.sqrt();
    while i >= BigUint::from(2usize) {
        if num.clone() % &i == BigUint::from(0usize) {
            return false;
        }
        i -= 1usize;
    }
    true
}

fn is_prime_aks(num: BigUint) -> bool {
    // 1. if n = a**b for a > 1, b > 1, not prime
    for b in 2..=num.bits() + 1 {
        // TODO: check the remainder for this, not the value
        //let a = num.nth_root(b);
    }
    false
}

fn is_prime_to(num: BigUint, to: &BigUint) -> bool {
    let (mut max, mut min) = if &num < to {
        (to.clone(), num)
    } else {
        (num, to.clone())
    };
    while min != BigUint::from(0usize) {
        let r = max % &min;
        max = min;
        min = r;
    }
    max == BigUint::from(1usize)
}

//fn pow_mod(base: BigUint, exponent: BigUint, modulus: BigUint) -> BigUint {
//let mut total = base;
//for i in 1..exponent {
//total *= base;
//total %= modulus;
//}
//total
//}

fn inv_mod(num: BigUint, modulus: BigUint) -> BigUint {
    // TODO: use faster algo
    let mut i = BigUint::from(0usize);
    while i < modulus {
        if (i.clone() * num.clone()) % modulus.clone() == BigUint::from(1usize) {
            return i;
        }
        i += 1usize;
    }
    panic!("No inverse found")
}
