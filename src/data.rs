
use std::fmt;
use std::ops;

#[derive(Debug, Clone)]
pub struct Bytes {
    bytes:Vec<u8>
}

#[allow(dead_code)]
impl Bytes {
    pub fn read_hex(s: &str) -> Bytes {
        let mut ret = Vec::new();
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
                _ => panic!("Unexpected Character {}", c)
            };
            if top {
                cur = tmp << 4;
                top = false;
            }else {
                ret.push(cur | tmp);
                top = true;
            }
        }
        if !top {
            ret.push(cur);
        }
        Bytes {bytes: ret}
    }
    pub fn read_64(s: &str) -> Bytes {
        let mut ret = Vec::new();
        let mut carry = 0u8;
        let mut mask = 0u8;
        for c in s.chars() {
            let sextet = Bytes::b64_to_sextet(c);
            if mask == 0 {
                carry = sextet << 2;
                mask = 0b11111100;
            }else if mask == 0b11111100 {
                ret.push((carry&mask) | (sextet >> 4));
                carry = sextet << 4;
                mask = 0b11110000;
            }else if mask == 0b11110000 {
                ret.push((carry&mask) | (sextet >> 2));
                carry = sextet << 6;
                mask = 0b11000000;
            }else if mask == 0b11000000 {
                ret.push((carry&mask) | (sextet >> 0));
                carry = 0;
                mask = 0;
            }
        }
        if mask == 0b11111100 {
            ret.push((carry&mask) | (0 >> 4));
        }else if mask == 0b11110000 {
            ret.push((carry&mask) | (0 >> 2));
        }else if mask == 0b11000000 {
            ret.push((carry&mask) | (0 >> 0));
        }
        Bytes {bytes: ret}
    }
    pub fn read_utf8(s: &str) -> Bytes {
        let mut ret = Vec::new();
        for c in s.bytes() {
            ret.push(c);
        }
        Bytes {bytes: ret}
    }
    pub fn from_vec(v: Vec<u8>) -> Bytes {
        Bytes {bytes: v}
    }
    pub fn from_bytes(v: &[u8]) -> Bytes {
        let mut ret = Bytes {bytes: Vec::new()};
        for val in v {
            ret.bytes.push(*val);
        }
        ret
    }
    pub fn zero(size: usize) -> Bytes {
        let mut ret = Vec::new();
        for _ in 0..size {
            ret.push(0u8);
        }
        Bytes {bytes: ret}
    }
    pub fn to_hex(&self) -> String {
        let mut ret = String::default();
        for b in self.bytes.iter() {
            ret.push_str(match b / 0x10 {
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
                _ => panic!("This Error should never be possible")
            });
            ret.push_str(match b % 0x10 {
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
                _ => panic!("This Error should never be possible")
            });
        }
        ret
    }
    pub fn to_ascii(&self) -> String {
        let mut ret = String::default();
        for b in self.bytes.iter() {
            ret.push(*b as char);
        }
        ret
    }
    pub fn to_64(&self) -> String {
        let mut ret = String::default();
        let mut leftover = 0u8;
        let mut mask = 0u8;
        for b in self.bytes.iter() {
            if       mask == 0b000000 {
                ret.push(Bytes::sextet_to_64(b >> 2u8));
                mask = 0b000011;
                leftover = b & mask;
            }else if mask == 0b000011 {
                ret.push(Bytes::sextet_to_64((b >> 4u8) | (leftover << 4u8)));
                mask = 0b001111;
                leftover = b & mask;
            }else if mask == 0b001111 {
                ret.push(Bytes::sextet_to_64((b >> 6u8) | (leftover << 2u8)));
                ret.push(Bytes::sextet_to_64(b & 0b111111));
                mask = 0;
                leftover = 0;
            }else {
                break;
            }
        }
        ret
    }
    fn sextet_to_64(i:u8) -> char {
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
            _ => panic!("This error should never occur")
        }
    }
    fn b64_to_sextet(i:char) -> u8 {
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
            _ => panic!("This error should never occur")
        }
    }
    pub fn to_bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub fn collate(&self, block:usize) -> Vec<Bytes> {
        let mut ret = Vec::new();
        for _ in 0..block {
            ret.push(Bytes {bytes:Vec::new()});
        }
        for b in self.bytes.iter().enumerate() {
            ret[b.0 % block].bytes.push(*b.1);
        }
        ret
    }
    pub fn decollate(parts:Vec<Bytes>) -> Bytes {
        let mut ret = Vec::new();
        for i in 0..parts.first().unwrap().size() {
            for p in parts.iter() {
                match p.bytes.get(i) {
                    Some(v) => ret.push(*v),
                    None => ()
                }
            }
        }

        Bytes {bytes:ret}
    }
    pub fn split(&self, len:usize) -> Vec<Bytes> {
        let mut ret: Vec<Bytes> = Vec::new();
        for i in 0..self.bytes.len()/len {
            let mut cur = Bytes {bytes:Vec::new()};
            for j in i*len..(i+1)*len {
                cur.bytes.push(self.bytes[j]);
            }
            ret.push(cur);
        }
        ret
    }
    pub fn pad_pkcs7(&self, padding:usize) -> Bytes {
        let mut ret = self.clone();
        for _ in 0..padding {
            ret.bytes.push(padding as u8);
        }
        ret
    }
    pub fn truncate(&self, len:usize) -> Bytes {
        let mut ret = self.clone();
        while ret.bytes.len() > len {
            ret.bytes.pop();
        }
        ret
    }
    pub fn get(&self, i:usize) -> char {
        return self.bytes[i] as char;
    }
    pub fn xor_byte(&self, other:u8) -> Bytes {
        let mut ret = Vec::new();
        for b in self.bytes.iter() {
            ret.push(b ^ other);
        }
        Bytes {bytes:ret}
    }
    pub fn inc(&mut self) -> bool {
        for b in self.bytes.iter_mut().rev() {
            if *b == 255  {
                *b = 0;
            }else {
                *b+= 1;
                return false;
            }
        }
        true
    }
    pub fn is_zero(&self) -> bool {
        for b in self.bytes.iter() {
            if *b != 0 {
                return false;
            }
        }
        true
    }
    pub fn size(&self) -> usize {
        self.bytes.len()
    }
}

impl ops::BitXor for Bytes {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, other: Self) -> Self::Output {
        let mut ret = Vec::new();
        for b in self.bytes.iter().enumerate() {
            ret.push(*b.1 ^ *other.bytes.get(b.0 % other.bytes.len()).unwrap());
        }
        Bytes {bytes:ret}
    }
}
impl ops::BitXorAssign for Bytes {

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor_assign(&mut self, other: Self) {
        for b in self.bytes.iter_mut().enumerate() {
            *b.1 = *b.1 ^ other.bytes.get(b.0 % other.bytes.len()).unwrap();
        }
    }
}
impl ops::Add<Self> for Bytes {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut ret  = self.bytes.clone();
        ret.append(&mut other.bytes.clone());
        Self {bytes: ret}
    }
}
impl ops::AddAssign<Self> for Bytes {
    fn add_assign(&mut self, other: Self) {
        self.bytes.append(&mut other.bytes.clone());
    }
}

impl PartialEq for Bytes {
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }
}
impl PartialEq<&str> for Bytes {
    fn eq(&self, other: &&str) -> bool {
        self.to_hex() == other.to_uppercase()
    }
}
impl std::cmp::Eq for Bytes {}

impl fmt::Display for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}
