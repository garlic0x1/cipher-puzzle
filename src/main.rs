use crate::game::*;
use rand::seq::SliceRandom;
static SET: &str = "abcdefghijklmnopqrstuvwxyz";

use rand::seq::IteratorRandom; // 0.7.3
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub mod game;
const FILENAME: &str = "./kjv.txt";

fn find_word() -> String {
    let f = File::open(FILENAME)
        .unwrap_or_else(|e| panic!("(;_;) file not found: {}: {}", FILENAME, e));
    let f = BufReader::new(f);

    let lines = f.lines().map(|l| l.expect("Couldn't read line"));

    lines
        .choose(&mut rand::thread_rng())
        .expect("File had no lines")
}

fn main() {
    let message = find_word();
    println!("cleartext: {}", message);
    let cipher = shuffle(SET);
    let encrypted = encrypt(&message, SET, &cipher);

    let mut game = Game::new(message, encrypted, SET.to_string());
    game.play();
}

fn shuffle(string: &str) -> String {
    let mut rng = rand::thread_rng();
    let mut bytes = string.to_string().into_bytes();
    bytes.shuffle(&mut rng);

    let result = String::from_utf8(bytes).unwrap();
    for i in 0..string.len() {
        if string.chars().nth(i) == result.chars().nth(i) {
            return shuffle(&result);
        }
    }

    result
}

fn encrypt(cleartext: &str, set: &str, cipher: &str) -> String {
    let mut s = String::new();

    for ch in cleartext.chars() {
        // standardize case
        let mut c = ch;
        let ascii = c as u8;
        if ascii > 64 && ascii < 91 {
            c = (ascii + 32) as char;
        } else {
            c = ch;
        }

        if let Some(index) = set.chars().position(|r| r == c) {
            s.push(cipher.chars().nth(index).unwrap());
        } else {
            s.push(c);
        }
    }

    s
}
