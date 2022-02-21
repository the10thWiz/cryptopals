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
    /// Create public key variant
    pub fn public_key(n: BigUint, p: BigUint) -> Self {
        Self { n, p, s: None }
    }

    /// Create private key variant
    pub fn private_key(n: BigUint, p: BigUint, s: BigUint) -> Self {
        Self { n, p, s: Some(s) }
    }

    /// Generates a keypair
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

    /// Encrypts the provided integer
    ///
    /// Can then be decrypted with `decrypt`
    pub fn encrypt(&self, m: BigUint) -> BigUint {
        m.modpow(&self.p, &self.n)
    }

    /// Encrypts the provided integer as a signature
    ///
    /// Requires the private key
    /// Can be decrypted with `decrypt_sign`
    pub fn encrypt_sign(&self, m: BigUint) -> BigUint {
        m.modpow(
            self.s.as_ref().expect("Cannot decrypt w/o secret key"),
            &self.n,
        )
    }

    /// Decrypts the provided integer
    ///
    /// Requires the private key
    pub fn decrypt(&self, c: BigUint) -> BigUint {
        // c^s mod n
        c.modpow(
            self.s.as_ref().expect("Cannot decrypt w/o secret key"),
            &self.n,
        )
    }

    /// Decrypts the provided integer as a signature
    ///
    /// Must have been encrypted with `decrypt_sign`
    pub fn decrypt_sign(&self, c: BigUint) -> BigUint {
        c.modpow(&self.p, &self.n)
    }

    /// Clones self into a public key variant of itself
    pub fn public_key_part(&self) -> Self {
        Self {
            n: self.n.clone(),
            p: self.p.clone(),
            s: None,
        }
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

/// Checks if num is a perfect power
///
/// Also doesn't work
fn perfect_power(num: &BigUint) -> bool {
    /// All primes up to a limit - 2048
    ///
    /// Note that this limit is the log base 2 of the maximum size of integer for is_prime_aks
    const EARLY_PRIMES: &[u64] = &[
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
        283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397,
        401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503,
        509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619,
        631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743,
        751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863,
        877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997,
        1009, 1013, 1019, 1021, 1031, 1033, 1039, 1049, 1051, 1061, 1063, 1069, 1087, 1091, 1093,
        1097, 1103, 1109, 1117, 1123, 1129, 1151, 1153, 1163, 1171, 1181, 1187, 1193, 1201, 1213,
        1217, 1223, 1229, 1231, 1237, 1249, 1259, 1277, 1279, 1283, 1289, 1291, 1297, 1301, 1303,
        1307, 1319, 1321, 1327, 1361, 1367, 1373, 1381, 1399, 1409, 1423, 1427, 1429, 1433, 1439,
        1447, 1451, 1453, 1459, 1471, 1481, 1483, 1487, 1489, 1493, 1499, 1511, 1523, 1531, 1543,
        1549, 1553, 1559, 1567, 1571, 1579, 1583, 1597, 1601, 1607, 1609, 1613, 1619, 1621, 1627,
        1637, 1657, 1663, 1667, 1669, 1693, 1697, 1699, 1709, 1721, 1723, 1733, 1741, 1747, 1753,
        1759, 1777, 1783, 1787, 1789, 1801, 1811, 1823, 1831, 1847, 1861, 1867, 1871, 1873, 1877,
        1879, 1889, 1901, 1907, 1913, 1931, 1933, 1949, 1951, 1973, 1979, 1987, 1993, 1997, 1999,
        2003, 2011, 2017, 2027, 2029, 2039,
    ];

    assert!(
        num.bits() <= 2028,
        "Only works on numbers smaller than 2^2048"
    );
    for prime in EARLY_PRIMES {
        let prime = BigUint::from(*prime);
        let mut power = prime.clone();
        while &power < num {
            power *= &prime;
        }
        if &power == num {
            return true;
        }
    }
    false
}

/// Checks whether the given number is prime, using the AKS test
fn is_prime_aks(num: BigUint) -> bool {
    // 1. if n = a**b for a > 1, b > 1, not prime
    if perfect_power(&num) {
        return false;
    }
    false
}

mod ask_test {
    use num_bigint::BigUint;
    use rayon::iter::{IntoParallelIterator, ParallelIterator};

    enum TestResult {
        Unknown,
        Prime,
        Composite,
    }

    fn log2(n: &BigUint) -> u64 {
        n.bits() - 1
    }

    fn test1(n: &BigUint) -> TestResult {
        let n_as_float = ();
        let top_limit = log2(n);

        let found_any_integer = (2..=top_limit).into_par_iter().any(|b| {
                //let rounded = n.nth_root(b);
                //rounded
                todo!()
            });
        todo!()
    }
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
