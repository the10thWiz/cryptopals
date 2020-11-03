// ABCDEFGHIJKLMNOPQRSTUVWXYZ
//AABCDEFGHIJKLMNOPQRSTUVWXYZ
//BBCDEFGHIJKLMNOPQRSTUVWXYZA
//CCDEFGHIJKLMNOPQRSTUVWXYZAB
//DDEFGHIJKLMNOPQRSTUVWXYZABC
//EEFGHIJKLMNOPQRSTUVWXYZABCD
//FFGHIJKLMNOPQRSTUVWXYZABCDE
//GGHIJKLMNOPQRSTUVWXYZABCDEF
//HHIJKLMNOPQRSTUVWXYZABCDEFG
//IIJKLMNOPQRSTUVWXYZABCDEFGH
//JJKLMNOPQRSTUVWXYZABCDEFGHI
//KKLMNOPQRSTUVWXYZABCDEFGHIJ
//LLMNOPQRSTUVWXYZABCDEFGHIJK
//MMNOPQRSTUVWXYZABCDEFGHIJKL
//NNOPQRSTUVWXYZABCDEFGHIJKLM
//OOPQRSTUVWXYZABCDEFGHIJKLMN
//PPQRSTUVWXYZABCDEFGHIJKLMNO
//QQRSTUVWXYZABCDEFGHIJKLMNOP
//RRSTUVWXYZABCDEFGHIJKLMNOPQ
//SSTUVWXYZABCDEFGHIJKLMNOPQR
//TTUVWXYZABCDEFGHIJKLMNOPQRS
//UUVWXYZABCDEFGHIJKLMNOPQRST
//VVWXYZABCDEFGHIJKLMNOPQRSTU
//WWXYZABCDEFGHIJKLMNOPQRSTUV
//XXYZABCDEFGHIJKLMNOPQRSTUVW
//YYZABCDEFGHIJKLMNOPQRSTUVWX
//ZZABCDEFGHIJKLMNOPQRSTUVWXY

//ABCDEFGHIJKLMNOPQRSTUVWXYZ
const VIRGENE_TABLE: [[char; 26]; 26] = [
    [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ],
    [
        'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
        'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'A',
    ],
    [
        'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
        'U', 'V', 'W', 'X', 'Y', 'Z', 'A', 'B',
    ],
    [
        'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
        'V', 'W', 'X', 'Y', 'Z', 'A', 'B', 'C',
    ],
    [
        'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
        'W', 'X', 'Y', 'Z', 'A', 'B', 'C', 'D',
    ],
    [
        'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W',
        'X', 'Y', 'Z', 'A', 'B', 'C', 'D', 'E',
    ],
    [
        'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
        'Y', 'Z', 'A', 'B', 'C', 'D', 'E', 'F',
    ],
    [
        'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y',
        'Z', 'A', 'B', 'C', 'D', 'E', 'F', 'G',
    ],
    [
        'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
    ],
    [
        'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'A',
        'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    ],
    [
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'A', 'B',
        'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
    ],
    [
        'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'A', 'B', 'C',
        'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K',
    ],
    [
        'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'A', 'B', 'C', 'D',
        'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    ],
    [
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'A', 'B', 'C', 'D', 'E',
        'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    ],
    [
        'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'A', 'B', 'C', 'D', 'E', 'F',
        'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
    ],
    [
        'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'A', 'B', 'C', 'D', 'E', 'F', 'G',
        'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
    ],
    [
        'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
        'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
    ],
    [
        'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
        'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
    ],
    [
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
    ],
    [
        'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K',
        'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    ],
    [
        'U', 'V', 'W', 'X', 'Y', 'Z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
        'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
    ],
    [
        'V', 'W', 'X', 'Y', 'Z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    ],
    [
        'W', 'X', 'Y', 'Z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
        'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
    ],
    [
        'X', 'Y', 'Z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
        'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W',
    ],
    [
        'Y', 'Z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
        'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
    ],
    [
        'Z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
        'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y',
    ],
];

pub fn encrypt(key: &str, message: &str) -> String {
    message
        .chars()
        .filter(char::is_ascii_alphabetic)
        .zip(key.chars().cycle())
        .map(|(m, k)| {
            VIRGENE_TABLE[(k.to_ascii_uppercase() as usize) - ('A' as usize)]
                [(m.to_ascii_uppercase() as usize) - ('A' as usize)]
        })
        .collect::<String>()
}
pub fn decrypt(key: &str, message: &str) -> String {
    message
        .chars()
        .filter(char::is_ascii_alphabetic)
        .zip(key.chars().cycle())
        .map(|(m, k)| {
            (VIRGENE_TABLE[(k.to_ascii_uppercase() as usize) - ('A' as usize)]
                .iter()
                .enumerate()
                .find(|(i, &c)| c == m.to_ascii_uppercase())
                .map(|(i, c)| i as u8)
                .unwrap()
                + ('A' as u8)) as char
        })
        .collect::<String>()
}

pub fn ind_coin(message: &str, shift: usize) -> usize {
    let mut count = 0;
    for i in 0..message.len() {
        if message[i..=i] == message[(i + shift) % message.len()..=(i + shift) % message.len()] {
            count += 1;
        }
    }
    count
}

pub fn break_cipher(key_len: usize, message: &str) -> String {
    let mut key = String::new();
    for i in 0..key_len {
        let (plain, key_part, score) = decrypt_single_letter(
            &message.chars().skip(i).step_by(key_len).collect::<String>(),
            &crate::lang::histogram_score,
        );
        //println!("{}: `{}`", i, &message.chars().skip(i).step_by(key_len).collect::<String>()[0..10]);
        key.push(key_part);
    }
    key //decrypt(&key, message)
}

pub fn decrypt_single_letter(
    data: &str,
    score: &dyn Fn(&str) -> isize,
) -> (String, char, isize) {
    let mut min = isize::max_value();
    let mut k = ' ';
    let mut best = String::new();
    for b in 0..26 {
        let tmp = data
            .chars()
            .filter(char::is_ascii_alphabetic)
            .map(|c| rot(c, b))
            .collect::<String>();
        let tmp_s = score(&tmp);
        if tmp_s < min {
            min = tmp_s;
            best = tmp;
            k = (b + 'A' as u8) as char;
        } else if tmp_s == min {
        }
        //if tmp_s < 20 {
            //k.push((b + 'A' as u8) as char);
        //}
    }
    (best, k, min)
}

pub fn rot(ch: char, rot: u8) -> char {
    (((((ch.to_ascii_lowercase() as u8 - 'a' as u8) + 26u8) - rot) % 26) + 'a' as u8) as char
}
