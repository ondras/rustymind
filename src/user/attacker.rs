use crate::guess::Guess;
use crate::util;
use crate::History;
use std::io::{Write, stdout, stdin};
use termion::cursor;
use termion::clear;

pub struct Attacker {
    code_length: u8,
}

impl Attacker {
    pub fn new(code_length: u8) -> Self {
        Self { code_length }
    }

    fn prompt(&self) {
        let g = Guess::from(util::all_chars());
        println!(" -> Enter your guess. Available chars: {}", g);
        print!  (" -> ");
        stdout().flush().unwrap();
    }

    fn parse(&self, str: &str) -> Option<Guess> {
        let chars: Vec<char> = str.chars().filter(|x| x.is_ascii_alphabetic()).collect();
        let guess = Guess::from(chars);

        let len = guess.len();
        if len == 0 || len > self.code_length {
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
    fn guess(&mut self, _previous: &[History]) -> Option<Guess> {
        loop {
            self.prompt();
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Cannot read input");
            print!("{}{}{}", cursor::Left(100), cursor::Up(2), clear::AfterCursor);

            if let Some(guess) = self.parse(input.trim()) {
                return Some(guess);
            }
        }
    }
}
