//
// eliptic_curve.rs
// Copyright (C) 2022 matthew <matthew@WINDOWS-05HIC4F>
// Distributed under terms of the MIT license.
//

use std::{str::FromStr, fmt::Debug};

use lazy_static::lazy_static;
use num_bigint::{BigInt, BigUint};

use super::modulus::{ModN, PrimeMod, N_101};

lazy_static! {
    pub static ref CURVE_25519: ElipticCurve = ElipticCurve {
        base_point: BigUint::from_str("9").unwrap(),
        prime: BigUint::from_str(
            "57896044618658097711785492504343953926634992332820282019728792003956564819949"
        )
        .unwrap(),
    };
}

// y^2 = x^3 + ax + b
// Add points on the curve as our operation
// - Given points p, q
// - The line defined by p and q intersects at a third point. We then reflect that point across the
// x axis to get the third point.
// - The additive inverse of p is just reflected across the x axis
// - for p + p we use the tangent at p as the line.
// - repeated doubling of a point is chaotic
// - Also has the point at infinity (which we use as zero)
//
// For the eliptic curve, we need a curve C, and a generator G
// private key is k, public key is k * G. We do multiplication as repreated doubling

pub enum Point<M: PrimeMod> {
    Pt(ModN<M>, ModN<M>),
    Inf,
}

impl<M: PrimeMod> Debug for Point<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pt(x, y) => write!(f, "({:?}, {:?})", x, y),
            Self::Inf => write!(f, "(inf)"),
        }
    }
}

impl<M: PrimeMod> Clone for Point<M> {
    fn clone(&self) -> Self {
        match self {
            Self::Pt(x, y) => Self::Pt(x.clone(), y.clone()),
            Self::Inf => Self::Inf,
        }
    }
}

pub struct WeienerstrassCurve<M: PrimeMod> {
    a: ModN<M>,
    b: ModN<M>,
}

impl<M: PrimeMod> WeienerstrassCurve<M> {
    pub fn add(&self, p: &Point<M>, q: &Point<M>) -> Point<M> {
        match (p, q) {
            (Point::Pt(px, py), Point::Pt(qx, qy)) => {
                if px == qx {
                    if py == qy && py != &ModN::new(0u64) {
                        let s = (ModN::new(3u64) * px.clone().pow(2u64.into()) + self.a.clone())
                            / (ModN::new(2u64) * py);
                        let rx = s.clone().pow(2u64.into()) - px - qx;
                        let ry = s * (&rx - px) + py;
                        Point::Pt(rx, -ry)
                    } else {
                        Point::Inf
                    }
                } else {
                    let s = (qy - py) / (qx - px);
                    let rx = s.clone().pow(2u64.into()) - px - qx;
                    let ry = s * (&rx - px) + py;
                    Point::Pt(rx, -ry)
                }
            }
            (Point::Inf, q) | (q, Point::Inf) => q.clone(),
        }
    }

    /// Shortcut for add(p, p). Note that this is marginally faster, since it shortcuts the initial
    /// testing.
    pub fn double(&self, p: &Point<M>) -> Point<M> {
        match p {
            Point::Inf => Point::Inf,
            Point::Pt(px, py) => {
                if py == &ModN::new(0u64) {
                    Point::Inf
                } else {
                    let s = (ModN::new(3u64) * px.clone().pow(2u64.into()) + self.a.clone())
                        / (ModN::new(2u64) * py);
                    let rx = s.clone().pow(2u64.into()) - px - px;
                    let ry = s * (&rx - px) + py;
                    Point::Pt(rx, -ry)
                }
            }
        }
    }

    pub fn mul(&self, p: &Point<M>, mut n: BigUint) -> Point<M> {
        let mut q = p.clone();
        while n > 0u64.into() {
            q = self.add(&p, &q);
            n -= 1u64;
        }
        q
    }
}

#[test]
fn weienerstrass() {
    let curve = WeienerstrassCurve {
        a: ModN::<N_101>::new(0u64),
        b: ModN::new(1u64),
    };
    let p = Point::Pt(ModN::new(38u64), ModN::new(38u64));
    let mut q = p.clone();
    for _ in 0..100 {
        print!("{:?} + {:?}", q, p);
        q = curve.add(&p, &q);
        println!(" = {:?}", q);
    }
}

/// (p, a, b, G, n, h)
pub struct ElipticCurve {
    /// G
    base_point: BigUint,
    /// p
    prime: BigUint,
    // a, b, n, h?
}

impl ElipticCurve {
    fn get_point(k: &BigUint) -> BigUint {
        todo!()
    }
}

// Eliptic curves

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("Hello!");
    }

    // If gcd(the product of the two private keys, P - 1) != 1, then the shared secret will be 1
    //
    // We choose germain primes (p if 2p+1 is also prime) to minimize this chance. We choose the
    // 2p+1, since the only possible orders of a memeber of (2p+1) are 1, 2, p, and 2p+1. The
    // number with order 1 is 1, so we just need a number that isn't divisible 2 or p.
    //
    // Actually, a number divisible by p might be okay?

    #[test]
    fn diffie() {
        let prime = BigUint::from_str("199679").unwrap();
        let private_key_a = 10000u64.into();
        let public_key_a = BigUint::from(2u64).modpow(&private_key_a, &prime);
        let private_key_b = 20480u64.into();
        let public_key_b = BigUint::from(2u64).modpow(&private_key_b, &prime);

        let a_shared = public_key_b.modpow(&private_key_a, &prime);
        let b_shared = public_key_a.modpow(&private_key_b, &prime);
        assert_eq!(a_shared, b_shared);
    }
}
