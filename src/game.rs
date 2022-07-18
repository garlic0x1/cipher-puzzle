use colored::Colorize;
use console::Term;

pub struct Game {
    cleartext: String,
    encoded: String,
    working: String,
    set: String,
    line_size: usize,
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
            line_size: 64,
        }
    }

    pub fn pretty_print(&self, highlight: Option<char>) {
        print!("{esc}c", esc = 27 as char);
    }

    pub fn print(&self, highlight: Option<char>) {
        print!("{esc}c", esc = 27 as char);
        let mut rot = 0;

        'outer: loop {
            for i in 0..self.line_size {
                let mut nth: char = '$';
                if let Some(c) = self.encoded.chars().nth((self.line_size * rot) + i) {
                    nth = c;
                } else {
                    break;
                }
                match highlight {
                    Some(h) => {
                        if nth == h {
                            print!("{}", nth.to_string().yellow().underline().italic())
                        } else {
                            print!("{}", nth);
                        }
                    }
                    None => print!("{}", nth),
                }
            }
            println!();

            for i in 0..self.line_size {
                let mut nth: char = '$';
                if let Some(c) = self.working.chars().nth((self.line_size * rot) + i) {
                    nth = c;
                } else {
                    println!();
                    break 'outer;
                }
                print!("{}", nth);
            }
            println!();
            rot += 1;
        }

        eprintln!(":h for help");
    }

    pub fn play(&mut self) {
        let stdout = Term::buffered_stdout();
        while self.working != self.cleartext.to_ascii_lowercase() {
            let mut cmd = String::new();

            self.print(None);

            if let Ok(sel) = stdout.read_char() {
                if self.set.contains(sel) {
                    self.print(Some(sel));
                }
                cmd.push(sel);
            }

            if let Ok(motion) = stdout.read_char() {
                cmd.push(motion);
            }
            if !self.command(&cmd) {
                break;
            }
        }

        print!("{esc}c", esc = 27 as char);
        println!("You win!");
        println!("\"{}\"", self.cleartext);
        println!("press any key to continue");
        stdout.read_char();
    }

    pub fn hint(&mut self) {}

    pub fn command(&mut self, command: &str) -> bool {
        match command {
            ":h" => {
                //self.print_help();
                println!(
                    "select a char with first key, and change it with second (selection, motion)"
                );
                println!("use '?' as a motion for a hint");
                println!("special commands\nclear - :c\nquit - :q");
                println!("press any key to continue");
                let stdout = Term::buffered_stdout();
                stdout.read_char();
                return true;
            }
            ":c" => {
                let mut working = String::new();
                for c in self.encoded.chars() {
                    if self.set.contains(c) {
                        working.push('_');
                    } else {
                        working.push(c);
                    }
                }
                self.working = working;
            }
            ":q" => return false,
            _ => (),
        }
        let selection = command.chars().nth(0);
        let motion = command.chars().nth(1);
        if let Some(selection) = selection {
            if let Some(motion) = motion {
                if self.set.contains(selection) && self.set.contains(motion) {
                    let mut ret = self.working.clone();
                    for i in 0..self.working.len() {
                        if self.encoded.chars().nth(i).unwrap() == selection {
                            ret.replace_range(i..=i, &motion.to_string())
                        }
                    }
                    self.working = ret;
                } else if motion == '?' {
                    if self.set.contains(selection) {
                        let mut ret = self.working.clone();
                        for i in 0..self.working.len() {
                            if self.encoded.chars().nth(i).unwrap() == selection {
                                let clear = self.cleartext.chars().nth(i).unwrap();
                                ret.replace_range(i..=i, &clear.to_string().to_ascii_lowercase());
                            }
                        }
                        self.working = ret;
                    }
                } else if motion == ' ' {
                    if self.set.contains(selection) {
                        let mut ret = self.working.clone();
                        for i in 0..self.working.len() {
                            if self.encoded.chars().nth(i).unwrap() == selection {
                                ret.replace_range(i..=i, "_");
                            }
                        }
                        self.working = ret;
                    }
                }
            }
        }
        true
    }
}
