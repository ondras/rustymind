mod ai;
mod config;
mod guess;
mod score;
mod util;

use config::Config;
use guess::Guess;
use score::Score;

pub const CODE_CHARS: u8 = 8;

type History = (Guess, Score);

trait Attacker {
    fn new(code_length: u8) -> Self;
    fn guess(&mut self, previous: Option<History>) -> Option<Guess>;
}

trait Defender {
    fn new(code_length: u8) -> Self;
    fn code_length(&self) -> u8;
    fn score(&self, attempt: &Guess) -> Score;
}

fn print_line(code_length: u8) {
    let count = 22 + code_length * 3;
    for _ in 0..count {
        print!("=");
    }
    println!();
}

fn game(mut attacker: impl Attacker, defender: impl Defender) -> String {
    let mut rounds = 0;
    let mut previous: Option<History> = None;

    loop {
        rounds += 1;

        let attempt = attacker.guess(previous);
        if let Some(attempt) = attempt {
            assert!(
                attempt.len() <= defender.code_length(),
                "Attempt is longer than code!"
            );

            let score = defender.score(&attempt);
            println!(
                "Round #{}           |\n           {} | {}",
                rounds, score, attempt
            );

            if score.is_won() {
                return format!("The code was found in {} rounds.", rounds);
            }

            previous = Some((attempt, score));
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

    let defender = ai::defender::Defender::new(config.code_length);
    let attacker = ai::attacker::Attacker::new(defender.code_length());

    print_line(config.code_length);
    let result = game(attacker, defender);
    print_line(config.code_length);
    println!("{}", result);
}
