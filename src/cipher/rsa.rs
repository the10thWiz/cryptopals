use num_bigint::{BigUint, RandBigInt};
use rand::prelude::*;
use rayon::iter::{plumbing::UnindexedProducer, IntoParallelIterator, ParallelIterator};
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

#[derive(Debug, Clone)]
struct BigUintRange {
    cur: BigUint,
    end: BigUint,
}

impl BigUintRange {
    fn new(cur: BigUint, end: BigUint) -> Self {
        Self { cur, end }
    }
}

impl UnindexedProducer for BigUintRange {
    type Item = BigUint;

    fn split(self) -> (Self, Option<Self>) {
        let range = &self.end - &self.cur;
        if range <= BigUint::from(10u8) {
            (self, None)
        } else {
            let mid = range / 2u8 + &self.cur;
            (
                Self {
                    cur: self.cur,
                    end: &mid - 1u8,
                },
                Some(Self {
                    cur: mid,
                    end: self.end,
                }),
            )
        }
    }

    fn fold_with<F>(self, folder: F) -> F
    where
        F: rayon::iter::plumbing::Folder<Self::Item>,
    {
        folder.consume_iter(self)
    }
}

impl ParallelIterator for BigUintRange {
    type Item = BigUint;
    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item>,
    {
        rayon::iter::plumbing::bridge_unindexed(self, consumer)
    }
}

impl Iterator for BigUintRange {
    type Item = BigUint;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur > self.end {
            None
        } else {
            let ret = self.cur.clone();
            self.cur += 1u8;
            Some(ret)
        }
    }
}

enum TestResult {
    Unknown,
    Prime,
    Composite,
}

fn log2(n: &BigUint) -> u64 {
    n.bits() - 1
}

pub fn gcd(m: BigUint, n: &BigUint) -> BigUint {
    if m == 0u64.into() {
        n.clone()
    } else {
        gcd(n % &m, &m)
    }
}

#[test]
fn basic() {
    use std::str::FromStr;
    assert!(!is_prime_aks(&BigUint::from_str("4").unwrap()));
    assert!(is_prime_aks(&BigUint::from_str("5").unwrap()));
    assert!(is_prime_aks(&BigUint::from_str("7").unwrap()));
    assert!(is_prime_aks(&BigUint::from_str("31").unwrap()));
    assert!(is_prime_aks(&BigUint::from_str("101").unwrap()));
    assert!(is_prime_aks(&BigUint::from_str("47939").unwrap()));
    assert!(is_prime_aks(&BigUint::from_str("1652843").unwrap()));
    assert!(!is_prime_aks(&BigUint::from_str("4218962626").unwrap()));
    assert!(is_prime_aks(&BigUint::from_str("4218962623").unwrap()));
    assert!(is_prime_aks(
        &BigUint::from_str("288088873692602757671872933931").unwrap()
    ));
    let p = BigUint::from_str(
        "57896044618658097711785492504343953926634992332820282019728792003956564819949",
    )
    .unwrap();
    assert!(is_prime_aks(&p));
}

/// Checks whether the given number is prime, using the AKS test
pub fn is_prime_aks(n: &BigUint) -> bool {
    if n <= &BigUint::from(1u8) {
        return false;
    }
    println!("In test 1: {}", n);
    let top_limit = log2(n);

    let found_any_integer = (2..=top_limit).into_par_iter().any(|b| {
        let rounded = n.nth_root(b as u32);
        &rounded.pow(b as u32) == n
    });
    if !found_any_integer {
        test2(n)
    } else {
        false
    }
}

/// Calculate r, smallest r st. Or(n) > log2(n)^2
fn test2(n: &BigUint) -> bool {
    println!("In test 2");
    let maxk = BigUint::from(log2(n)).pow(2);
    let maxr = (BigUint::from(log2(n)).pow(5) + 1u8).max(BigUint::from(3u8));

    let krange = BigUintRange::new(BigUint::from(1u8), maxk);

    let final_r = BigUintRange::new(BigUint::from(2u8), &maxr - 1u8)
        .find_any(|r| !krange.clone().any(|k| n.modpow(&k, r) < BigUint::from(1u8)))
        .unwrap_or(maxr);
    test3(n, final_r)
}

/// check if 1 < gcd(a, n) < n for some a <= r => composite
fn test3(n: &BigUint, r: BigUint) -> bool {
    println!("In test 3");
    let found_any =
        BigUintRange::new(BigUint::from(1u8), r.clone()).any(|a| gcd(a, n) > BigUint::from(1u8));
    if found_any {
        false
    } else {
        if n <= &r {
            true
        } else {
            test5(n)
        }
    }
}

///
fn test5(n: &BigUint) -> bool {
    println!("In test 5: max {}", n / 2u8 - 1u8);
    let mut current_root = BigUint::from(1u8);
    let mut iter = BigUintRange::new(1u8.into(), n / 2u8 - 1u8);
    let has_divisible_coeffient = Iterator::any(&mut iter, |a| {
        //a.modpow(n, n) - a != BigUint::from(0u8)
        current_root *= n - &a + 1u8;
        current_root /= a;
        if &current_root % n != 0u8.into() {
            true
        } else {
            false
        }
    });
    //let has_divisible_coeffient = BigUintRange::new(1u8.into(), n / 2u8 - 1u8).any(|a| {
    //current_root *= n - &a + 1u8;
    //current_root /= a;
    //if &current_root % n != 0u8.into() {
    //true
    //} else {
    //false
    //}
    //});
    if has_divisible_coeffient {
        false
    } else {
        true
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

#[cfg(test)]
mod fermat {
    use std::str::FromStr;

    use num_bigint::BigUint;

    #[test]
    fn test_fermat() {
        assert_eq!(
            fermat_factorization(BigUint::from(195usize)),
            vec![BigUint::from(13usize), BigUint::from(15usize)],
        );
        println!("{:?}", fermat_factorization(BigUint::from_str("1234567891").unwrap()));
    }

    fn fermat_factorization(mut n: BigUint) -> Vec<BigUint> {
        while &n % 2u8 == BigUint::from(0u8) {
            n /= 2u8;
        }
        let mut x = n.sqrt();
        if &x * &x == n {
            vec![x.clone(), x]
        } else {
            loop {
                println!("iteration");
                x += 1u8;
                let y = &x * &x - &n;
                if y > x {
                    println!("Larger");
                    return vec![];
                }
                let (x1, x2) = (&x - &y, &x + y);
                if &x1 * &x2 == n {
                    return vec![x1, x2];
                }
            }
        }
    }
}
