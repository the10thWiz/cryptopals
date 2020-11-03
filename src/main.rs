#![allow(dead_code)]
mod cipher;
mod data;
mod decrypt;
mod file;
mod keys;
mod lang;
mod oracle;
mod random;

use oracle::Oracle;
//use diff_fmt::*;

/**
 * Note: this main only runs one challenge
 * In order to run other challenges, line
 * 20 should be replaced with the revelant
 * function
 */

fn main() {
    challenge_3_22();
    println!("---------- Ok {}", cipher::BLOCK_SIZE);
}

fn challenge_3_22() {
    // Rust doesn't have an easy way to get the seconds since the UNIX_EPOCH, so I just used a random value
    // from the rand module
    let mut rng = random::MersenneGen::new(rand::random());
    let test = rng.extract_number();

    let mut y = test;
    y = y ^ (y >> random::L);
    y = y ^ ((y << random::T) & random::C);
    // println!("{}", print::mask_bin(y, random::B & (random::B << random::S)));
    y = y ^ ((y << random::S) & (random::B ^ (random::B & (random::B << random::S))));
    y = y ^ ((y << random::S) & (random::B & (random::B << random::S)));
    // y = y ^ ((y >> random::U) & random::D);
    println!("{:032b}", random::B);
    println!("{:032b}", random::B << random::S);
    println!("{:032b}", random::B & (random::B << random::S));
    println!(
        "{:032b}",
        random::B ^ (random::B & (random::B << random::S))
    );
    println!();
    //println!("{:032b}", diff(&rng.get_internal(0), &y));
    //println!("{:032b}", diff(&y, &rng.get_internal(0)));
    // println!(
    //     "{}",
    //     print::mask_bin(rng.get_internal(0), random::B & (random::B << random::S))
    // );
    // println!(
    //     "{}",
    //     print::mask_bin(rng.get_internal(0), random::B ^ (random::B & (random::B << random::S)))
    // );
    // println!("{}", print::diff_bin(y, rng.get_internal(0)));
    // println!();
    // println!(
    //     "{}",
    //     print::mask_bin(y, random::B & (random::B << random::S))
    // );
    // println!(
    //     "{}",
    //     print::mask_bin(y >> random::S, random::B & (random::B << random::S))
    // );

    // y = y ^ (((y & random::B) << random::S) & random::B);
}

fn challenge_3_21() {
    let mut rng = random::MersenneGen::new(0);
    for _ in 0..100 {
        println!("{}", rng.extract_number());
    }
}

fn challenge_3_20() {
    let columns_enc = data::Bytes::pivot(oracle::gen_ctr_tests_3_20());
    let mut columns_plain = Vec::with_capacity(columns_enc.len());

    for column in columns_enc {
        let (plain, _key, _score) =
            decrypt::decrypt_xor(column, keys::KeyGen::new(1), &lang::count_invalid_letters);
        columns_plain.push(plain);
    }
    let rows_plain = data::Bytes::pivot(columns_plain);
    for row in rows_plain {
        println!("{}", row);
    }
}

fn challenge_3_19() {
    /*
     * Skipped this challenge through no fault of my own
     * When presented with this challenge, I choose to use
     * the method outlined in 3.20, before reading 3.20
     *
     * It turns out that 3.20's method doesn't work for 3.19,
     * beacuse there aren't enough samples in 3.19 (only 40)
     */
    let columns_enc = data::Bytes::pivot(oracle::gen_ctr_tests_3_19());
    let mut columns_plain = Vec::with_capacity(columns_enc.len());

    for column in columns_enc {
        let (plain, _key, _score) =
            decrypt::decrypt_xor(column, keys::KeyGen::new(1), &lang::count_invalid_letters);
        columns_plain.push(plain);
    }
    let rows_plain = data::Bytes::pivot(columns_plain);
    for row in rows_plain {
        println!("{}", row);
    }
}

fn challenge_3_18() {
    let data = data::Bytes::read_64(
        "L77na/nrFsKvynd6HzOoG7GHTLXsTVu9qvY/2syLXzhPweyyMTJULu/6/kXX0KSvoOLSFQ==",
    );
    let mut stream = cipher::CTRstream::new(0, data::Bytes::read_utf8("YELLOW SUBMARINE"));
    println!("Decrypted: {}", stream.crypt(data));
}

fn challenge_3_17() {
    let oracle = oracle::CBCPaddingOracle::new();

    let enc = oracle.encrypt();

    let mut last = enc.0.clone();
    let mut known = data::Bytes::zero(0);
    for block in enc.1.split(16) {
        known += decrypt::attack_byte_padding(last.clone(), block.clone(), &oracle);
        last = block;
    }
    oracle.print_raw(enc);
    println!("{}", known.trim_pkcs7());
}

fn challenge_2_16() {
    let oracle = oracle::ProfileCBCOracle::new();

    // edit string to include ";admin=true;"
    // step one, create input to block align
    let start_padding =
        data::Bytes::read_utf8("a") * (16 - "comment1=cooking%20MCs;userdata=".len() % 16);

    // step two, create encrpted version
    let enc = oracle.encode_profile(start_padding.clone() + data::Bytes::zero(16));
    // enc = blocks + zero*16 + ";comment2=%20lik" + "e%20a%20pound%20of%20bacon"
    // therefore, I need to edit zero block of the cipertext, to edit the following block

    // target to edit, and result to get
    let target = data::Bytes::read_utf8(";comment2=%20like%20a%20pound%20of%20bacon").truncate(16);
    let result = data::Bytes::read_utf8(";admin=true;aaaaaaaaa").truncate(16);
    // In theory, if I swap the zero and result^target block, it will cause all the nessecary 1 bit errors
    let swapped = enc.replace_block(
        (target ^ result).to_bytes(),
        ("comment1=cooking%20MCs;userdata=".len() + start_padding.len()) / 16,
    );

    assert_eq!(oracle.get_role(enc), oracle::Role::USER);
    assert_eq!(oracle.get_role(swapped), oracle::Role::ADMIN);
}

fn challenge_2_15() {
    // I'm supposed to write a function to trim PKCS#7 padding,
    // but I already wrote it. It's `data::Bytes::trim_pkcs7()`
    // It doesn't panic if there isn't padding, it just assumes
    // there wasn't anything to remove
}

fn challenge_2_14() {
    let oracle = oracle::RandomOracle::new();

    // Calculate prefix size (in blocks)
    let duplicate_location = oracle.encrypt(data::Bytes::read_utf8("a") * 48).split(16);
    let mut num_blocks = 0;
    for i in 1..duplicate_location.len() {
        if duplicate_location[i - 1] == duplicate_location[i] {
            num_blocks = i - 1;
            break;
        }
    }

    let mut prefix_len = 16;
    for i in 0..16 {
        let mut test = data::Bytes::read_utf8("a") * 48;
        test[i] = 0u8;
        let enc = oracle.encrypt(test).split(16);
        if duplicate_location[num_blocks] != enc[num_blocks] {
            prefix_len = i;
            break;
        }
    }
    // Now I can ignore prefix blocks
    let mut known = data::Bytes::zero(0);
    let len = oracle.encrypt(known.clone()).len();

    for _ in num_blocks * 16 - prefix_len..len {
        known += decrypt::decrypt_byte_2(&oracle, &known, num_blocks * 16 - prefix_len);
    }
    println!("{}", known);
}

fn challenge_2_13() {
    let oracle = oracle::ProfileOracle::new();

    // Guess oracle padding = pkcs_7
    // Therefore, final part should be "admin"+padding to BLOCK_SIZE
    let final_plain = data::Bytes::read_utf8("admin").pad_pkcs7(16);
    // first part is "email=", so add padding to make it one block:
    let start_padding = data::Bytes::read_utf8("a") * (16 - "email=".len());

    // create block
    let final_cipher = oracle.encode_profile(start_padding + final_plain);
    // create new block, with "user"+padding as final block
    let email_padding = data::Bytes::read_utf8("a") * (16 - "email=&uid=10&role=".len() % 16);
    let new_cipher = oracle.encode_profile(email_padding);
    // oracle.print_raw(final_cipher.clone());
    // oracle.print_raw(data::Bytes::from_bytes(&final_cipher[16..32]));
    let admin_profile = new_cipher.replace_block(&final_cipher[16..32], new_cipher.len() / 16 - 1);
    oracle.print_raw(admin_profile.clone());

    assert_eq!(oracle.get_role(admin_profile), oracle::Role::ADMIN);
}

fn challenge_2_12() {
    let oracle = oracle::OracleSimple::new();

    // 1. Block Size
    let len = oracle.encrypt(data::Bytes::zero(0)).len();

    // 2. ECB mode
    let vec = oracle.encrypt(data::Bytes::read_utf8("a") * 64).split(16);
    if vec[0] != vec[1] {
        panic!("Oracle doesn't use ECB");
    }

    // 3. 1 byte short
    let mut known = data::Bytes::zero(0);
    for _ in 0..len {
        known += decrypt::decrypt_byte(&oracle, &known, 0);
    }
    println!("{}", known);
}

fn challenge_2_11() {
    for i in 0..1000 {
        println!("Trial {}", i);
        let (data, cbc) = oracle::encryption_oracle(data::Bytes::read_utf8("a") * 100);
        // println!("data: {}\n{}", data, if cbc {"CBC"} else {"ECB"});

        let vec = data.split(16);
        // println!("{}\n{}", vec[1], vec[2]);
        if vec[1] == vec[2] {
            assert_eq!(cbc, false);
        } else {
            assert_eq!(cbc, true);
        }
    }
}

fn challenge_2_10() {
    let data = file::File::read_64_file("data_2_10").read_bytes();
    let key = data::Bytes::read_utf8("YELLOW SUBMARINE");
    let iv =
        data::Bytes::read_utf8("\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    println!("{}", cipher::aes_cbc_de(data, key, iv));
}

fn challenge_2_9() {
    let data = data::Bytes::read_utf8("YELLOW SUBMARINE");
    assert_eq!(
        data.pad_pkcs7(20).to_utf8(),
        "YELLOW SUBMARINE\x04\x04\x04\x04"
    );
}

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
    println!("Ciphered: {:X}", detected);
    println!("Repeats: {}", max);
}

fn challenge_1_7() {
    let data = file::File::read_64_file("data_1_7").read_bytes();
    let key = data::Bytes::read_utf8("YELLOW SUBMARINE");
    println!("{}", cipher::aes_ecb_de(data, key));
}

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
            dist += lang::hamming_dist(&first.to_utf8(), &chunk.to_utf8());
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
        let result = decrypt::decrypt_xor(block, keys::KeyGen::new(1), &lang::score_string);
        key.push(result.1.get(0));
    }
    println!("Key Guess: {}", key);
    let text = raw ^ data::Bytes::read_utf8(&key);
    println!("Text: {}", text);
}

fn challenge_1_5() {
    let text = data::Bytes::read_utf8(
        "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal",
    );
    let key = data::Bytes::read_utf8("ICE");
    assert_eq!(text.clone() ^ key.clone(), "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");

    let xor = data::Bytes::read_hex("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
    assert_eq!((xor ^ key).to_utf8(), text.to_utf8());
}

fn challenge_1_4() {
    let mut best = (data::Bytes::default(), data::Bytes::default(), 0.0);

    let raw_iter = file::File::read_hex_file("data_1_4");

    for raw in raw_iter {
        let tmp = decrypt::decrypt_xor(raw, keys::KeyGen::new(1), &lang::score_string);
        if tmp.2 > best.2 {
            best = tmp;
        }
    }
    println!("\n{}", best.0);
}

fn challenge_1_3() {
    let raw1 = data::Bytes::read_hex(
        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
    );

    println!(
        "{}",
        decrypt::decrypt_xor(raw1, keys::KeyGen::new(1), &lang::score_string).0
    );
}

fn challenge_1_2() {
    let raw1 = data::Bytes::read_hex("1c0111001f010100061a024b53535009181c");
    let raw2 = data::Bytes::read_hex("686974207468652062756c6c277320657965");

    assert_eq!(raw1 ^ raw2, "746865206b696420646f6e277420706c6179");
}

fn challenge_1_1() {
    let raw1 = data::Bytes::read_hex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    assert_eq!(
        raw1.to_64(),
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
    );

    let raw2 =
        data::Bytes::read_64("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    assert_eq!(raw2.to_hex(), "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_uppercase());
}
