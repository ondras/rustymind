use crate::util;
use std::fmt;
use termion::{color, style};

pub struct Guess {
    pub data: Vec<char>,
}

impl Guess {
    pub fn len(&self) -> u8 {
        self.data.len() as u8
    }
}

impl From<Vec<char>> for Guess {
    fn from(data: Vec<char>) -> Self {
        Self {
            data: data.into_iter().map(|x| x.to_ascii_uppercase()).collect(),
        }
    }
}

impl fmt::Display for Guess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", style::Bold)?;
        for ch in self.data.iter() {
            let char_color = util::format_char(*ch);
            write!(
                f,
                "{} {} {}",
                char_color,
                ch.to_string(),
                color::Fg(color::Reset)
            )?;
        }
        write!(f, "{}", style::Reset)
    }
}
