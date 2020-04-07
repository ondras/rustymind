use crate::guess::Guess;
use crate::score::Score;
use crate::util;
use ansi_term::Style;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Defender {
    code: Guess,
}

impl crate::Defender for Defender {
    fn new(code_length: u8) -> Self {
        let all = util::all_chars();
        let mut data: Vec<char> = vec![];
        for _ in 1..code_length + 1 {
            let mut rng = thread_rng();
            data.push(*all.choose(&mut rng).unwrap());
        }

        let code = Guess::new(data);
        println!(
            "The {} is | {}",
            Style::new().bold().paint("secret code"),
            code
        );
        print!("======================");
        for _ in 0..(3 * code.data.len()) {
            print!("=");
        }
        println!();

        Self { code }
    }

    fn code_length(&self) -> u8 {
        self.code.len() as u8
    }

    fn score(&self, attempt: &Guess) -> Score {
        Score::compute(&attempt, &self.code)
    }
}
