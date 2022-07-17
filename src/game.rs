use std::{io, io::prelude::*};

pub struct Game {
    cleartext: String,
    encoded: String,
    working: String,
    set: String,
}

impl Game {
    pub fn new(cleartext: String, encoded: String, set: String) -> Self {
        let mut working = String::new();
        for c in encoded.chars() {
            if set.contains(c) {
                working.push('_');
            } else {
                working.push(c);
            }
        }
        Self {
            cleartext,
            encoded,
            set,
            working,
        }
    }

    pub fn play(&mut self) {
        eprintln!("Enter 2 character commands to edit (selection+motion syntax)");
        while self.working != self.cleartext.to_ascii_lowercase() {
            print!("{esc}c", esc = 27 as char);
            println!("encrypted: {}\nworking: {}\n", self.encoded, self.working);
            let mut cmd = String::new();
            let res = std::io::stdin().read_line(&mut cmd);
            if let Ok(_) = res {
                self.command(&cmd);
            } else {
                eprintln!("Please enter a 2 character command (selection, motion)");
            }
        }

        println!("You win!");
        println!("cleartext:\t\"{}\"", self.cleartext);
    }

    pub fn command(&mut self, command: &str) {
        let selection = command.chars().nth(0);
        let motion = command.chars().nth(1);
        if let Some(selection) = selection {
            if let Some(motion) = motion {
                println!("changing {} to {}", selection, motion);
                let mut ret = self.working.clone();
                for i in 0..self.working.len() {
                    if self.encoded.chars().nth(i).unwrap() == selection {
                        ret.replace_range(i..=i, &motion.to_string())
                    }
                }
                self.working = ret;
            }
        }
    }
}
