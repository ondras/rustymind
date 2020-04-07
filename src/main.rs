mod ai;
mod guess;
mod score;
mod util;

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

fn game(mut attacker: impl Attacker, defender: impl Defender) {
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
            println!("Round #{}\n           {} | {}", rounds, score, attempt);

            if score.is_won() {
                println!("The code was found in {} rounds.", rounds);
                break;
            }

            if rounds == 10 {
                println!("Not found in {} rounds", rounds);
                break;
            }

            previous = Some((attempt, score));
        } else {
            println!("Code was not found");
            break;
        }
    }
}

fn main() {
    let defender = ai::defender::Defender::new(4);
    let attacker = ai::attacker::Attacker::new(defender.code_length());
    game(attacker, defender);
}
