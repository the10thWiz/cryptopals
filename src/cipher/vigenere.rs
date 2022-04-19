//
// vigenere.rs
// Copyright (C) 2022 matthew <matthew@WINDOWS-05HIC4F>
// Distributed under terms of the MIT license.
//

fn add_chars(a: char, key: char) -> char {
    match a {
        'a'..='z' => ((a as u8 + key as u8 - ('a' as u8) * 2) % 26 + 'a' as u8) as char,
        'A'..='Z' => ((a as u8 + key as u8 - ('a' as u8) * 2) % 26 + 'a' as u8) as char,
        _ => a,
    }
}

fn sub_chars(a: char, key: char) -> char {
    match a {
        'a'..='z' => ((a as u8 + 26 - key as u8) % 26 + 'a' as u8) as char,
        'A'..='Z' => ((a as u8 + 26 - key as u8) % 26 + 'a' as u8) as char,
        _ => a,
    }
}

pub struct VignereKey {
    key: String,
}

impl VignereKey {
    pub fn new(s: impl Into<String>) -> Self {
        Self { key: s.into() }
    }

    pub fn encrypt(&self, plain: impl AsRef<str>) -> String {
        plain
            .as_ref()
            .chars()
            .filter(|c| c.is_alphabetic())
            .zip(self.key.chars().cycle())
            .map(|(c, k)| add_chars(c, k))
            .collect()
    }

    pub fn decrypt(&self, plain: impl AsRef<str>) -> String {
        let mut ret = String::new();
        let mut key = self.key.chars().cycle();
        for c in plain.as_ref().chars() {
            if c.is_alphabetic() {
                ret.push(sub_chars(c, key.next().unwrap()));
            }
        }
        ret
    }

    pub fn break_len(cipher: impl AsRef<str>) -> usize {
        // When slid to the right by n, count the number of matches
        // find the max number of matches to guess key length
        let s = cipher.as_ref();
        (0..20)
            .max_by_key(|&i| {
                s.chars()
                    .zip(s.chars().skip(i))
                    .filter(|(a, b)| a == b)
                    .count()
            })
            .unwrap_or(0)
    }
}

mod straddling_checkerboard {
    use rand::Rng;

    // 0 1 2 3 4 5 6 7 8 9
    // t o 	e 		r 	s 	i 	n 		a
    //3 	b 	d 	f 	c 	m 	l 	k 	h 	j 	g
    //8 	q 	/ 	u 	w 	z 	v 	x 	y 	. 	p
    //
    // 3 & 8 don't encode spaces, they mark the next digit as being in one of the lower rows.
    fn straddling_checkerboard_encode(symbol: char) -> &'static str {
        match symbol {
            't' => "0",
            'o' => "1",
            'e' => "2",
            ' ' => {
                if rand::thread_rng().gen_bool(0.5) {
                    "3"
                } else {
                    "8"
                }
            }
            'r' => "4",
            's' => "5",
            'i' => "6",
            'n' => "7",
            ' ' => "8",
            'a' => "9",

            'b' => "30",
            'd' => "31",
            'f' => "32",
            'c' => "33",
            'm' => "34",
            'l' => "35",
            'k' => "36",
            'h' => "37",
            'j' => "38",
            'g' => "39",

            'q' => "80",
            '/' => "81",
            'u' => "82",
            'w' => "83",
            'z' => "84",
            'v' => "85",
            'x' => "86",
            'y' => "87",
            '.' => "88",
            'p' => "89",

            '0' => "810",
            '1' => "811",
            '2' => "812",
            '3' => "813",
            '4' => "814",
            '5' => "815",
            '6' => "816",
            '7' => "817",
            '8' => "818",
            '9' => "819",

            _ => unimplemented!("Only a subset of characters are valid"),
        }
    }

    fn straddling_checkerboard_decode(symbol: &str) -> char {
        match symbol {
            "0" => 't',
            "1" => 'o',
            "2" => 'e',
            "3" => ' ',
            "4" => 'r',
            "5" => 's',
            "6" => 'i',
            "7" => 'n',
            "8" => ' ',
            "9" => 'a',

            "30" => 'b',
            "31" => 'd',
            "32" => 'f',
            "33" => 'c',
            "34" => 'm',
            "35" => 'l',
            "36" => 'k',
            "37" => 'h',
            "38" => 'j',
            "39" => 'g',

            "80" => 'q',
            "81" => '/',
            "82" => 'u',
            "83" => 'w',
            "84" => 'z',
            "85" => 'v',
            "86" => 'x',
            "87" => 'y',
            "88" => '.',
            "89" => 'p',

            _ => unimplemented!("Only a subset of characters are valid"),
        }
    }

    fn to_inner(c: char) -> Option<char> {
        match c {
            'a'..='z' | ' ' | '.' | '/' => Some(c),
            '0'..='9' => Some(c),
            'A'..='Z' => Some(c.to_ascii_uppercase()),
            _ => None,
        }
    }

    pub fn encode_straddling(plain: &str) -> String {
        plain
            .chars()
            .flat_map(to_inner)
            .map(|c| straddling_checkerboard_encode(c))
            .collect()
    }

    pub fn decode_straddling(cipher: &str) -> String {
        todo!()
        //plain.chars().filter_map(to_inner).map(|c| straddling_checkerboard_encode(c)).collect()
    }

    pub fn encrypt(plain: &str, key: &str) -> String {
        let key = encode_straddling(key);
        encode_straddling(plain).chars().zip(key.chars().cycle()).map(|(p, k)| {
            (b'0' + ((p as u8 - b'0') + (k as u8 - b'0')) % 10) as char
        }).collect()
    }

    pub fn decrypt(cipher: &str, key: &str) -> String {
        let key = encode_straddling(key);
        encode_straddling(cipher).chars().zip(key.chars().cycle()).map(|(c, k)| {
            (b'0' + ((c as u8 - b'0') - (k as u8 - b'0') + 10) % 10) as char
        }).collect()
    }
}

mod playfair {
    // Select Key
    // remove duplicates, and convert j to i
    //
    // Key goes in 5x5 grid
    // Plain text is split into chunks of two
    //  if the chunk has two of the same letter, insert an x
    // Encode each chunk by taking the other two corners of the rectangle formed by the two letter
    // in the matrix. (keep the row the same to keep the order consistent)
    // If they are on the same row, use the letters after them, if they are in the same col use the
    // letters below them. (wrapping as needed)
    //
    // Decoding is symetric, although the same row/col is now left/up
    pub struct PlayFairKey {
        matrix: [[char; 5]; 5],
    }

    // Alternate: construct 5x5 grid (i/j) (ADFGX)
    // encode each character as the coordinates in the grid

    // My system:
    // Split plaintext into chunks of five characters. Each chunk contains five characters of the
    // plain text, and five characters as a key that encrypts the next chunk.
    //
    // Alternate: One Time Pad plaintext pad? This requires random padding in the plaintext such
    // that it isn't possible to guess part of the plaintext. The initial key is used to encrypt
    // the first part of the plaintext. Padding also needs to be systematic to allow removal later.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let key = VignereKey::new("secret");
        assert_eq!(key.encrypt("today is tuesday"), "lsfrcbkxwvwwsc");
        assert_eq!(key.decrypt("lsfrcbkxwvwwsc"), "todayistuesday");
    }

    #[test]
    fn straddling_checkerboard_encode() {
        let plain = "thereareatotalof26lettersintheenglishalphabet.howevermanyoftheselettersappearatdifferentfrequencies.
asanexampleweshouldnotethatthelettereappearsfarmoreoftenthanmostletters.
ontheotherhandthelettersqandzarebothexceptionallyuncommon.";
        let cipher = super::straddling_checkerboard::encode_straddling(plain);
        println!("{}", cipher);
        let mut counts = [0; 10];
        for c in cipher.chars() {
            counts[(c as u8 - b'0') as usize] += 1;
        }
        println!("C: {:?}", counts);
        let cipher = super::straddling_checkerboard::encode_straddling("mscs396");
        println!("{}", cipher);
        let cipher = super::straddling_checkerboard::encrypt(plain, "mscs396");
        println!("{}", cipher);
        let mut counts = [0; 10];
        for c in cipher.chars() {
            counts[(c as u8 - b'0') as usize] += 1;
        }
        println!("C: {:?}", counts);
    }
}
