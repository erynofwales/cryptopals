// cryptopals.rs
// Eryn Wells <eryn@erynwells.me>

extern crate cryptopals;

use std::f32;
use std::fs::File;
use std::io::{BufRead, BufReader};
use cryptopals::b64::Base64Encodable;
use cryptopals::hex::{HexDecodable, HexEncodable};
use cryptopals::letter_frequency::LetterFreq;
use cryptopals::xor::{FixedXOR, ByteXOR};

/// Base 64 encode some bytes.
#[test]
fn s1c1() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let ex_output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    let output: String = input.chars().hex_decoded().base64_encoded().collect();
    assert_eq!(output, ex_output);
}

/// XOR two arrays of bytes together to get a new byte string.
#[test]
fn s1c2() {
    let a = "1c0111001f010100061a024b53535009181c";
    let b = "686974207468652062756c6c277320657965";
    let ex_output = "746865206b696420646f6e277420706c6179";
    let output: String = a.chars().hex_decoded()
        .fixed_xor(b.chars().hex_decoded())
        .hex_encoded().collect();
    assert_eq!(output, ex_output);
}

/// Determine the key for a single byte XOR encryption scheme using English letter frequency analysis.
#[test]
fn s1c3() {
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let mut best_key = 0u8;
    let mut best_score = f32::INFINITY;
    let mut best_output: Option<String> = None;
    for key in 32u8..127 {
        let possible_output = input.chars().hex_decoded()
            .byte_xor(key)
            .map(char::from)
            .collect::<String>();
        let score = possible_output.chi2_freqs("en");
        println!("{}: {:?} -> {}", key, possible_output, score);
        if !score.is_nan() && score < best_score {
            best_score = score;
            best_output = Some(possible_output);
            best_key = key;
        }
    }
    let best_output = best_output.expect("expected output");
    println!("{}: {:?} -> {}", best_key, best_output, best_score);
    assert!(best_output.to_lowercase().contains("bacon"));
}

/// Detect a single byte XOR encryption scheme using the letter frequency analysis from s1c3.
#[test]
fn s1c4() {
    let f = match File::open("resources/s1c4.txt") {
        Ok(f) => f,
        Err(e) => panic!("failed to open the strings file: {}", e),
    };
    let reader = BufReader::new(&f);

    let mut best_overall_key = 0u8;
    let mut best_overall_score = f32::INFINITY;
    let mut best_overall_output: Option<String> = None;

    for line in reader.lines() {
        // Decode the string.
        let line = line.unwrap();
        println!("{:?}", line);
        let decoded = line.chars().hex_decoded().collect::<Vec<u8>>();

        // Do the decryption across all printable ASCII characters
        let mut best_key = 0u8;
        let mut best_score = f32::INFINITY;
        let mut best_output: Option<String> = None;
        for key in 32u8..127 {
            let decrypted = decoded.iter().map(|c| *c).byte_xor(key).map(char::from).collect::<String>();
            let score = decrypted.chi2_freqs("en");
            if !score.is_nan() && score < best_score {
                best_score = score;
                best_output = Some(decrypted);
                best_key = key;
                println!("\tnew best {} -> {:7.3} {:?}", key, score, best_output);
            }
        }

        if best_output.is_some() && best_score < best_overall_score {
            best_overall_key = best_key;
            best_overall_score = best_score;
            best_overall_output = best_output;
            println!("-> new overall best: {}: {:?} -> {}", best_overall_key, best_overall_output, best_overall_score);
        }
    }

    println!("Overall Best: {} {:7.3} {:?}", best_overall_key, best_overall_score, best_overall_output);
}
