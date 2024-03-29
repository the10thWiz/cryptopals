mod conversion;
mod display;
mod ops;

use crate::cipher::BLOCK_SIZE;
use rand::prelude::random;
use std::char;

/*
    data.rs 1013 lines before I split it up into the 4 files that exist now. I expect
    to work on some better comments/docs, as well as adding new functionality as needed.

    TODO: Write better Documentation
*/

///
/// A Standard buffer for binary data. This struct offers a variety of
/// convience methods for manipulating the data buffered inside.
///
/// The following operator overloads are defined:
///
/// Binary xor => xors raw data, byte for byte. right will be repeated
/// if it is shorter
///
/// Add => Concatinates buffers
///
/// Index => extracts raw data at index (u8), or slice(&[u8])
///
/// Multiply => repeats buffer x number of times
///
/// Equals/Compare => compares raw data in buffers
///
/// Display and Debug => Defualts to printing as utf-8, UpperHex is also
/// implemented
///
#[derive(Clone, Default)]
pub struct Bytes {
    bytes: Vec<u8>,
}

impl Bytes {
    pub fn new() -> Bytes {
        Self { bytes: Vec::new() }
    }
    pub fn with_capacity(size: usize) -> Bytes {
        Self {
            bytes: Vec::with_capacity(size),
        }
    }
    pub fn empty() -> Bytes {
        Self {
            bytes: Vec::default(),
        }
    }
    ///
    /// Reads raw u8 values from `v`
    ///
    pub fn from_vec(v: Vec<u8>) -> Bytes {
        Bytes { bytes: v }
    }
    ///
    /// Reads raw u8 values from `v`
    ///
    pub fn from_bytes(v: &[u8]) -> Bytes {
        let mut ret = Bytes {
            bytes: Vec::with_capacity(v.len()),
        };
        for val in v {
            ret.bytes.push(*val);
        }
        ret
    }
    ///
    /// Creates a new buffer of length `size`, filled with
    /// binary zeros
    ///
    pub fn zero(size: usize) -> Bytes {
        let mut ret = Vec::new();
        for _ in 0..size {
            ret.push(0u8);
        }
        Bytes { bytes: ret }
    }
    ///
    /// Generates a new buffer of length `size`, filled
    /// with random data
    ///
    pub fn rand(size: usize) -> Bytes {
        let mut ret = Vec::new();
        for _ in 0..size {
            ret.push(random());
        }
        Bytes { bytes: ret }
    }
    ///
    /// Returns raw data buffer as a `&[u8]`
    ///
    pub fn to_bytes(&self) -> &[u8] {
        &self.bytes
    }
    ///
    /// Returns raw data buffer as a `&[u8]`
    ///
    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        &mut self.bytes
    }
    ///
    /// Creates a `Vec` of size `block`, and creates a Bytes for each `block`
    ///
    /// e.g. [1F 22 33 44].collate(2) -> [[1F 33], [22 44]]
    ///
    pub fn collate(&self, block: usize) -> Vec<Bytes> {
        let mut ret = Vec::new();
        for _ in 0..block {
            ret.push(Bytes { bytes: Vec::new() });
        }
        for b in self.bytes.iter().enumerate() {
            ret[b.0 % block].bytes.push(*b.1);
        }
        ret
    }
    ///
    /// Undoes collate method
    ///
    /// `Bytes::decollate(a.collate(x)) == a`, for any x
    ///
    pub fn decollate(parts: Vec<Bytes>) -> Bytes {
        let mut ret = Vec::new();
        for i in 0..parts.first().unwrap().len() {
            for p in parts.iter() {
                match p.bytes.get(i) {
                    Some(v) => ret.push(*v),
                    None => (),
                }
            }
        }

        Bytes { bytes: ret }
    }
    ///
    /// Splits raw data buffer into `Bytes` of size
    /// block
    ///
    pub fn split(&self, len: usize) -> Vec<Bytes> {
        let mut ret: Vec<Bytes> = Vec::new();
        for i in 0..self.bytes.len() / len {
            let mut cur = Bytes { bytes: Vec::new() };
            for j in i * len..(i + 1) * len {
                cur.bytes.push(self.bytes[j]);
            }
            ret.push(cur);
        }
        ret
    }
    ///
    /// Pads data to a multiple of `padding`, using
    /// PKCS#7
    ///
    pub fn pad_pkcs7(&self, padding: usize) -> Bytes {
        let mut ret = self.clone();
        let num = padding - self.len() % padding;
        for _ in 0..num {
            ret.bytes.push(num as u8);
        }
        ret
    }
    ///
    /// Trims PKCS#7 padding from bytes
    ///
    /// Doesn't assume any padding has been applied, and
    /// just returns itself if the last byte isn't a multiple
    /// of the padding
    ///
    pub fn trim_pkcs7(&self) -> Bytes {
        let pad_num = self.bytes[self.bytes.len() - 1] as usize;
        // println!("{}", pad_num);
        if pad_num <= BLOCK_SIZE {
            for b in self.bytes.len() - pad_num..self.bytes.len() {
                if self.bytes[b] != pad_num as u8 {
                    return self.clone();
                }
            }
            return self.clone().truncate(self.bytes.len() - pad_num);
        }
        self.clone()
    }
    ///
    /// Truncates data to len bytes, discarding any data after
    /// the cutoff
    ///
    pub fn truncate(&self, len: usize) -> Bytes {
        let mut ret = self.clone();
        while ret.bytes.len() > len {
            ret.bytes.pop();
        }
        ret
    }
    ///
    /// Discard first len bytes
    ///
    pub fn truncate_start(&self, len: usize) -> Bytes {
        let mut ret = self.clone();
        for _ in 0..len {
            ret.bytes.remove(0);
        }
        ret
    }
    ///
    /// replace bytes from `index` with `part`
    ///
    pub fn replace(&self, part: &[u8], index: usize) -> Bytes {
        if part.len() + index > self.bytes.len() {
            panic!("Part is to long to fit");
        }
        let mut ret = self.bytes.clone();
        for i in 0..part.len() {
            ret[i + index] = part[i];
        }
        Bytes { bytes: ret }
    }
    ///
    /// replace bytes from `block * open_ssl::BLOCK_SIZE` with `part`
    ///
    pub fn replace_block(&self, part: &[u8], block: usize) -> Bytes {
        if part.len() + block * 16 > self.bytes.len() {
            panic!("Part is to long to fit");
        }
        let mut ret = self.bytes.clone();
        for i in 0..part.len() {
            ret[i + block * 16] = part[i];
        }
        Bytes { bytes: ret }
    }
    ///
    /// Similar to `self[i]`, except the return type is `char`
    ///
    pub fn get(&self, i: usize) -> char {
        self.bytes[i] as char
    }
    ///
    /// Increments data from the right
    ///
    /// returns true if data rolls over from the max value
    ///
    pub fn inc(&mut self) -> bool {
        for b in self.bytes.iter_mut().rev() {
            if *b == 255 {
                *b = 0;
            } else {
                *b += 1;
                return false;
            }
        }
        true
    }
    ///
    /// Only returns true if data is entirely zeros
    ///
    pub fn is_zero(&self) -> bool {
        for b in self.bytes.iter() {
            if *b != 0 {
                return false;
            }
        }
        true
    }
    ///
    /// Length of buffer, in bytes
    ///
    pub fn len(&self) -> usize {
        self.bytes.len()
    }
    ///
    /// Creates a new buffer with only values that don't match
    /// `val`
    ///
    pub fn remove(&self, val: u8) -> Bytes {
        let mut ret = Vec::new();
        for b in self.bytes.iter() {
            if *b != val {
                ret.push(*b);
            }
        }
        Bytes { bytes: ret }
    }
    /// Transposes the vector of bytes
    ///
    /// Most useful when converting a series of blocks into seperate
    /// parts. Intended for things like repeating key xor, etc
    pub fn pivot(v: Vec<Bytes>) -> Vec<Bytes> {
        let mut len = usize::max_value();
        for t in v.iter() {
            if t.len() < len {
                len = t.len();
            }
        }
        let mut ret = Vec::with_capacity(len);
        for i in 0..len {
            let mut b = Self::zero(v.len());
            for j in 0..v.len() {
                b[j] = v[j][i];
            }
            ret.push(b);
        }
        ret
    }
    // Methods to convert u8 and chars in differing radixs
    fn sextet_to_64(i: u8) -> char {
        match i {
            00 => 'A',
            01 => 'B',
            02 => 'C',
            03 => 'D',
            04 => 'E',
            05 => 'F',
            06 => 'G',
            07 => 'H',
            08 => 'I',
            09 => 'J',
            10 => 'K',
            11 => 'L',
            12 => 'M',
            13 => 'N',
            14 => 'O',
            15 => 'P',
            16 => 'Q',
            17 => 'R',
            18 => 'S',
            19 => 'T',
            20 => 'U',
            21 => 'V',
            22 => 'W',
            23 => 'X',
            24 => 'Y',
            25 => 'Z',
            26 => 'a',
            27 => 'b',
            28 => 'c',
            29 => 'd',
            30 => 'e',
            31 => 'f',
            32 => 'g',
            33 => 'h',
            34 => 'i',
            35 => 'j',
            36 => 'k',
            37 => 'l',
            38 => 'm',
            39 => 'n',
            40 => 'o',
            41 => 'p',
            42 => 'q',
            43 => 'r',
            44 => 's',
            45 => 't',
            46 => 'u',
            47 => 'v',
            48 => 'w',
            49 => 'x',
            50 => 'y',
            51 => 'z',
            52 => '0',
            53 => '1',
            54 => '2',
            55 => '3',
            56 => '4',
            57 => '5',
            58 => '6',
            59 => '7',
            60 => '8',
            61 => '9',
            62 => '+',
            63 => '/',
            _ => panic!("This error should never occur"),
        }
    }
    fn b64_to_sextet(i: char) -> u8 {
        match i {
            'A' => 00,
            'B' => 01,
            'C' => 02,
            'D' => 03,
            'E' => 04,
            'F' => 05,
            'G' => 06,
            'H' => 07,
            'I' => 08,
            'J' => 09,
            'K' => 10,
            'L' => 11,
            'M' => 12,
            'N' => 13,
            'O' => 14,
            'P' => 15,
            'Q' => 16,
            'R' => 17,
            'S' => 18,
            'T' => 19,
            'U' => 20,
            'V' => 21,
            'W' => 22,
            'X' => 23,
            'Y' => 24,
            'Z' => 25,
            'a' => 26,
            'b' => 27,
            'c' => 28,
            'd' => 29,
            'e' => 30,
            'f' => 31,
            'g' => 32,
            'h' => 33,
            'i' => 34,
            'j' => 35,
            'k' => 36,
            'l' => 37,
            'm' => 38,
            'n' => 39,
            'o' => 40,
            'p' => 41,
            'q' => 42,
            'r' => 43,
            's' => 44,
            't' => 45,
            'u' => 46,
            'v' => 47,
            'w' => 48,
            'x' => 49,
            'y' => 50,
            'z' => 51,
            '0' => 52,
            '1' => 53,
            '2' => 54,
            '3' => 55,
            '4' => 56,
            '5' => 57,
            '6' => 58,
            '7' => 59,
            '8' => 60,
            '9' => 61,
            '+' => 62,
            '/' => 63,
            '=' => 00,
            _ => panic!("This error should never occur"),
        }
    }
    fn byte_to_hex(b: u8) -> String {
        let mut ret = String::with_capacity(2);
        ret += match b / 0x10 {
            0x00 => "0",
            0x01 => "1",
            0x02 => "2",
            0x03 => "3",
            0x04 => "4",
            0x05 => "5",
            0x06 => "6",
            0x07 => "7",
            0x08 => "8",
            0x09 => "9",
            0x0A => "A",
            0x0B => "B",
            0x0C => "C",
            0x0D => "D",
            0x0E => "E",
            0x0F => "F",
            _ => panic!("This Error should never be possible"),
        };
        ret += match b % 0x10 {
            0x00 => "0",
            0x01 => "1",
            0x02 => "2",
            0x03 => "3",
            0x04 => "4",
            0x05 => "5",
            0x06 => "6",
            0x07 => "7",
            0x08 => "8",
            0x09 => "9",
            0x0A => "A",
            0x0B => "B",
            0x0C => "C",
            0x0D => "D",
            0x0E => "E",
            0x0F => "F",
            _ => panic!("This Error should never be possible"),
        };
        ret
    }
    fn byte_to_lower_hex(b: u8) -> String {
        let mut ret = String::with_capacity(2);
        ret += match b / 0x10 {
            0x00 => "0",
            0x01 => "1",
            0x02 => "2",
            0x03 => "3",
            0x04 => "4",
            0x05 => "5",
            0x06 => "6",
            0x07 => "7",
            0x08 => "8",
            0x09 => "9",
            0x0A => "a",
            0x0B => "b",
            0x0C => "c",
            0x0D => "d",
            0x0E => "e",
            0x0F => "f",
            _ => panic!("This Error should never be possible"),
        };
        ret += match b % 0x10 {
            0x00 => "0",
            0x01 => "1",
            0x02 => "2",
            0x03 => "3",
            0x04 => "4",
            0x05 => "5",
            0x06 => "6",
            0x07 => "7",
            0x08 => "8",
            0x09 => "9",
            0x0A => "a",
            0x0B => "b",
            0x0C => "c",
            0x0D => "d",
            0x0E => "e",
            0x0F => "f",
            _ => panic!("This Error should never be possible"),
        };
        ret
    }

    /**
     * Reads data as hex values from `s`
     *
     * Allows both upper and lower case
     */
    pub fn read_hex(s: &str) -> Bytes {
        let mut ret = Vec::with_capacity(s.len() / 2);
        let mut cur = 0u8;
        let mut top = true;
        for c in s.chars() {
            let tmp = match c {
                '0' => 0x0u8,
                '1' => 0x1u8,
                '2' => 0x2u8,
                '3' => 0x3u8,
                '5' => 0x5u8,
                '4' => 0x4u8,
                '6' => 0x6u8,
                '7' => 0x7u8,
                '8' => 0x8u8,
                '9' => 0x9u8,
                'A' | 'a' => 0xAu8,
                'B' | 'b' => 0xBu8,
                'C' | 'c' => 0xCu8,
                'D' | 'd' => 0xDu8,
                'E' | 'e' => 0xEu8,
                'F' | 'f' => 0xFu8,
                _ => panic!("Unexpected Character {}", c),
            };
            if top {
                cur = tmp << 4;
                top = false;
            } else {
                ret.push(cur | tmp);
                top = true;
            }
        }
        if !top {
            ret.push(cur);
        }
        Bytes { bytes: ret }
    }
    /**
     * Reads UTF-8 values from `s`
     */
    pub fn read_utf8(s: &str) -> Bytes {
        let mut ret = Vec::with_capacity(s.len());
        for c in s.bytes() {
            ret.push(c);
        }
        Bytes { bytes: ret }
    }
    /**
     * Reads data as base 64 values from `s`
     *
     * Allows, but does not require `=` padding
     */
    pub fn read_64(s: &str) -> Bytes {
        let mut ret = Vec::new();
        let mut carry = 0u8;
        let mut mask = 0u8;
        for c in s.chars() {
            let sextet = Bytes::b64_to_sextet(c);
            if mask == 0 {
                carry = sextet << 2;
                mask = 0b11111100;
            } else if mask == 0b11111100 {
                ret.push((carry & mask) | (sextet >> 4));
                carry = sextet << 4;
                mask = 0b11110000;
            } else if mask == 0b11110000 {
                ret.push((carry & mask) | (sextet >> 2));
                carry = sextet << 6;
                mask = 0b11000000;
            } else if mask == 0b11000000 {
                ret.push((carry & mask) | (sextet >> 0));
                carry = 0;
                mask = 0;
            }
        }
        if mask == 0b11111100 {
            ret.push((carry & mask) | (0 >> 4));
        } else if mask == 0b11110000 {
            ret.push((carry & mask) | (0 >> 2));
        } else if mask == 0b11000000 {
            ret.push((carry & mask) | (0 >> 0));
        }
        Bytes { bytes: ret }
    }
    /**
     * Converts raw data buffer to a Hex encoded string
     */
    pub fn to_hex(&self) -> String {
        let mut ret = String::with_capacity(self.bytes.len() * 2);
        for b in self.bytes.iter() {
            ret += &Bytes::byte_to_hex(*b);
        }
        ret
    }
    /**
     * Converts raw data buffer to a Hex encoded string, using lower case letters
     */
    pub fn to_lower_hex(&self) -> String {
        let mut ret = String::with_capacity(self.bytes.len() * 2);
        for b in self.bytes.iter() {
            ret += &Bytes::byte_to_lower_hex(*b);
        }
        ret
    }
    /**
     * Converts raw data buffer to a UTF-8 string, assuming the buffer contains
     * valid UTF-8
     */
    pub fn to_utf8(&self) -> std::borrow::Cow<str> {
        String::from_utf8_lossy(&self.bytes[..])
    }
    /**
     * Converts raw data buffer to a UTF-8 string, assuming the buffer contains
     * valid UTF-8
     */
    pub fn to_ascii(&self) -> Result<String, Self> {
        self.bytes.iter().try_fold(String::new(), |mut s, &b| {
            if b <= 127 {
                s.push(b as char);
                Ok(s)
            } else {
                Err(self.clone())
            }
        })
    }
    /**
     * Converts raw data buffer to a Base 64 encoded
     * string
     */
    pub fn to_64(&self) -> String {
        let mut ret = String::default();
        let mut leftover = 0u8;
        let mut mask = 0u8;
        for b in self.bytes.iter() {
            if mask == 0b000000 {
                ret.push(Bytes::sextet_to_64(b >> 2u8));
                mask = 0b000011;
                leftover = b & mask;
            } else if mask == 0b000011 {
                ret.push(Bytes::sextet_to_64((b >> 4u8) | (leftover << 4u8)));
                mask = 0b001111;
                leftover = b & mask;
            } else if mask == 0b001111 {
                ret.push(Bytes::sextet_to_64((b >> 6u8) | (leftover << 2u8)));
                ret.push(Bytes::sextet_to_64(b & 0b111111));
                mask = 0;
                leftover = 0;
            } else {
                break;
            }
        }
        ret
    }
}
