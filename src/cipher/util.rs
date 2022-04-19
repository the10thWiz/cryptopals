use std::str::FromStr;

use num_bigint::{BigInt, BigUint};
use num_traits::sign::Signed;
use num_traits::ToPrimitive;

pub fn fast_exp(
    base: impl Into<BigUint>,
    exp: impl Into<BigUint>,
    modulus: impl Into<BigUint>,
) -> BigUint {
    let mut base = base.into();
    let mut exp = exp.into();
    let modulus = modulus.into();
    let mut product = BigUint::from(1usize);
    while exp > BigUint::from(0usize) {
        if exp.bit(0) {
            product = (product * &base) % &modulus;
        }
        base = base.pow(2) % &modulus;
        exp = exp >> 1;
    }
    product
}

pub fn extended_euclid(m: impl Into<BigInt>, n: impl Into<BigInt>) -> (BigInt, BigInt, BigInt) {
    let m = m.into();
    let n = n.into();
    if n == BigInt::from(0u64) {
        (1u64.into(), 0u64.into(), m)
    } else {
        let (n0, m0) = (m.clone() % &n, n.clone());
        let (x0, y0, gcd) = extended_euclid(m0, n0);
        let x1 = x0 - (m / &n * &y0);
        (y0, x1, gcd)
    }
}

pub fn gcd(m: impl Into<BigInt>, n: impl Into<BigInt>) -> BigInt {
    extended_euclid(m, n).2
}

/// a * b = res mod modulus
/// returns b
fn solve_mod_inv(
    a: impl Into<BigUint>,
    res: impl Into<BigUint>,
    modulus: impl Into<BigUint>,
) -> BigUint {
    todo!()
}

fn inv_mod(x: BigInt, n: BigInt) -> BigInt {
    todo!()
}

pub fn crt(rems: &[BigInt], moduli: &[BigInt]) -> (BigInt, BigInt) {
    if rems.len() == 1 {
        (rems[0].clone() % &moduli[0], moduli[0].clone())
    } else {
        let (r, m) = crt(&rems[1..], &moduli[1..]);
        let (u, v, gcd) = extended_euclid(m.clone(), moduli[0].clone());
        let rem = (u * &rems[0] * &m + &r * &moduli[0] * &v) % (&moduli[0] * &m);
        let new_mod = m * &moduli[0];
        (rem, new_mod)
    }
}
/// Finds prime factors of n, faster than brute force
pub fn pollard_rho(n: BigInt) -> (BigInt, BigInt) {
    let h = |x: BigInt| x.pow(2) + BigInt::from(1u64);
    let mut x = 2u64.into();
    let mut y = 2u64.into();
    let mut g = 1u64.into();
    let mut count = 1u64.into();
    while g == 1u64.into() {
        x = h(x) % &n;
        y = h(h(y)) % &n;
        g = gcd((&x - &y).abs(), n.clone());
        count += 1u32;
    }
    (g, count)
}

fn main() {
    //println!("{}", fast_exp(5u64, 177u64, 11u64));
    //println!("{}", fast_exp(
    //BigUint::from_str("1245723409857329408572309485723094758").unwrap(),
    //BigUint::from_str("1245723409857329408572309485723094719024873558").unwrap(),
    //BigUint::from_str("12457234098573294085723094857231903487503294875320984572309485723094857094758").unwrap(),
    //));
    //let (x, y, gcd) = extended_euclid(119u64, 175u64);
    //println!("({}, {}, {})", x, y, gcd);
    //let mut n = BigInt::from_str("720777718430892426412699013841273041436894957841223").unwrap();
    //while n > 0u64.into() {
    //let tmp = pollard_rho(n.clone());
    //println!("{:?}", tmp);
    //n = n / tmp.0;
    //}
    //break_different_n();
}

// Alice is sending
// Bob's pubic key is     (3, 8876044532898802067 ) and he receives the message: 8187081806215505471
// Robert's public key is (3, 18801105946394093459) and he receives the message: 5285356194805239972
// Bobby's public key is  (3, 39199007823998693533) and he receives the message: 6399213745575232470
//println!("{}", break_common_message(3u64.into(), &[
//BigInt::from_str("8876044532898802067").unwrap(),
//BigInt::from_str("18801105946394093459").unwrap(),
//BigInt::from_str("39199007823998693533").unwrap(),
//], &[
//BigInt::from_str("8187081806215505471").unwrap(),
//BigInt::from_str("5285356194805239972").unwrap(),
//BigInt::from_str("6399213745575232470").unwrap(),
//]));

fn break_common_message(exp: BigInt, keys: &[BigInt], enc: &[BigInt]) -> BigInt {
    // Alice:
    // m^3 % n1
    // m^3 % n2
    // m^3 % n3
    //
    // Use CRT to find message from the three actual messages
    //     m < [n1, n2, n3]
    //     m^3 < n1 * n2 * n3
    //     crt(&[m1, m2, m3], &[n1, n2, n3])
    let (m_3, _mod) = crt(enc, keys);
    m_3.cbrt()
}

fn break_different_n() {
    // Eve is able to corrupt Bob's public key in transmission so that Alice encrypts her message with
    // with the public key (65540, 2169657800746705868906997566280821497741) instead of
    // (65537, 2169657800746705868906997566280821497741). Note that only the encryption exponent has
    // changed. Bob receives the message: 1147836106621857253994178866934024626264 but is unable to get
    // anything meaningful from the decrypted message. He resends his public key to Alice who correctly
    // encrypts using the exponent 65537. Bob receives the message:
    // 1622980102927711263837825272257800131879 which provides him with the vital information he was
    // hoping for.
    //
    // How can Eve determine the plaintext? What was the plaintext?
    //
    // m^e1 % n = m1 = 1147836106621857253994178866934024626264
    // m^e2 % n = m2 = 1622980102927711263837825272257800131879
    //              gcd(e1, e2) = 1
    // there exists e1 x + e2 y = 1
    let e1 = BigInt::from_str("65540").unwrap();
    let e2 = BigInt::from_str("65537").unwrap();
    let n = BigInt::from_str("2169657800746705868906997566280821497741").unwrap();
    let m1 = BigInt::from_str("1147836106621857253994178866934024626264").unwrap();
    let m2 = BigInt::from_str("1622980102927711263837825272257800131879").unwrap();
    let m2_inv = inv_mod(m2, n.clone());
    //let m2_inv = (-m2) % &n;
    let (x, y, _gcd) = extended_euclid(e1, e2);
    let tmp = m1.pow(x.to_u32().unwrap()) * m2_inv.pow(y.abs().to_u32().unwrap()) % n;
    println!("{}", tmp);
}
