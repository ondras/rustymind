mod ai;
mod config;
mod guess;
mod score;
mod user;
mod util;

use config::Config;
use guess::Guess;
use score::Score;
use termion::clear;
use termion::cursor;

pub const CODE_CHARS: u8 = 8;

type History = (Guess, Score);

trait Attacker {
    fn guess(&mut self, previous: &[History]) -> Option<Guess>;
}

trait Defender {
    fn new(code_length: u8) -> Self;
    fn code_length(&self) -> u8;
    fn score(&self, attempt: &Guess) -> Score;
    fn print_header(&self, show_code: bool);
}

fn print_line(code_length: u8) {
    let count = 22 + code_length * 3;
    for _ in 0..count {
        print!("=");
    }
    println!();
}

fn game(mut attacker: Box<dyn Attacker>, defender: impl Defender) -> String {
    let mut history: Vec<History> = vec![];

    loop {
        println!("Round #{:<12}|", history.len() + 1);

        let attempt = attacker.guess(&history);
        if let Some(attempt) = attempt {
            assert!(
                attempt.len() <= defender.code_length(),
                "Attempt is longer than code!"
            );

            let score = defender.score(&attempt);

            println!("           {} | {}", score, attempt);

            if score.is_won() {
                return format!("The code was found in {} rounds.", history.len() + 1);
            }

            history.push((attempt, score));
        } else {
            return "Code was not found".to_string();
        }
    }
}

fn main() {
    let config = Config::parse().unwrap_or_else(|e| {
        println!("{}", e);
        std::process::exit(1);
    });

    print!("{}{}", clear::All, cursor::Goto(1, 1));

    let defender = ai::defender::Defender::new(config.code_length);
    let attacker: Box<dyn Attacker>;

    if config.ai {
        let a = ai::attacker::Attacker::new(defender.code_length());
        attacker = Box::new(a);
        defender.print_header(true);
    } else {
        let a = user::attacker::Attacker::new(defender.code_length());
        attacker = Box::new(a);
        defender.print_header(false);
    }

    print_line(config.code_length);
    let result = game(attacker, defender);
    print_line(config.code_length);
    println!("{}", result);
}
