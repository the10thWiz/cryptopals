use super::Bytes;
use std::ops::{
    Add, AddAssign, BitXor, BitXorAssign, Deref, Index, IndexMut, Mul, MulAssign, Range, RangeFrom,
    RangeFull, RangeTo,
};
use std::slice::{Iter, IterMut};

impl Bytes {
    pub fn iter_mut(&mut self) -> IterMut<u8> {
        self.bytes.iter_mut()
    }
    pub fn iter(&self) -> Iter<u8> {
        self.bytes.iter()
    }
}

impl Deref for Bytes {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

impl BitXor for Bytes {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, other: Self) -> Self::Output {
        let mut ret = Vec::new();
        for b in self.bytes.iter().enumerate() {
            ret.push(*b.1 ^ *other.bytes.get(b.0 % other.bytes.len()).unwrap());
        }
        Bytes { bytes: ret }
    }
}
impl BitXorAssign for Bytes {
    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor_assign(&mut self, other: Self) {
        for b in self.bytes.iter_mut().enumerate() {
            *b.1 = *b.1 ^ other.bytes.get(b.0 % other.bytes.len()).unwrap();
        }
    }
}
impl BitXor<u8> for Bytes {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, other: u8) -> Self::Output {
        let mut ret = Vec::new();
        for b in self.bytes.iter() {
            ret.push(*b ^ other);
        }
        Bytes { bytes: ret }
    }
}
impl BitXorAssign<u8> for Bytes {
    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor_assign(&mut self, other: u8) {
        for b in self.bytes.iter_mut() {
            *b = *b ^ other;
        }
    }
}

impl Add<Self> for Bytes {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut ret = self.bytes.clone();
        ret.append(&mut other.bytes.clone());
        Self { bytes: ret }
    }
}
impl Add<Bytes> for u8 {
    type Output = Bytes;
    fn add(self, other: Bytes) -> Bytes {
        let mut ret = other.bytes.clone();
        ret.insert(0, self);
        Bytes { bytes: ret }
    }
}
impl AddAssign<Self> for Bytes {
    fn add_assign(&mut self, other: Self) {
        self.bytes.append(&mut other.bytes.clone());
    }
}
impl Add<u8> for Bytes {
    type Output = Self;
    fn add(self, other: u8) -> Self {
        let mut ret = self.bytes.clone();
        ret.push(other);
        Self { bytes: ret }
    }
}
impl AddAssign<u8> for Bytes {
    fn add_assign(&mut self, other: u8) {
        self.bytes.push(other);
    }
}
impl Mul<usize> for Bytes {
    type Output = Self;
    fn mul(self, num: usize) -> Self {
        if num == 0 {
            return Bytes::zero(0);
        }
        let mut ret = self.bytes.clone();
        for _ in 1..num {
            ret.append(&mut self.bytes.clone());
        }
        Self { bytes: ret }
    }
}
impl MulAssign<usize> for Bytes {
    fn mul_assign(&mut self, num: usize) {
        let mut ret = self.bytes.clone();
        for _ in 1..num {
            ret.append(&mut self.bytes.clone());
        }
        self.bytes = ret;
    }
}

impl Index<usize> for Bytes {
    type Output = u8;

    fn index(&self, i: usize) -> &Self::Output {
        &self.bytes[i]
    }
}
impl IndexMut<usize> for Bytes {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.bytes[i]
    }
}
impl Index<Range<usize>> for Bytes {
    type Output = [u8];

    fn index(&self, i: Range<usize>) -> &Self::Output {
        &self.bytes[i]
    }
}
impl IndexMut<Range<usize>> for Bytes {
    fn index_mut(&mut self, i: Range<usize>) -> &mut Self::Output {
        &mut self.bytes[i]
    }
}
impl Index<RangeFrom<usize>> for Bytes {
    type Output = [u8];

    fn index(&self, i: RangeFrom<usize>) -> &Self::Output {
        &self.bytes[i]
    }
}
impl IndexMut<RangeFrom<usize>> for Bytes {
    fn index_mut(&mut self, i: RangeFrom<usize>) -> &mut Self::Output {
        &mut self.bytes[i]
    }
}
impl Index<RangeTo<usize>> for Bytes {
    type Output = [u8];

    fn index(&self, i: RangeTo<usize>) -> &Self::Output {
        &self.bytes[i]
    }
}
impl IndexMut<RangeTo<usize>> for Bytes {
    fn index_mut(&mut self, i: RangeTo<usize>) -> &mut Self::Output {
        &mut self.bytes[i]
    }
}
impl Index<RangeFull> for Bytes {
    type Output = [u8];

    fn index(&self, _: RangeFull) -> &Self::Output {
        &self.bytes
    }
}
impl IndexMut<RangeFull> for Bytes {
    fn index_mut(&mut self, _: RangeFull) -> &mut Self::Output {
        &mut self.bytes
    }
}

impl PartialEq for Bytes {
    fn eq(&self, other: &Self) -> bool {
        if self.bytes.len() == other.bytes.len() {
            for v in self.bytes.iter().zip(other.bytes.iter()) {
                if v.0 != v.1 {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
}
impl PartialEq<&str> for Bytes {
    fn eq(&self, other: &&str) -> bool {
        self.to_hex() == other.to_uppercase()
    }
}
impl std::cmp::Eq for Bytes {}

impl std::cmp::PartialOrd for Bytes {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::Ord for Bytes {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.bytes.len() > other.bytes.len() {
            std::cmp::Ordering::Greater
        } else if self.bytes.len() < other.bytes.len() {
            std::cmp::Ordering::Less
        } else {
            for bytes in self.bytes.iter().zip(other.bytes.iter()) {
                if bytes.0 > bytes.1 {
                    return std::cmp::Ordering::Greater;
                } else if bytes.0 < bytes.1 {
                    return std::cmp::Ordering::Less;
                }
            }
            std::cmp::Ordering::Equal
        }
    }
}
