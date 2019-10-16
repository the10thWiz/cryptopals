pub const W: usize = 32; // word size (in number of bits)
pub const N: usize = 624; // degree of recurrence
pub const M: usize = 397; // middle word, an offset used in the recurrence relation defining the series x, 1 ≤ m < n
pub const R: u32 = 31; // separation point of one word, or the number of bits of the lower bitmask, 0 ≤ r ≤ w - 1
pub const A: u32 = 0x9908B0DF; // coefficients of the rational normal form twist matrix
pub const B: u32 = 0x9D2C5680; // TGFSR(R) tempering bitmasks
pub const C: u32 = 0xEFC60000;
pub const S: u32 = 7; // TGFSR(R) tempering bit shifts
pub const T: u32 = 15;
pub const U: u32 = 11; // additional Mersenne Twister tempering bit shifts/masks
pub const D: u32 = 0xFFFFFFFF;
pub const L: u32 = 18;
// 2^(nw − r) − 1 is a Mersenne prime
// (w, n, m, r) = (32, 624, 397, 31)
// a = 9908B0DF_16
// (u, d) = (11, FFFFFFFF_16)
// (s, b) = (7, 9D2C5680_16)
// (t, c) = (15, EFC60000_16)
// l = 18
pub const F: u64 = 1812433253;

const LOWSER_MASK: u32 = (1 << R) - 1; // That is, the binary number of r 1's
const UPPER_MASK: u32 = !LOWSER_MASK;

pub struct MersenneGen {
    vals: [u32; N],
    index: usize,
}

impl MersenneGen {
    pub fn new(seed: u32) -> Self {
        let mut ret = Self {
            vals: [0; N],
            index: N,
        };
        ret.vals[0] = seed;
        for i in 1..N {
            ret.vals[i] =
                (F * ((ret.vals[i - 1] ^ (ret.vals[i - 1] >> (W - 2))) + i as u32) as u64) as u32;
        }
        ret
    }
    pub fn get_internal(&self, i: usize) -> u32 {
        self.vals[i]
    }
    pub fn extract_number(&mut self) -> u32 {
        if self.index >= N {
            self.twist()
        }

        let mut y = self.vals[self.index];
        // y = y ^ ((y >> U) & D);
        y = y ^ ((y << S) & B);
        y = y ^ ((y << T) & C);
        y = y ^ (y >> L);

        self.index = self.index + 1;
        y // y is a u32
    }
    fn twist(&mut self) {
        for i in 0..N {
            let x = (self.vals[i] & UPPER_MASK) + (self.vals[(i + 1) % N] & LOWSER_MASK);
            let mut x_a = x >> 1;
            if (x % 2) != 0 {
                // lowest bit of x is 1
                x_a = x_a ^ A;
            }
            self.vals[i] = self.vals[(i + M) % N] ^ x_a;
        }
        self.index = 0;
    }
}
