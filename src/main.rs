use crate::game::*;
use rand::seq::SliceRandom;
static SET: &str = "abcdefghijklmnopqrstuvwxyz";

pub mod game;

fn main() {
    let mut message = String::from("this is a message");
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
