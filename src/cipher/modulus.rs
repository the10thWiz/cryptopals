//
// modulus.rs
// Copyright (C) 2022 matthew <matthew@WINDOWS-05HIC4F>
// Distributed under terms of the MIT license.
//

use std::{cmp::Ordering, marker::PhantomData, str::FromStr, fmt::Debug};

use num_bigint::BigUint;

pub trait PrimeMod {
    fn get_modulus() -> &'static BigUint;
}

macro_rules! prime_mod {
    ($name:ident => $num:literal $(, $($tt:tt)*)?) => {
        lazy_static::lazy_static! {
            pub static ref $name: BigUint = BigUint::from_str($num).unwrap();
        }

        impl PrimeMod for $name {
            fn get_modulus() -> &'static BigUint {
                &*$name
            }
        }
        $(
            prime_mod!($($tt)*);
        )?
    };
    () => {};
}
prime_mod!(
    N_5 => "5",
    N_7 => "7",
    N_101 => "101",
    CURVE_25519 =>
            "57896044618658097711785492504343953926634992332820282019728792003956564819949"
);

//lazy_static::lazy_static! {
//pub static ref N_5: BigUint = BigUint::from_str("5").unwrap();
//}

//impl PrimeMod for N_5 {
//fn get_modulus() -> &'static BigUint {
//&*N_5
//}
//}

pub struct ModN<M: PrimeMod>(BigUint, PhantomData<M>);

impl<M: PrimeMod> Clone for ModN<M> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}
impl<M: PrimeMod> PartialEq for ModN<M> {
    fn eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0
    }
}
impl<M: PrimeMod> Eq for ModN<M> {}
impl<M: PrimeMod> PartialOrd for ModN<M> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&rhs.0)
    }
}
impl<M: PrimeMod> Ord for ModN<M> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.0.cmp(&rhs.0)
    }
}
impl<M: PrimeMod> Debug for ModN<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn gcd(m: BigUint, n: BigUint) -> BigUint {
    if m == 0u64.into() {
        n
    } else {
        gcd(n % &m, m)
    }
}

impl<M: PrimeMod> ModN<M> {
    pub fn mul_inverse(self) -> Self {
        // Since PrimeMod is prime, gcd(self, M) = 1
        self.pow(M::get_modulus() - BigUint::from(2u64))
    }

    pub fn pow(mut self, mut exp: BigUint) -> Self {
        let modulus = M::get_modulus();
        let mut product = BigUint::from(1usize);
        while exp > BigUint::from(0usize) {
            if exp.bit(0) {
                product = (product * &self.0) % modulus;
            }
            self.0 = self.0.pow(2) % modulus;
            exp = exp >> 1u64;
        }
        Self(product, PhantomData)
    }

    pub fn new(u: impl Into<BigUint>) -> Self {
        Self(u.into() % M::get_modulus(), PhantomData)
    }
}

macro_rules! impl_op {
    (ModN, $op:ident, $fn:ident, $mod:ident, $lhs:ident, $rhs:ident => $code: expr) => {
        impl<$mod: PrimeMod> $op<ModN<$mod>> for ModN<$mod> {
            type Output = ModN<$mod>;
            fn $fn(self, $rhs: ModN<$mod>) -> Self::Output {
                let $lhs = self;
                ModN($code, PhantomData)
            }
        }
        impl<$mod: PrimeMod> $op<ModN<$mod>> for &ModN<$mod> {
            type Output = ModN<$mod>;
            fn $fn(self, $rhs: ModN<$mod>) -> Self::Output {
                let $lhs = self;
                ModN($code, PhantomData)
            }
        }
        impl<$mod: PrimeMod> $op<&ModN<$mod>> for &ModN<$mod> {
            type Output = ModN<$mod>;
            fn $fn(self, $rhs: &ModN<$mod>) -> Self::Output {
                let $lhs = self;
                ModN($code, PhantomData)
            }
        }
        impl<$mod: PrimeMod> $op<&ModN<$mod>> for ModN<$mod> {
            type Output = ModN<$mod>;
            fn $fn(self, $rhs: &ModN<$mod>) -> Self::Output {
                let $lhs = self;
                ModN($code, PhantomData)
            }
        }
    };
    (CPt, $op:ident, $fn:ident, $mod:ident, $lhs:ident, $rhs:ident => $code: expr) => {
        impl<M: PrimeMod, $mod: ElipticCurve<M>> $op<CurvePoint<M, $mod>> for CurvePoint<M, $mod> {
            type Output = CurvePoint<M, $mod>;
            fn $fn(self, $rhs: CurvePoint<M, $mod>) -> Self::Output {
                let $lhs = self;
                $code
            }
        }
        impl<M: PrimeMod, $mod: ElipticCurve<M>> $op<CurvePoint<M, $mod>> for &CurvePoint<M, $mod> {
            type Output = CurvePoint<M, $mod>;
            fn $fn(self, $rhs: CurvePoint<M, $mod>) -> Self::Output {
                let $lhs = self;
                $code
            }
        }
        impl<M: PrimeMod, $mod: ElipticCurve<M>> $op<&CurvePoint<M, $mod>> for &CurvePoint<M, $mod> {
            type Output = CurvePoint<M, $mod>;
            fn $fn(self, $rhs: &CurvePoint<M, $mod>) -> Self::Output {
                let $lhs = self;
                $code
            }
        }
        impl<M: PrimeMod, $mod: ElipticCurve<M>> $op<&CurvePoint<M, $mod>> for CurvePoint<M, $mod> {
            type Output = CurvePoint<M, $mod>;
            fn $fn(self, $rhs: &CurvePoint<M, $mod>) -> Self::Output {
                let $lhs = self;
                $code
            }
        }
    };
}

use std::ops::{Add, Div, Mul, Sub};
impl_op!(ModN, Add, add, M, lhs, rhs => (&lhs.0 + &rhs.0) % M::get_modulus());
impl_op!(ModN, Sub, sub, M, lhs, rhs => (&lhs.0 + &(-rhs).0) % M::get_modulus());
impl_op!(ModN, Mul, mul, M, lhs, rhs => (&lhs.0 * &rhs.0) % M::get_modulus());
impl_op!(ModN, Div, div, M, lhs, rhs => (&lhs.0 * rhs.clone().mul_inverse().0) % M::get_modulus());

impl<M: PrimeMod> std::ops::Neg for ModN<M> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(M::get_modulus() - self.0, PhantomData)
    }
}

impl<M: PrimeMod> std::ops::Neg for &ModN<M> {
    type Output = ModN<M>;
    fn neg(self) -> Self::Output {
        ModN(M::get_modulus() - &self.0, PhantomData)
    }
}

pub trait ElipticCurve<M: PrimeMod>: Sized {
    fn on_curve(pt: &CurvePoint<M, Self>) -> bool;
    fn add_pts(lhs: &CurvePoint<M, Self>, rhs: &CurvePoint<M, Self>) -> CurvePoint<M, Self>;
}

pub enum CurvePoint<M: PrimeMod, E: ElipticCurve<M>> {
    Pt(ModN<M>, ModN<M>, PhantomData<E>),
    Inf(PhantomData<E>),
}

pub enum CurveError {
    PointNotOnCurve,
}

impl<M: PrimeMod, E: ElipticCurve<M>> CurvePoint<M, E> {
    pub fn new(x: BigUint, y: BigUint) -> Result<Self, CurveError> {
        let ret = Self::Pt(ModN::new(x), ModN::new(y), PhantomData);
        if E::on_curve(&ret) {
            Ok(ret)
        } else {
            Err(CurveError::PointNotOnCurve)
        }
    }

    pub fn new_zero() -> Self {
        Self::Inf(PhantomData)
    }

    pub fn x(&self) -> Option<&ModN<M>> {
        match self {
            Self::Pt(x, y, _) => Some(x),
            Self::Inf(_) => None,
        }
    }

    pub fn y(&self) -> Option<&ModN<M>> {
        match self {
            Self::Pt(x, y, _) => Some(y),
            Self::Inf(_) => None,
        }
    }

    pub fn coords(&self) -> Option<(&ModN<M>, &ModN<M>)> {
        match self {
            Self::Pt(x, y, _) => Some((x, y)),
            Self::Inf(_) => None,
        }
    }
}

impl<M: PrimeMod, E: ElipticCurve<M>> Debug for CurvePoint<M, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pt(x, y, _) => write!(f, "({:?}, {:?})", x, y),
            Self::Inf(_) => write!(f, "(inf)"),
        }
    }
}

impl<M: PrimeMod, E: ElipticCurve<M>> Clone for CurvePoint<M, E> {
    fn clone(&self) -> Self {
        match self {
            Self::Pt(x, y, _) => Self::Pt(x.clone(), y.clone(), PhantomData),
            Self::Inf(_) => Self::Inf(PhantomData),
        }
    }
}

impl_op!(CPt, Add, add, C, lhs, rhs => C::add_pts(&lhs, &rhs));

pub struct TestCurve<M: PrimeMod>(PhantomData<M>);

impl<M: PrimeMod> ElipticCurve<M> for TestCurve<M> {
    fn on_curve(pt: &CurvePoint<M, Self>) -> bool {
        if let Some((x, y)) = pt.coords() {
            y.clone().pow(2u64.into()) == x.clone().pow(3u64.into()) + ModN::new(1u64)
        } else {
            true
        }
    }

    fn add_pts(p: &CurvePoint<M, Self>, q: &CurvePoint<M, Self>) -> CurvePoint<M, Self> {
        match (p, q) {
            (CurvePoint::Pt(px, py, _), CurvePoint::Pt(qx, qy, _)) => {
                if px == qx {
                    if py == qy && py != &ModN::new(0u64) {
                        let s = (ModN::new(3u64) * px.clone().pow(2u64.into()) + ModN::new(0u64))
                            / (ModN::new(2u64) * py);
                        let rx = s.clone().pow(2u64.into()) - px - qx;
                        let ry = s * (&rx - px) + py;
                        CurvePoint::Pt(rx, -ry, PhantomData)
                    } else {
                        CurvePoint::Inf(PhantomData)
                    }
                } else {
                    let s = (qy - py) / (qx - px);
                    let rx = s.clone().pow(2u64.into()) - px - qx;
                    let ry = s * (&rx - px) + py;
                    CurvePoint::Pt(rx, -ry, PhantomData)
                }
            }
            (CurvePoint::Inf(_), q) | (q, CurvePoint::Inf(_)) => q.clone(),
        }
    }
}
