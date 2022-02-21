#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

mod cipher;
mod data;
mod decrypt;
mod file;
mod keys;
mod lang;
mod mac;
mod oracle;
mod random;

mod comms;
mod passwd;

use cryptopals::cipher::diffie::diffie_hellman_a;
use num_bigint::BigUint;
use oracle::Oracle;
use rand::random;
use sha::sha256;
use std::hash::Hasher;
use std::iter::FromIterator;
use std::sync::mpsc::*;

use crate::passwd::PasswdStore;

fn fast_exp(mut base: BigUint, mut exp: BigUint, modulus: BigUint) -> BigUint {
    let mut product = BigUint::from(1usize);
    while exp > BigUint::from(0usize) {
        if exp.bit(0) {
            product = (product * &base) % &modulus;
        }
        base = base.pow(2) % &modulus;
        exp = exp >> 1;
    }
    product
}

/**
 * Note: this main only runs one challenge
 * In order to run other challenges, line
 * 20 should be replaced with the revelant
 * function
 */

fn main() {
    let start = std::time::Instant::now();
    //challenge_5_37();
    println!("{}", fast_exp(BigUint::from(5usize), BigUint::from(177usize), BigUint::from(11usize)));
    println!("Completed in {} mS", start.elapsed().as_millis());
}

fn challenge_5_39() {
    // RSA
}

fn challenge_5_38() {
    // Offline dictionary attack
    //
    // From my understanding, by mitm-ing the connection, it is possible to get your hands on
    // enough information to make an offline attack on the hash, while also allowing the client
    // to still authenticate with the server
}

fn challenge_5_37() {
    // SRP - Secure Remote Protocol
    enum Message {
        SendD { email: String, d_a: BigUint },
        SendSalt { salt: data::Bytes, d_b: BigUint },
        SendHMAC { sig: Vec<u8> },
        Complete { res: bool },
    }

    let mut store = PasswdStore::new();
    store.add_user("admin@me.net".to_string(), "admin");
    let client = |tx: Sender<Message>, rx: Receiver<Message>| {
        let (m_a, d_a) = diffie_hellman_a();
        println!("Sending SendD");
        tx.send(Message::SendD {
            email: "admin@me.net".to_string(),
            d_a: d_a.clone(),
        })
        .unwrap();
        println!("Waiting on Salt");
        if let Message::SendSalt { salt, d_b } = rx.recv().unwrap() {
            println!("Recieved Salt");
            let mut hasher = sha256::Sha256::default();
            hasher.write(&d_a.to_bytes_le());
            hasher.write(&d_b.to_bytes_le());
            let u = BigUint::from(hasher.finish().to_le());

            let mut hasher = sha256::Sha256::default();
            hasher.write(&salt);
            hasher.write("admin".as_bytes());
            let x = BigUint::from(hasher.finish().to_le());

            let t =
                BigUint::from(3usize) * cipher::diffie::NIST_G.modpow(&x, &*cipher::diffie::NIST_P);

            let s = (&d_b - &t).modpow(&(m_a + (&u * &x)), &*cipher::diffie::NIST_P);

            let mut hasher = sha256::Sha256::default();
            hasher.write(&s.to_bytes_le());
            let k_ = hasher.finish().to_le();
            let mac = mac::HMAC::key(salt);
            let sig = mac.sign(format!("{}", k_));
            println!("Sending HMAC");
            tx.send(Message::SendHMAC { sig }).unwrap();
            println!("Waiting on Complete");
            if let Message::Complete { res } = rx.recv().unwrap() {
                if !res {
                    panic!("Failed to authenticate");
                }
                println!("Result: {}", res);
            }
        }
    };
    let server = move |tx: Sender<Message>, rx: Receiver<Message>| {
        println!("Waiting on SendD");
        if let Message::SendD { email, d_a } = rx.recv().unwrap() {
            println!("Recieved SendD");
            let tmp = store.db.get(&email).unwrap();
            let (m_b, d_b_old) = diffie_hellman_a();
            let x = BigUint::from_bytes_le(tmp.hash.to_bytes()); // Same hash
            let v = cipher::diffie::NIST_G.modpow(&x, &*cipher::diffie::NIST_P);
            let d_b = d_b_old.clone() + (v.clone() * BigUint::from(3usize));
            println!("Sending Salt");
            tx.send(Message::SendSalt {
                salt: tmp.salt.clone(),
                d_b: d_b.clone(),
            })
            .unwrap();
            let mut hasher = sha256::Sha256::default();
            hasher.write(&d_a.to_bytes_le());
            hasher.write(&d_b.to_bytes_le());
            let u = BigUint::from(hasher.finish().to_le());

            let s = (d_a * v.clone().modpow(&u, &*cipher::diffie::NIST_P))
                .modpow(&m_b, &*cipher::diffie::NIST_P);
            println!("s: {}", s);

            let mut hasher = sha256::Sha256::default();
            hasher.write(&s.to_bytes_le());
            let k_ = hasher.finish().to_le();
            println!("Waiting on HMAC");
            if let Message::SendHMAC { sig } = rx.recv().unwrap() {
                let mac = mac::HMAC::key(tmp.salt.clone());
                let res = mac.verify(format!("{}", k_), &data::Bytes::from_vec(sig));
                println!("Sending Complete");
                tx.send(Message::Complete { res }).unwrap();
            }
        }
    };
    // Verify the server works - Can't due to lifetimes
    //comms::comm_channel(client, server);
    let attacker = |tx: Sender<Message>, rx: Receiver<Message>| {
        let (m_a, d_a) = diffie_hellman_a();
        println!("Sending SendD");
        tx.send(Message::SendD {
            email: "admin@me.net".to_string(),
            d_a: cipher::diffie::NIST_P.clone() * BigUint::from(1usize),
            // s = 0 (regaurdless of the size of the multiplier)
            //d_a: 0usize.into(),
            // s = 0
            //d_a: d_a.clone(),
            // Requires actual password
        })
        .unwrap();
        println!("Waiting on Salt");
        if let Message::SendSalt { salt, d_b } = rx.recv().unwrap() {
            println!("Recieved Salt");
            let mut hasher = sha256::Sha256::default();
            hasher.write(&d_a.to_bytes_le());
            hasher.write(&d_b.to_bytes_le());
            let u = BigUint::from(hasher.finish().to_le());

            let mut hasher = sha256::Sha256::default();
            hasher.write(&salt);
            hasher.write("admin".as_bytes());
            let x = BigUint::from(hasher.finish().to_le());

            let t =
                BigUint::from(3usize) * cipher::diffie::NIST_G.modpow(&x, &*cipher::diffie::NIST_P);

            let s = (&d_b - &t).modpow(&(m_a + (&u * &x)), &*cipher::diffie::NIST_P);
            let s: BigUint = 0usize.into();

            let mut hasher = sha256::Sha256::default();
            hasher.write(&s.to_bytes_le());
            let k_ = hasher.finish().to_le();
            let mac = mac::HMAC::key(salt);
            let sig = mac.sign(format!("{}", k_));
            println!("Sending HMAC");
            tx.send(Message::SendHMAC { sig }).unwrap();
            println!("Waiting on Complete");
            if let Message::Complete { res } = rx.recv().unwrap() {
                if !res {
                    panic!("Failed to authenticate");
                }
                println!("Result: {}", res);
            }
        }
    };
    comms::comm_channel(attacker, server);
}

//#[test]
fn challenge_5_36() {
    // SRP - Secure Remote Protocol
    enum Message {
        SendD { email: String, d_a: BigUint },
        SendSalt { salt: data::Bytes, d_b: BigUint },
        SendHMAC { sig: Vec<u8> },
        Complete { res: bool },
    }

    let mut store = PasswdStore::new();
    store.add_user("admin@me.net".to_string(), "admin");
    let client = |tx: Sender<Message>, rx: Receiver<Message>| {
        let (m_a, d_a) = diffie_hellman_a();
        println!("Sending SendD");
        tx.send(Message::SendD {
            email: "admin@me.net".to_string(),
            d_a: d_a.clone(),
        })
        .unwrap();
        println!("Waiting on Salt");
        if let Message::SendSalt { salt, d_b } = rx.recv().unwrap() {
            println!("Recieved Salt");
            let mut hasher = sha256::Sha256::default();
            hasher.write(&d_a.to_bytes_le());
            hasher.write(&d_b.to_bytes_le());
            let u = BigUint::from(hasher.finish().to_le());

            let mut hasher = sha256::Sha256::default();
            hasher.write(&salt);
            hasher.write("admin".as_bytes());
            let x = BigUint::from(hasher.finish().to_le());

            let t =
                BigUint::from(3usize) * cipher::diffie::NIST_G.modpow(&x, &*cipher::diffie::NIST_P);

            let s = (&d_b - &t).modpow(&(m_a + (&u * &x)), &*cipher::diffie::NIST_P);

            let mut hasher = sha256::Sha256::default();
            hasher.write(&s.to_bytes_le());
            let k_ = hasher.finish().to_le();
            let mac = mac::HMAC::key(salt);
            let sig = mac.sign(format!("{}", k_));
            println!("Sending HMAC");
            tx.send(Message::SendHMAC { sig }).unwrap();
            println!("Waiting on Complete");
            if let Message::Complete { res } = rx.recv().unwrap() {
                if !res {
                    panic!("Failed to authenticate");
                }
                println!("Result: {}", res);
            }
        }
    };
    let server = move |tx: Sender<Message>, rx: Receiver<Message>| {
        println!("Waiting on SendD");
        if let Message::SendD { email, d_a } = rx.recv().unwrap() {
            println!("Recieved SendD");
            let tmp = store.db.get(&email).unwrap();
            let (m_b, d_b_old) = diffie_hellman_a();
            let x = BigUint::from_bytes_le(tmp.hash.to_bytes()); // Same hash
            let v = cipher::diffie::NIST_G.modpow(&x, &*cipher::diffie::NIST_P);
            let d_b = d_b_old.clone() + (v.clone() * BigUint::from(3usize));
            println!("Sending Salt");
            tx.send(Message::SendSalt {
                salt: tmp.salt.clone(),
                d_b: d_b.clone(),
            })
            .unwrap();
            let mut hasher = sha256::Sha256::default();
            hasher.write(&d_a.to_bytes_le());
            hasher.write(&d_b.to_bytes_le());
            let u = BigUint::from(hasher.finish().to_le());

            let s = (d_a * v.clone().modpow(&u, &*cipher::diffie::NIST_P))
                .modpow(&m_b, &*cipher::diffie::NIST_P);

            let mut hasher = sha256::Sha256::default();
            hasher.write(&s.to_bytes_le());
            let k_ = hasher.finish().to_le();
            println!("Waiting on HMAC");
            if let Message::SendHMAC { sig } = rx.recv().unwrap() {
                let mac = mac::HMAC::key(tmp.salt.clone());
                let res = mac.verify(format!("{}", k_), &data::Bytes::from_vec(sig));
                println!("Sending Complete");
                tx.send(Message::Complete { res }).unwrap();
            }
        }
    };
    comms::comm_channel(client, server);
}

#[test]
fn challenge_5_35() {
    // Is this worth it?
    // The challenge is to negotiate p and g
    // Examples:
    // - g = 1; this means that the base is 1, so the resulting values will always be one
    // - g = p; g % p = 0
    // - g = p - 1; g % p = 1
    //
    // In all cases, these resulted in bad bases, that create easily broken A and B
}

#[allow(non_snake_case)]
#[test]
fn challenge_5_34() {
    enum Message {
        SendD(BigUint),
        Message(data::Bytes, data::Bytes),
    }
    // Working Normally
    let a = |tx: Sender<Message>, rx: Receiver<Message>| {
        let (a, A) = cipher::diffie::diffie_hellman_a();
        tx.send(Message::SendD(A)).unwrap();
        if let Message::SendD(B) = rx.recv().unwrap() {
            let sb = cipher::diffie::diffie_hellman_key(a, B);
            let key = data::Bytes::zero(BLOCK_SIZE) ^ data::Bytes::from_bytes(&sb.to_bytes_be());
            let message = data::Bytes::read_utf8("Simple Msg").pad_pkcs7(BLOCK_SIZE);
            let iv = data::Bytes::rand(BLOCK_SIZE);
            let enc = cipher::aes_cbc_en(message.clone(), key.clone(), iv.clone());
            tx.send(Message::Message(iv.clone(), enc)).unwrap();
            if let Message::Message(iv, enc) = rx.recv().unwrap() {
                let new_message = cipher::aes_cbc_de(enc, key, iv);
                println!("A -> {}", new_message);
                assert_eq!(message, new_message);
            }
        }
    };
    let b = |tx: Sender<Message>, rx: Receiver<Message>| {
        let (b, B) = cipher::diffie::diffie_hellman_a();
        if let Message::SendD(A) = rx.recv().unwrap() {
            tx.send(Message::SendD(B)).unwrap();
            let sb = cipher::diffie::diffie_hellman_key(b, A);
            let key = data::Bytes::zero(BLOCK_SIZE) ^ data::Bytes::from_bytes(&sb.to_bytes_be());
            if let Message::Message(iv, enc) = rx.recv().unwrap() {
                let message = cipher::aes_cbc_de(enc, key.clone(), iv).trim_pkcs7();
                println!("B -> {}", message);
                let iv = data::Bytes::rand(BLOCK_SIZE);
                let enc = cipher::aes_cbc_en(message.pad_pkcs7(BLOCK_SIZE), key, iv.clone());
                tx.send(Message::Message(iv, enc)).unwrap();
            }
        }
    };
    comms::comm_channel(a, b);
    // MitM attack (Note that a & b are reused, since they don't change)
    let m = |atx: Sender<Message>,
             arx: Receiver<Message>,
             btx: Sender<Message>,
             brx: Receiver<Message>| {
        let _u = if let Message::SendD(u) = arx.recv().unwrap() {
            u
        } else {
            panic!()
        };
        btx.send(Message::SendD(cipher::diffie::p_bytes())).unwrap();
        let _u = if let Message::SendD(u) = brx.recv().unwrap() {
            u
        } else {
            panic!()
        };
        atx.send(Message::SendD(cipher::diffie::p_bytes())).unwrap();
        let (iv, m) = if let Message::Message(iv, m) = arx.recv().unwrap() {
            (iv, m)
        } else {
            panic!()
        };
        println!(
            "M -> {}",
            cipher::aes_cbc_de(m.clone(), data::Bytes::zero(BLOCK_SIZE), iv.clone())
        );
        btx.send(Message::Message(iv, m)).unwrap();
        let (iv, m) = if let Message::Message(iv, m) = brx.recv().unwrap() {
            (iv, m)
        } else {
            panic!()
        };
        println!(
            "M -> {}",
            cipher::aes_cbc_de(m.clone(), data::Bytes::zero(BLOCK_SIZE), iv.clone())
        );
        atx.send(Message::Message(iv, m)).unwrap();
    };
    comms::comm_channel_mitm(a, b, m);
}

#[test]
#[allow(non_snake_case)]
fn challenge_5_33() {
    let (a, A) = cipher::diffie::diffie_hellman_weak_a();
    let (b, B) = cipher::diffie::diffie_hellman_weak_a();
    let sa = cipher::diffie::diffie_hellman_weak_key(a, B);
    let sb = cipher::diffie::diffie_hellman_weak_key(b, A);
    assert_eq!(sa, sb);
    let (a, A) = cipher::diffie::diffie_hellman_a();
    let (b, B) = cipher::diffie::diffie_hellman_a();
    let sa = cipher::diffie::diffie_hellman_key(a, B);
    let sb = cipher::diffie::diffie_hellman_key(b, A);
    assert_eq!(sa, sb);
}

/// Test to see if a timing difference could be detected without
/// an artificial delay
fn challenge_4_32_2() {
    let hmac = mac::HMAC::init(1);
    let file = "test";
    let actual_sig = data::Bytes::from_vec(hmac.sign(file));
    let mut times = [0; 20];
    let num_trials = 100;
    for i in 0..20 {
        let mut copied_sig = actual_sig.clone();
        copied_sig[i] += 0;
        for _ in 0..num_trials {
            let start = std::time::Instant::now();
            //hmac.verify(file, &copied_sig);
            mac::weak_compare(&actual_sig, &copied_sig);
            times[i] += start.elapsed().as_nanos();
        }
        //times[i] /= num_trials;
    }
    for (i, n) in times.iter().enumerate() {
        println!("Steps: {}, time: {}", i, n);
    }
}

fn challenge_4_32() {
    use sha::utils::DigestExt;
    let hmac = mac::HMAC::init(1);
    let file = "test";
    let start = std::time::Instant::now();
    hmac.weak_verify(file, &data::Bytes::from_vec(hmac.sign(file)));
    println!("Normal Verify time: {}ns", start.elapsed().as_nanos());
    assert!(hmac.verify(file, &data::Bytes::from_vec(hmac.sign(file))));
    let real_sig = hmac.sign(file);
    println!(
        "Esitmated time: {}ms",
        real_sig
            .iter()
            .enumerate()
            .map(|(i, &b)| i * hmac.get_millis() as usize * b as usize)
            .sum::<usize>()
    );
    println!("Correct Signature: {:X}", data::Bytes::from_vec(real_sig));
    let mut signature = data::Bytes::zero(sha::sha1::Sha1::default_len());
    let mut i = 0;
    loop {
        // Timing accuracy could be increased by averaging multiple calls.
        // However, this likely isn't worth it while using std::thread::sleep,
        // since it doesn't seem to be more accurate than 1ms anyway. At this
        // point, these time differences could be significantly impacted by other
        // processes running concurrently.
        //
        // This could be improved, esp. since the base verify (without sleep)
        // takes less than .02ms, but I would need to write my own sleep method
        // to have a reasonably good chance to break it.
        let time = std::time::Instant::now();
        let correct = hmac.weak_verify(file, &signature);
        let mut elapsed = time.elapsed().as_millis();
        //if elapsed as u64 > hmac.get_millis() * 10 {
        //elapsed += hmac.get_millis() as u128 / 2;
        //}
        if correct {
            break;
        } else {
            let key = (elapsed / hmac.get_millis() as u128).min(19);
            if i != key {
                println!("Current Signature: {:X}", signature);
            }
            i = key;
            signature[key as usize] = signature[key as usize].wrapping_add(1u8);
        }
    }
    println!("Current Signature: {:X}", signature);
    assert!(hmac.verify(file, &signature));
}

fn challenge_4_31() {
    use sha::utils::DigestExt;
    let hmac = mac::HMAC::init(100);
    let file = "test";
    assert!(hmac.verify(file, &data::Bytes::from_vec(hmac.sign(file))));
    let real_sig = hmac.sign(file);
    println!(
        "Esitmated time: {}ms",
        real_sig
            .iter()
            .enumerate()
            .map(|(i, &b)| i * 100 * b as usize)
            .sum::<usize>()
    );
    println!("Correct Signature: {:X}", data::Bytes::from_vec(real_sig));
    let mut signature = data::Bytes::zero(sha::sha1::Sha1::default_len());
    let mut i = 0;
    loop {
        let time = std::time::Instant::now();
        let correct = hmac.weak_verify(file, &signature);
        let elapsed = time.elapsed().as_millis() + 50;
        if correct {
            break;
        } else {
            let key = elapsed / 100;
            if i != key {
                println!("Current Signature: {:X}", signature);
            }
            i = key;
            signature[key as usize] = signature[key as usize].wrapping_add(1u8);
        }
    }
    assert!(hmac.verify(file, &signature));
}

fn challenge_4_30() {
    use sha::utils::DigestExt;
    let key = mac::SecrectDigest::md4();
    let message = file::File::read_hex_file("data_1_4").next().unwrap();
    let mac = key.sign(&message);
    println!("mac len: {}", mac.len());
    assert!(key.verify(&message, &mac));
    // cheap method to just get the key size
    // This could just be trial and error, since there are only 64 possible values (since the
    // message is padded to 64 bytes)
    let key_len = key.len();

    let padded_message = pad_md4(data::Bytes::zero(key_len) + message.clone());
    //println!("{:X?}", padded_message.split(8));
    let mut padded_message = padded_message.truncate_start(key_len);
    println!("hasher size: {}", std::mem::size_of::<md4::Md4>());
    let state: Vec<u32> = mac
        .split(4)
        .into_iter()
        .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
        .collect();
    println!("Collected Mac: {:X?}", mac);
    println!("Collected State: {:X?}", state);
    let mut broken_hasher: md4::Md4 = unsafe {
        let mut md4 = md4::Md4::default();
        //use digest::Digest;
        md4.update(data::Bytes::zero(0).to_bytes());
        md4.update(padded_message.to_bytes());
        let mut raw: [u32; 24] = std::mem::transmute(md4);
        println!("{:X?}{:X?}", &raw[0..2], &raw[20..]);
        //println!("Raw memory");
        const S: [u32; 4] = [0x6745_2301, 0xEFCD_AB89, 0x98BA_DCFE, 0x1032_5476];
        for r in raw.iter_mut() {
            //println!(" {:X}", r);
        }
        //let message_len: [u32; 2] = std::mem::transmute((key_len + message.len()) as u64);
        //raw[0] = message_len[0];
        //raw[1] = message_len[1];
        println!(
            "message_len: {:X?}, {:X}",
            &raw[0..2],
            key_len + message.len()
        );
        raw[20] = state[0];
        raw[21] = state[1];
        raw[22] = state[2];
        raw[23] = state[3];
        std::mem::transmute(raw)
    };
    use digest::Digest;
    let addition = data::Bytes::read_utf8(";admin=true;");
    //broken_hasher.update(addition.to_bytes());
    //padded_message += addition;

    unsafe {
        let mut md4: md4::Md4 = std::mem::transmute_copy(&broken_hasher);
        let mut gen_arr: () = Default::default();
        use digest::FixedOutputDirty;
        //md4.finalize_into_dirty(&mut gen_arr);
        let raw: [u32; 24] = std::mem::transmute(md4);
        println!("Copy:");
        println!("{:X} => {:X?}", padded_message, &raw[20..]);
        println!("len: {:X?}", &raw[0..2]);
    }

    let new_mac = data::Bytes::from_bytes(&broken_hasher.finalize()[..]);
    println!("mac: {:X}", new_mac);
    println!("mes: {:X}", key.sign(&padded_message));
    //assert_eq!(key.sign(&padded_message), &new_mac);

    //// Trim fake key
    //let padded_message = padded_message.truncate_start(key_len);
    ////println!("{}", padded_message);
    ////assert_eq!(new_mac, mac);
    //assert_eq!(key.sign(&padded_message), &new_mac);
}

fn pad_md4(message: data::Bytes) -> data::Bytes {
    let mut ret = data::Bytes::empty();
    let mut buffer: block_buffer::BlockBuffer<digest::consts::U64> =
        block_buffer::BlockBuffer::default();
    buffer.input_block(message.to_bytes(), |b| ret += &b[..]);
    buffer.len64_padding_le(message.len() as u64, |b| ret += &b[..]);
    ret
}

#[test]
fn challenge_4_29() {
    use sha::utils::DigestExt;
    let key = mac::SecrectDigest::md4();
    let message = file::File::read_hex_file("data_1_4").next().unwrap();
    let mac = key.sign(&message);
    println!("mac len: {}", mac.len());
    assert!(key.verify(&message, &mac));
    // cheap method to just get the key size
    // This could just be trial and error, since there are only 64 possible values (since the
    // message is padded to 64 bytes)
    let key_len = key.len();

    let state: Vec<u32> = mac
        .split(4)
        .into_iter()
        .map(|b| u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
        .collect();
    // the zero bytes added to the message represent the size of the key
    let (mut padded_message, mut sha1_gen) =
        sha1_pad_bytes(data::Bytes::zero(key_len) + message.clone());
    sha1_gen.0 = [state[0], state[1], state[2], state[3], state[4]];
    println!("Padded len: {}", padded_message.len());
    println!("{:X}", padded_message);

    let addition = data::Bytes::read_utf8("Some new data");
    let blocks_in_message = padded_message.len() / 64;
    padded_message += addition;
    use sha::utils::*;
    // .skip(n), n is the number of blocks in the original message
    for block in padded_message
        .to_vec()
        .pad_blocks(64, sha::sha1::ops::pad)
        .skip(blocks_in_message)
    // number of blocks in the original message
    {
        sha::sha1::ops::digest_block(&mut sha1_gen.0, &block[..]);
    }
    let new_mac = data::Bytes::from_vec(sha1_gen.to_bytes()); // Without the for loop to add new sha1 blocks

    // Trim fake key
    let padded_message = padded_message.truncate_start(key_len);
    //println!("{}", padded_message);
    //assert_eq!(new_mac, mac);
    assert_eq!(key.sign(&padded_message), &new_mac);
}

fn sha1_pad_bytes(message: data::Bytes) -> (data::Bytes, sha::sha1::Sha1) {
    use sha::utils::*;
    let mut padded_message = data::Bytes::zero(0);
    let mut sha1 = sha::sha1::Sha1::default();
    let padding_vec = message.to_vec();
    let padded_blocks = padding_vec.pad_blocks(64, sha::sha1::ops::pad);
    for block in padded_blocks {
        sha::sha1::ops::digest_block(&mut sha1.0, &block[..]);
        padded_message += data::Bytes::from_vec(block);
    }
    (padded_message, sha1)
}

#[test]
fn challenge_4_28() {
    let key = mac::SecrectDigest::sha1();
    let mut message = file::File::read_hex_file("data_1_4").next().unwrap();
    let mac = key.sign(&message);
    assert!(key.verify(&message, &mac));
    message[2] ^= 1;
    assert!(!key.verify(&message, &mac));
    println!("Verified message");
}

#[test]
fn challenge_4_27() {
    let oracle = oracle::ProfileCBCOracle::key_as_iv();
    let ciphertext_parts = oracle
        .encode_profile(data::Bytes::read_utf8("some text"))
        .split(16);
    let ciphertext =
        ciphertext_parts[0].clone() + data::Bytes::zero(16) + ciphertext_parts[0].clone();
    if let Err(plain) = oracle.get_role(ciphertext) {
        let plain = plain.split(16);
        let key = &plain[0] ^ &plain[2];
        println!("Key: {:X}", key);
        assert_eq!(key, oracle.get_raw_key());
    }
}

#[test]
fn challenge_4_26() {
    let oracle = oracle::CTRProfileOracle::new();
    // email=<>&uid=10&role=user
    let ciphertext = oracle.encode_profile(data::Bytes::read_utf8("test\x00role\x00admin"));
    let mut mask = data::Bytes::zero(ciphertext.len());
    mask["email=test".len()] = '&' as u8;
    mask["email=test&role".len()] = '=' as u8;
    let ciphertext = ciphertext ^ mask;
    println!("Role: {:?}", oracle.get_role(ciphertext));
}

#[test]
fn challenge_4_25() {
    let data = file::File::read_64_file("data_1_7").read_bytes();
    let key = data::Bytes::read_utf8("YELLOW SUBMARINE");
    let plaintext = cipher::aes_ecb_de(data, key);

    let key = data::Bytes::rand(16);
    let nonce = rand::random();
    let cipher = cipher::stream::SeekableStream::new(cipher::CTRstream::new(nonce, key));
    let encrypted = cipher.encrypt(&plaintext, 0);

    // Get keystream by replacing the plaintext (via `edit`) with all zeros
    let mut key_stream = encrypted.clone();
    cipher.edit(&mut key_stream, 0, &data::Bytes::zero(encrypted.len()));
    let recovered = key_stream ^ encrypted;
    assert_eq!(plaintext, recovered, "Failed to recover plaintext");
}

#[test]
fn challenge_3_24() {
    // Prove that the cipher actually works
    let seed = rand::random();
    let mut cipher = cipher::stream::Stream::new(random::MersenneGen::new(seed));
    let test = data::Bytes::rand(100);
    let encrypted = cipher.encrypt(&test);
    let mut cipher = cipher::stream::Stream::new(random::MersenneGen::new(seed));
    let decrypted = cipher.encrypt(&encrypted);
    assert_eq!(test, decrypted, "Encryption and decryption failed");

    // Known plaintext attack
    let seed = rand::random();
    let mut cipher = cipher::stream::Stream::new(random::MersenneGen::new(seed));

    // Plaintest: "garbage" + "AAAAA", 0..20 bytes of garbage, 14 bytes of 'A'
    let plaintext =
        data::Bytes::rand(rand::random::<usize>() % 20) + data::Bytes::from_bytes(b"A") * 14;
    let encrypted = cipher.encrypt(&plaintext);
    let unknown_size = encrypted.len() - 14;
    // get the known portion of the plaintext
    let known_plain = data::Bytes::from_bytes(&encrypted[unknown_size..]);
    // xor with plaintext to get the keystream portion
    let key_stream = known_plain ^ (data::Bytes::from_bytes(b"A") * 14);
    // Every 4 bytes of the keystream is one 32bit output of the rng

    // Could brute force solution by just running through all possible seeds.
    // The challenge does specify that the seed should only be 16 bits, so
    // this is more feasible than with 32 bits
    //
    // I'm pretty sure this isn't possible otherwise, without a much larger known
    // plaintext
    //
    // The password token portion is likely also a brute force operation, assuming
    // the token was generated in the last ten minutes, there are only 10*60 seeds
    // The token could also be chosen at random from the prng output, to increase
    // the size, but not substantially
}

#[test]
fn challenge_3_23() {
    // randomly seed rng
    let mut rng = random::MersenneGen::new(rand::random());

    // algo from seed to first output
    //top down input            bottom up for inverse
    //y = y ^ ((y >> U) & D);  |
    //let mut y = y ^ ((y >> random::U) & random::D);
    //y = y ^ ((y << S) & B);  |
    //let mut y = y ^ ((y << random::S) & random::B);
    //y = y ^ ((y << T) & C);  |y = y ^ ((y << T) & C)
    //let mut y = y ^ ((y << random::T) & random::C);
    //y = y ^ (y >> L);        |y = y ^ (y >> L);
    //let mut y = y ^ (y >> random::L);
    //---------- Inverse
    let mut state = [0u32; random::N];
    for n in state.iter_mut() {
        let mut y = rng.extract_number();
        y = y ^ (y >> random::L);
        y = y ^ ((y << random::T) & random::C);

        for i in 0..32 {
            y = y ^ ((y << random::S) & random::B & (1 << i));
        }
        for i in 0..32 {
            y = y ^ ((y >> random::U) & random::D & (1 << (31 - i)));
        }
        //let test = rng.get_internal(0);
        //println!("{:032b}", test);
        //println!("{:032b}", y);
        //println!("{}", if test == y {"Correct"} else {"Failed"});
        *n = y;
    }
    for (i, (test, actual)) in state.iter().zip(rng.get_state().iter()).enumerate() {
        if test != actual {
            println!("Failed at {}", i);
        }
    }
    /*
     * MT19937 can be improved by hashing the outputs, which would prevent effective
     * reversing the output back to the internal stat
     */
}

#[test]
fn challenge_3_22() {
    /*
     * I cannot easily do this challenge. Rust doesn't have an easy way to get the current
     * system time, and this relies on reducing the search space by getting the current time
     *
     * Basically, I could grab the current system time, some time after seeding the rng, and just
     * work backwards by brute forcing the rng generator. In the future, I might make a quick and
     * dirty `get_system_time()`, which could implement fake time passage, and doesn't actually
     * need to use the system time at all.
     */
}

#[test]
fn challenge_3_21() {
    let mut rng = random::MersenneGen::new(0);
    for _ in 0..100 {
        println!("{}", rng.extract_number());
    }
}

#[test]
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

#[test]
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

#[test]
fn challenge_3_18() {
    let data = data::Bytes::read_64(
        "L77na/nrFsKvynd6HzOoG7GHTLXsTVu9qvY/2syLXzhPweyyMTJULu/6/kXX0KSvoOLSFQ==",
    );
    let mut stream = cipher::stream::Stream::new(cipher::CTRstream::new(
        0,
        data::Bytes::read_utf8("YELLOW SUBMARINE"),
    ));
    println!("Decrypted: {}", stream.encrypt(&data));
}

#[test]
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

#[test]
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

    assert_eq!(oracle.get_role(enc), Ok(oracle::Role::USER));
    assert_eq!(oracle.get_role(swapped), Ok(oracle::Role::ADMIN));
}

#[test]
fn challenge_2_15() {
    // I'm supposed to write a function to trim PKCS#7 padding,
    // but I already wrote it. It's `data::Bytes::trim_pkcs7()`
    // It doesn't panic if there isn't padding, it just assumes
    // there wasn't anything to remove
}

#[test]
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

#[test]
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

#[test]
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

#[test]
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

#[test]
fn challenge_2_10() {
    let data = file::File::read_64_file("data_2_10").read_bytes();
    let key = data::Bytes::read_utf8("YELLOW SUBMARINE");
    let iv =
        data::Bytes::read_utf8("\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    println!("{}", cipher::aes_cbc_de(data, key, iv));
}

#[test]
fn challenge_2_9() {
    let data = data::Bytes::read_utf8("YELLOW SUBMARINE");
    assert_eq!(
        data.pad_pkcs7(20).to_utf8(),
        "YELLOW SUBMARINE\x04\x04\x04\x04"
    );
}

#[test]
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

#[test]
fn challenge_1_7() {
    let data = file::File::read_64_file("data_1_7").read_bytes();
    let key = data::Bytes::read_utf8("YELLOW SUBMARINE");
    println!("{}", cipher::aes_ecb_de(data, key));
}

#[test]
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

#[test]
fn challenge_1_5() {
    let text = data::Bytes::read_utf8(
        "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal",
    );
    let key = data::Bytes::read_utf8("ICE");
    assert_eq!(text.clone() ^ key.clone(), "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");

    let xor = data::Bytes::read_hex("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
    assert_eq!((xor ^ key).to_utf8(), text.to_utf8());
}

#[test]
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

#[test]
fn challenge_1_3() {
    let raw1 = data::Bytes::read_hex(
        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
    );

    println!(
        "{}",
        decrypt::decrypt_xor(raw1, keys::KeyGen::new(1), &lang::score_string).0
    );
}

#[test]
fn challenge_1_2() {
    let raw1 = data::Bytes::read_hex("1c0111001f010100061a024b53535009181c");
    let raw2 = data::Bytes::read_hex("686974207468652062756c6c277320657965");

    assert_eq!(raw1 ^ raw2, "746865206b696420646f6e277420706c6179");
}

#[test]
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
