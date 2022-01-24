use rand::{prelude::SliceRandom, thread_rng};

//etaoinsrhldcumfpgwybvkxjqz ETAOINSRHLDCUMFPGWYBVKXJQZ.?!123456789
//etaoin srhldcumfpgwybvkxjqzETAOINSRHLDCUMFPGWYBVKXJQZ0123456789.?!
//                   etaoinsrhldcumfpgwybvkxjqz
const ORDER: &str = "etaoinsrhldcumfpgwybvkxjqz ETAOINSRHLDCUMFPGWYBVKXJQZ.?!123456789";

/**
 * Makes a simple guess at whether the string provided is English, using etaoin order
 */
pub fn score_string(s: &str) -> f64 {
    let mut score: isize = 0;
    for c in s.chars() {
        score += match ORDER.find(c) {
            None => 255,
            Some(i) => i as isize,
        }
    }
    score as f64
}

/**
 * Returns the number of "invalid" letters in a string - the valid
 *
 * Valid = Ascii, alpha-numeric, or selected puncuation
 */
pub fn count_invalid_letters(s: &str) -> f64 {
    let mut score = 0;
    for c in s.chars() {
        score += match c {
            'a'..='z'
            | 'A'..='Z'
            | '0'..='9'
            | ' '
            | '.'
            | ','
            | '?'
            | '!'
            | '-'
            | '\''
            | '"'
            | '/' => -1,
            _ => 1,
        }
    }
    score as f64
}

#[allow(dead_code)]
/// (char, expected frequency, multiplier)
/// The multiplier can be used to weight
/// letters based on how close they should match
const STD_FREQ: [(char, f64, f64); 26] = [
    ('a', 08.167, 0.0),
    ('b', 01.492, 0.0),
    ('c', 02.782, 0.0),
    ('d', 04.253, 0.0),
    ('e', 12.702, 1.0),
    ('f', 02.228, 0.0),
    ('g', 02.015, 0.0),
    ('h', 06.094, 0.0),
    ('i', 06.966, 0.0),
    ('j', 00.153, 0.0),
    ('k', 00.772, 0.0),
    ('l', 04.025, 0.0),
    ('m', 02.406, 0.0),
    ('n', 06.749, 0.0),
    ('o', 07.507, 0.0),
    ('p', 01.929, 0.0),
    ('q', 00.095, 0.0),
    ('r', 05.987, 0.0),
    ('s', 06.327, 0.0),
    ('t', 09.056, 0.0),
    ('u', 02.758, 0.0),
    ('v', 00.978, 0.0),
    ('w', 02.360, 0.0),
    ('x', 00.150, 0.0),
    ('y', 01.974, 0.0),
    ('z', 00.074, 0.0),
];

#[allow(dead_code)]
pub fn histogram_score(s: &str) -> f64 {
    let mut count = [0usize; 26];
    for c in s.chars() {
        match c {
            'a'..='z' => count[c as usize - 'a' as usize] += 1,
            'A'..='Z' => count[c as usize - 'A' as usize] += 1,
            _ => (),
        }
    }
    let mut diff = 0f64;
    for (&ac, (_ch, ex, variance)) in count.iter().zip(STD_FREQ.iter()) {
        let act = (ac as f64) / s.len() as f64;
        diff += (act * 1000.0 - ex * 10.0).abs().round() * variance;
    }
    diff * 1f64
}

pub fn histogram(s: impl AsRef<str>) -> String {
    let mut count = [0usize; 26];
    for c in s.as_ref().chars() {
        match c {
            'a'..='z' => count[c as usize - 'a' as usize] += 1,
            'A'..='Z' => count[c as usize - 'A' as usize] += 1,
            _ => (),
        }
    }
    String::new()
}

pub fn mono(message: impl AsRef<str>, key: impl AsRef<str>) -> String {
    let key = key.as_ref();
    let mut ret = String::new();
    for ch in message.as_ref().chars() {
        if ch.is_ascii_alphabetic() {
            ret.push(
                key.chars()
                    .nth(ch.to_ascii_lowercase() as usize - 'a' as usize)
                    .unwrap(),
            );
        } else {
            ret.push(ch);
        }
    }
    ret
}

/**
 * Calculates the hamming_dist for `s1` and `s2`
 *
 * Hamming distance is the number of different bits
 */
pub fn hamming_dist(s1: impl AsRef<str>, s2: impl AsRef<str>) -> usize {
    let mut dist = 0usize;
    for cs in s1.as_ref().bytes().zip(s2.as_ref().bytes()) {
        for i in 0..8 {
            let tmp = 1u8 << i;
            if cs.0 & tmp != cs.1 & tmp {
                dist += 1;
            }
        }
    }
    dist
}

struct PairIter<I: Iterator<Item = T>, T: Clone> {
    temp: Option<T>,
    inner: I,
}

impl<I: Iterator<Item = T>, T: Clone> PairIter<I, T> {
    fn new(inner: I) -> Self {
        Self { temp: None, inner }
    }
}

impl<I: Iterator<Item = T>, T: Clone> Iterator for PairIter<I, T> {
    type Item = (T, T);
    fn next(&mut self) -> Option<Self::Item> {
        let first = self.temp.take().or_else(|| self.inner.next())?;
        let second = self.inner.next()?;
        self.temp = Some(second.clone());
        Some((first, second))
    }
}

#[derive(Debug, Default, Clone)]
struct DigramMatrix {
    matrix: [[usize; 26]; 26],
    num_digrams: usize,
}

impl DigramMatrix {
    const ENGLISH: Self = Self {
        num_digrams: 1000,
        matrix: [
            [
                0378, 0413, 0688, 0073, 0183, 1454, 1339, 2048, 0026, 0530, 1168, 0477, 0031, 0374,
                0163, 0172, 0120, 0117, 0144, 0027, 0255, 0016, 0214, 0005, 0057, 0005,
            ],
            [
                1205, 0171, 0530, 1041, 1343, 0010, 0337, 0426, 3556, 0098, 0001, 0026, 0255, 0026,
                0006, 0004, 0002, 0082, 0227, 0003, 0001, 0000, 0000, 0000, 0000, 0004,
            ],
            [
                0012, 1487, 0003, 0005, 0316, 1985, 0871, 1075, 0014, 1087, 0368, 0448, 0119, 0285,
                0074, 0203, 0205, 0060, 0217, 0230, 0205, 0105, 0019, 0012, 0002, 0012,
            ],
            [
                0039, 0442, 0057, 0210, 0088, 1758, 0290, 1277, 0021, 0365, 0195, 0166, 0870, 0546,
                1175, 0224, 0094, 0330, 0036, 0097, 0178, 0064, 0019, 0007, 0001, 0003,
            ],
            [
                0385, 1123, 0286, 0835, 0023, 2433, 1128, 0315, 0002, 0432, 0296, 0699, 0017, 0318,
                0203, 0089, 0255, 0001, 0000, 0099, 0288, 0043, 0022, 0001, 0011, 0064,
            ],
            [
                0692, 1041, 0347, 0465, 0339, 0073, 0509, 0009, 0011, 0064, 1352, 0416, 0079, 0028,
                0067, 0006, 0953, 0006, 0098, 0004, 0052, 0052, 0003, 0011, 0006, 0004,
            ],
            [
                0932, 1053, 0218, 0398, 0550, 0009, 0405, 0006, 0315, 0056, 0005, 0155, 0311, 0065,
                0017, 0191, 0002, 0024, 0057, 0008, 0001, 0039, 0000, 0000, 0007, 0000,
            ],
            [
                1854, 0362, 0686, 0727, 0728, 0160, 0397, 0121, 0015, 0086, 0189, 0121, 0128, 0175,
                0032, 0042, 0100, 0013, 0248, 0027, 0069, 0097, 0001, 0001, 0001, 0001,
            ],
            [
                3075, 0130, 0926, 0485, 0763, 0026, 0015, 0084, 0001, 0013, 0003, 0001, 0074, 0013,
                0002, 0001, 0000, 0005, 0050, 0004, 0000, 0000, 0000, 0000, 0000, 0000,
            ],
            [
                0829, 0124, 0528, 0387, 0624, 0006, 0142, 0010, 0002, 0577, 0253, 0012, 0135, 0023,
                0053, 0019, 0006, 0013, 0425, 0007, 0035, 0020, 0000, 0000, 0000, 0000,
            ],
            [
                0765, 0003, 0151, 0188, 0493, 0008, 0126, 0085, 0005, 0032, 0043, 0003, 0148, 0018,
                0003, 0002, 0031, 0008, 0050, 0003, 0019, 0000, 0000, 0005, 0001, 0000,
            ],
            [
                0651, 0461, 0538, 0794, 0281, 0001, 0023, 0149, 0598, 0149, 0002, 0083, 0163, 0003,
                0001, 0001, 0001, 0000, 0042, 0001, 0000, 0118, 0000, 0000, 0005, 0001,
            ],
            [
                0147, 0405, 0136, 0011, 0101, 0394, 0454, 0543, 0001, 0346, 0091, 0188, 0001, 0138,
                0019, 0136, 0128, 0000, 0005, 0089, 0003, 0005, 0004, 0001, 0000, 0002,
            ],
            [
                0793, 0001, 0565, 0337, 0318, 0009, 0093, 0003, 0001, 0005, 0001, 0004, 0115, 0096,
                0004, 0239, 0001, 0001, 0062, 0090, 0000, 0000, 0000, 0000, 0000, 0000,
            ],
            [
                0237, 0082, 0164, 0488, 0285, 0000, 0006, 0213, 0000, 0065, 0000, 0001, 0096, 0001,
                0146, 0000, 0001, 0000, 0009, 0000, 0000, 0000, 0000, 0000, 0000, 0000,
            ],
            [
                0478, 0106, 0324, 0361, 0123, 0001, 0055, 0474, 0094, 0263, 0001, 0001, 0105, 0016,
                0001, 0137, 0000, 0001, 0012, 0001, 0000, 0001, 0000, 0000, 0000, 0000,
            ],
            [
                0385, 0015, 0148, 0132, 0152, 0066, 0051, 0197, 0228, 0061, 0003, 0000, 0086, 0010,
                0001, 0000, 0025, 0001, 0026, 0000, 0000, 0000, 0000, 0000, 0000, 0000,
            ],
            [
                0361, 0007, 0385, 0222, 0374, 0079, 0035, 0031, 0379, 0015, 0004, 0001, 0001, 0001,
                0002, 0001, 0000, 0000, 0002, 0001, 0000, 0001, 0000, 0000, 0000, 0000,
            ],
            [
                0093, 0017, 0016, 0150, 0029, 0013, 0097, 0008, 0001, 0015, 0007, 0014, 0001, 0024,
                0001, 0025, 0003, 0003, 0000, 0004, 0000, 0000, 0000, 0000, 0000, 0002,
            ],
            [
                0576, 0017, 0146, 0195, 0107, 0002, 0046, 0112, 0001, 0233, 0002, 0002, 0185, 0003,
                0000, 0001, 0000, 0000, 0176, 0011, 0004, 0000, 0000, 0023, 0000, 0000,
            ],
            [
                0825, 0000, 0140, 0071, 0270, 0000, 0001, 0001, 0000, 0000, 0000, 0000, 0002, 0000,
                0000, 0000, 0000, 0000, 0005, 0000, 0000, 0000, 0000, 0000, 0000, 0000,
            ],
            [
                0214, 0001, 0017, 0006, 0098, 0051, 0048, 0003, 0003, 0011, 0001, 0000, 0003, 0002,
                0002, 0001, 0003, 0002, 0006, 0001, 0000, 0000, 0000, 0000, 0000, 0000,
            ],
            [
                0022, 0047, 0030, 0003, 0039, 0000, 0000, 0000, 0004, 0001, 0000, 0026, 0005, 0000,
                0002, 0067, 0000, 0000, 0003, 0000, 0002, 0000, 0003, 0000, 0000, 0000,
            ],
            [
                0052, 0000, 0026, 0054, 0003, 0000, 0000, 0000, 0000, 0000, 0000, 0000, 0059, 0000,
                0000, 0000, 0000, 0000, 0000, 0000, 0000, 0000, 0000, 0000, 0000, 0000,
            ],
            [
                0000, 0000, 0000, 0000, 0000, 0000, 0000, 0000, 0000, 0000, 0000, 0000, 0148, 0000,
                0000, 0000, 0000, 0000, 0000, 0000, 0000, 0000, 0000, 0000, 0000, 0000,
            ],
            [
                0050, 0000, 0025, 0007, 0012, 0000, 0000, 0000, 0001, 0001, 0000, 0000, 0002, 0000,
                0000, 0000, 0000, 0000, 0002, 0000, 0000, 0000, 0000, 0000, 0000, 0003,
            ],
        ],
    };

    fn from(s: &str) -> Self {
        let mut ret = Self::default();
        for (f, s) in PairIter::new(s.chars()) {
            if f.is_ascii_alphabetic() && s.is_ascii_alphabetic() {
                if let Some(f_i) = ORDER.chars().position(|c| f == c) {
                    if let Some(s_i) = ORDER.chars().position(|c| s == c) {
                        ret.matrix[f_i][s_i] += 1;
                        ret.num_digrams += 1;
                    }
                }
            }
        }
        ret
    }

    fn dist(&self, other: &Self) -> f64 {
        self.matrix
            .iter()
            .zip(other.matrix.iter())
            .flat_map(|(s, o)| {
                s.iter().zip(o.iter()).map(|(&s, &o)| {
                    (s as f64) / (self.num_digrams as f64) - (o as f64) / (other.num_digrams as f64)
                })
            })
            .sum()
    }

    fn swap(&mut self, i: usize, j: usize) {
        assert!(i < 26 && j < 26);
        for x in 0..26 {
            let tmp = self.matrix[i][x];
            self.matrix[i][x] = self.matrix[j][x];
            self.matrix[j][x] = tmp;
            let tmp = self.matrix[x][i];
            self.matrix[x][i] = self.matrix[x][j];
            self.matrix[x][j] = tmp;
        }
    }
}

pub fn mono_sub_solve(s: impl AsRef<str>) -> String {
    let s = s.as_ref();
    let mut guess: Vec<_> = ('a'..='z').collect();
    //let mut guess: Vec<_> = ORDER.chars().take(26).collect();
    //guess.shuffle(&mut thread_rng());
    let mut best_guess = guess.clone();
    let mut matrix = DigramMatrix::from(&mono(s, guess.iter().collect::<String>()));
    let mut best_matrix = matrix.clone();
    let mut best = matrix.dist(&DigramMatrix::ENGLISH);
    let mut last_best = best;
    loop {
        for i in 0..26 {
            for j in i+1..26 {
                guess.swap(i, j);
                matrix.swap(i, j);
                let cur = matrix.dist(&DigramMatrix::ENGLISH);
                if cur < best {
                    best = cur;
                    best_matrix = matrix.clone();
                    best_guess = guess.clone();
                } else {
                    //guess.swap(i, j);
                    //matrix.swap(i, j);
                }
            }
        }
        println!("Iteration");
        if best == last_best {
            break mono(s, guess.iter().collect::<String>());
        }
        last_best = best;
    }
}
