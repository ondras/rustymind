use crate::util;
use std::fmt;

pub struct Guess {
    pub data: Vec<char>,
}

impl Guess {
    pub fn new(data: Vec<char>) -> Self {
        Self {
            data: data.into_iter().map(|x| x.to_ascii_uppercase()).collect(),
        }
    }

    pub fn len(&self) -> u8 {
        self.data.len() as u8
    }
}

impl fmt::Display for Guess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for ch in self.data.iter() {
            let colour = util::letter_colour(*ch);
            let ch = util::pad(ch.to_string());
            write!(f, "{}", colour.bold().paint(ch))?;
        }
        Ok(())
    }
}
