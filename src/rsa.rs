use rand::prelude::*;
use std::ops::Range;

#[derive(Debug)]
pub struct RSAKey {
    n: usize,
    p: usize,
    s: Option<usize>,
}

impl RSAKey {
    pub fn public_key(n: usize, p: usize) -> Self {
        Self { n, p, s: None }
    }
    pub fn private_key(n: usize, p: usize, s: usize) -> Self {
        Self { n, p, s: Some(s) }
    }
    pub fn key_gen_pq(p: usize, q: usize) -> Self {
        let n = p * q;
        let phi = (p - 1) * (q - 1);
        let e = 1025;
        let d = inv_mod(e, phi); //e^{-1} mod phi
        Self {
            n,
            p: e,
            s: Some(d),
        }
    }
    pub fn key_gen() -> Self {
        let mut rng = ThreadRng::default();
        let p = gen_prime(100..1000, &mut rng);
        let q = gen_prime(100..1000, &mut rng);
        let n = p * q;
        let phi = (p - 1) * (q - 1);
        let e = gen_rel_prime(2..phi-2, phi, &mut rng);
        let d = inv_mod(e, phi); //e^{-1} mod phi
        Self {
            n,
            p: e,
            s: Some(d),
        }
    }
    pub fn encrypt(&self, m: usize) -> usize {
        pow_mod(m, self.p, self.n)
    }
    pub fn encrypt_sign(&self, m: usize) -> usize {
        pow_mod(m, self.s.expect("Cannot decrypt w/o secret key"), self.n)
    }
    pub fn decrypt(&self, c: usize) -> usize {
        // c^s mod n
        pow_mod(c, self.s.expect("Cannot decrypt w/o secret key"), self.n)
    }
    pub fn decrypt_sign(&self, c: usize) -> usize {
        pow_mod(c, self.p, self.n)
    }
}

fn gen_prime(size: Range<usize>, rng: &mut ThreadRng) -> usize {
    let mut num = 0;
    while !size.contains(&num) || !is_prime(num) {
        println!("*");
        num = rng.next_u64() as usize % (size.end - size.start) + size.start;
    }
    num
}
fn gen_rel_prime(size: Range<usize>, to: usize, rng: &mut ThreadRng) -> usize {
    let mut num = 0;
    while !size.contains(&num) || !is_prime_to(num, to) {
        println!("+");
        num = rng.next_u64() as usize % (size.end - size.start) + size.start;
    }
    num
}

fn is_prime(num: usize) -> bool {
    for i in 2..(num as f64).sqrt() as usize {
        if num % i == 0 {
            return false;
        }
    }
    true
}
fn is_prime_to(num: usize, to: usize) -> bool {
    let (mut max, mut min) = if num < to { (to, num) } else { (num, to) };
    while min != 0 {
        let r = max % min;
        max = min;
        min = r;
    }
    max == 1
}

fn pow_mod(base: usize, exponent: usize, modulus: usize) -> usize {
    let mut total = base;
    for i in 1..exponent {
        total *= base;
        total %= modulus;
    }
    total
}
fn inv_mod(num: usize, modulus: usize) -> usize {
    for i in 0..modulus {
        if (i * num) % modulus == 1 {
            return i;
        }
    }
    panic!("No inverse found")
}
