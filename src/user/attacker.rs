use crate::guess::Guess;
use crate::util;
use crate::History;
use std::io;

pub struct Attacker {
    code_length: u8,
}

impl Attacker {
    fn prompt(&self) {
        let g = Guess::from(util::all_chars());
        println!("Enter your guess, space-separated. Available chars: {}", g);
    }

    fn parse(&self, str: &str) -> Option<Guess> {
        let chars: Vec<char> = str.chars().filter(|x| x.is_ascii_alphabetic()).collect();
        let guess = Guess::from(chars);

        if guess.len() > self.code_length {
            return None;
        }

        let min = &util::min_char();
        let max = &util::max_char();
        if guess.data.iter().any(|x| x < min || x > max) {
            return None;
        }

        Some(guess)
    }
}

impl crate::Attacker for Attacker {
    fn new(code_length: u8) -> Self {
        Self { code_length }
    }

    fn guess(&mut self, _previous: &[History]) -> Option<Guess> {
        let mut input = String::new();
        loop {
            self.prompt();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            if let Some(guess) = self.parse(input.trim()) {
                return Some(guess);
            }
        }
    }
}
