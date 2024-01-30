//
// fermat.rs
// Copyright (C) 2022 matthew <matthew@WINDOWS-05HIC4F>
// Distributed under terms of the MIT license.
//

// Fermat test
// if n in prime, then
//  a**(n-1) = 1 (mod n)
//
// Therefore, we could check every a to verify n is prime.
// For speed, we can check a subset of possible a, but we only get a probablity
//  should be select a fully at random?
//
// # Miller-Rabin
// Given an odd n, n - 1 = 2^s * d where d is odd => divide n - 1 by two until d is odd
// If n is prime, the either
// - a^d = 1 mod n
// - a^((2^r)d) = -1 (mod n) for some { r | 0<=r<=s-1 }
//
// Given a random a, composite numbers will fail 75% of the time
//
// if p is prime, then x^2 = 1 (mod p), only has x = { 1, -1 }
// if p is prime and gcd(a, p) = 1, then a^(p-1) = 1(mod p)
//  p-1 = 2^s * d => a^(2^s * d) = 1 (mod p)
//  => (a^(2^(s-1) * d))^2 = { -1, 1 } (mod p)
//
// # Miller test (deterministic version), depends on the reinman hypothesis
// run a from 2 <= a <= ln(p)^2

use num_bigint::BigUint;

pub fn is_even(p: BigUint) -> bool {
    if let Some(least_sig_digit) = p.iter_u32_digits().next() {
        if least_sig_digit % 2 == 0 {
            true
        } else {
            false
        }
    } else {
        true
    }
}

pub fn miller_rabin_prime_test(p: BigUint) -> bool {
    // Find d
    let mut s = BigUint::from(1usize);
    let mut tmp = &p / 2usize;
    let d = loop {

    };
    todo!()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
	}
}
