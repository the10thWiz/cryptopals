
mod file;
mod data;
mod lang;
mod keys;
mod decrypt;
mod open_ssl;

use std::time::Instant;

fn main() {
    println!("-------------");
    let start = Instant::now();
    challenge_2_11();
    println!("-------------\nSuccess: {}ms", start.elapsed().as_millis() as u64);
}

#[allow(dead_code)]
fn challenge_2_11() {
    let data = open_ssl::encryption_oracle(file::File::read_utf8_file("data_lorem").read_bytes());
}

#[allow(dead_code)]
fn challenge_2_10() {
    let data = file::File::read_64_file("data_2_10").read_bytes();
    let key = data::Bytes::read_utf8("YELLOW SUBMARINE");
    let iv = data::Bytes::read_utf8("\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    println!("{}", open_ssl::decrypt_cbc(data, key, iv).to_ascii());
}

#[allow(dead_code)]
fn challenge_2_9() {
    let data = data::Bytes::read_utf8("YELLOW SUBMARINE");
    assert_eq!(data.pad_pkcs7(20).to_ascii(), "YELLOW SUBMARINE\x04\x04\x04\x04");
}

#[allow(dead_code)]
fn challenge_1_8() {
    let ciphertexts = file::File::read_hex_file("data_1_8");
    let mut detected = data::Bytes::zero(0);
    let mut max = 0;
    for text in ciphertexts {
        let cur = decrypt::count_repeats(text.split(16));
        if cur > max {
            max = cur;
            detected = text;
        }
    }
    println!("Ciphered: {}", detected.to_hex());
    println!("Repeats: {}", max);
}

#[allow(dead_code)]
fn challenge_1_7() {
    let data = file::File::read_64_file("data_1_7").read_bytes();
    let key = data::Bytes::read_utf8("YELLOW SUBMARINE");
    println!("{}", open_ssl::decrypt_ecb(data, key).to_ascii());
}

#[allow(dead_code)]
fn challenge_1_6() {
    // Hamming_dist (Step 2)
    assert_eq!(lang::hamming_dist("this is a test", "this is a test"), 0);
    assert_eq!(lang::hamming_dist("this is a test", "wokka wokka!!!"), 37);
    
    let raw = file::File::read_64_file("data_1_6").read_bytes();
    // Guess key length (Step 1)
    println!("Len\tDist\tScore");
    let mut key_size = (0, 1000f64);
    for key_size_guess in 2..40 {
        // Take first and second set of bytes (Step 3)
        let mut chunks = raw.split(key_size_guess);
        let len = chunks.len();
        let first = chunks.remove(0);
        let mut dist = 0;
        for chunk in chunks {
            dist+= lang::hamming_dist(&first.to_ascii(), &chunk.to_ascii());
        }
        let score = dist as f64 / len as f64 / key_size_guess as f64;
        // Take smallest (Step 4)
        if key_size.1 > score {
            key_size.0 = key_size_guess;
            key_size.1 = score;
        }
        println!("{}\t{}\t{}", key_size_guess, dist, score);
    }
    println!("Final:\n{}\t \t{}\n", key_size.0, key_size.1);

    //Break the raw text into parts (Steps 5 & 6)
    let blocks = raw.collate(key_size.0);
    let mut key = String::new();
    for block in blocks {
        //decrypt::decrypt_xor(raw, keys::KeyGen::new(1), lang::score_string);
        let result = decrypt::decrypt_xor(block, keys::KeyGen::new(1), lang::score_string);
        key.push(result.1.get(0));
    }
    println!("Key Guess: {}", key);
    let text = raw ^ data::Bytes::read_utf8(&key);
    println!("Text: {}", text.to_ascii());
}
#[allow(dead_code)]
fn challenge_1_5() {
    let text = data::Bytes::read_utf8("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal");
    let key = data::Bytes::read_utf8("ICE");
    assert_eq!(text.clone() ^ key.clone(), "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");

    let xor = data::Bytes::read_hex("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
    assert_eq!((xor ^ key).to_ascii(), text.to_ascii());
}

#[allow(dead_code)]
fn challenge_1_4() {
    let mut best = (String::default(), data::Bytes::zero(0), 0);

    let raw_iter = file::File::read_hex_file("data_1_4");

    for raw in raw_iter {
        let tmp = decrypt::decrypt_xor(raw, keys::KeyGen::new(1), lang::score_string);
        if tmp.2 > best.2 {
            best = tmp;
        }
    }
    println!("\n{}", best.0);
}
#[allow(dead_code)]
fn challenge_1_3() {
    let raw1 = data::Bytes::read_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    
    println!("{}", decrypt::decrypt_xor(raw1, keys::KeyGen::new(1), lang::score_string).0);
}
#[allow(dead_code)]
fn challenge_1_2() {
    let raw1 = data::Bytes::read_hex("1c0111001f010100061a024b53535009181c");
    let raw2 = data::Bytes::read_hex("686974207468652062756c6c277320657965");

    assert_eq!(raw1 ^ raw2, "746865206b696420646f6e277420706c6179");
}
#[allow(dead_code)]
fn challenge_1_1() {
    let raw1 = data::Bytes::read_hex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    assert_eq!(raw1.to_64(), "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");

    let raw2 = data::Bytes::read_64("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    assert_eq!(raw2.to_hex(), "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_uppercase());
}
