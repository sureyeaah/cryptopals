use std::iter::{zip, FromIterator};

fn main() {
    run_hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d", "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    run_fixed_xor(
        "1c0111001f010100061a024b53535009181c",
        "686974207468652062756c6c277320657965",
        "746865206b696420646f6e277420706c6179",
    );
    println!("done")
}

fn run_fixed_xor(input1: &str, input2: &str, output: &str) {
    assert_eq!(
        output,
        to_hex_string(fixed_xor(from_hex_string(input1), from_hex_string(input2)))
    )
}

fn fixed_xor(input1: Vec<u8>, input2: Vec<u8>) -> Vec<u8> {
    Vec::from_iter(zip(input1.into_iter(), input2.into_iter()).map(|(x, y)| x ^ y))
}

fn run_hex_to_base64(input: &str, output: &str) {
    let base64 = hex_to_base64(from_hex_string(input));
    assert_eq!(output, print_base64(&base64));
}

fn hex_to_base64(hex: Vec<u8>) -> Vec<u8> {
    let mut base64 = Vec::new();
    let mut counter = 0;
    let mut bits = 0 as u8;
    for x in hex {
        if counter == 0 {
            bits = x;
            counter = 1;
        } else if counter == 1 {
            base64.push(bits << 2 | x >> 2);
            bits = 0b11 & x;
            counter = 2;
        } else {
            base64.push(bits << 4 | x);
            bits = 0;
            counter = 0;
            continue;
        }
    }
    if counter == 1 {
        base64.push(bits << 2);
    } else if counter == 2 {
        base64.push(bits << 4);
    }
    return base64;
}

fn print_base64(raw: &Vec<u8>) -> String {
    String::from_iter(raw.iter().map(|x| match x {
        0..=25 => (x + 65) as char,
        26..=51 => (x + 71) as char,
        52..=61 => (x - 4) as char,
        62 => 43 as char,
        63 => 47 as char,
        _ => '=',
    }))
}

fn from_hex_string(hex: &str) -> Vec<u8> {
    Vec::from_iter(hex.as_bytes().iter().map(|x| match x {
        48..=57 => x - 48,
        65..=90 => x - 65 + 10,
        97..=122 => x - 97 + 10,
        _ => panic!(),
    }))
}

fn to_hex_string(raw: Vec<u8>) -> String {
    String::from_iter(raw.iter().map(|x| match x {
        0..=9 => (x + 48) as char,
        10..=15 => (x - 10 + 97) as char,
        _ => panic!(),
    }))
}
