use super::generator::Generator;
use crate::guess::Guess;
use crate::score::Score;
use crate::History;

pub struct Attacker {
    generator: Generator,
}

impl crate::Attacker for Attacker {
    fn new(code_length: u8) -> Self {
        Self { generator: Generator::new(code_length) }
    }

    fn guess(&mut self, history: &[History]) -> Option<Guess> {
        let mut guess: Option<Guess> = None;

        loop {
            let g = self.generator.get();

            if satisfies_history(&g, history) {
                guess = Some(g);
                break;
            } else {
                let result = self.generator.advance();
                if result.is_err() {
                    break;
                }
            }
        }

        guess
    }
}

fn satisfies_history(test: &Guess, history: &[History]) -> bool {
    for (attempt, score) in history {
        let alt_score = Score::compute(attempt, test);
        if alt_score != *score {
            return false;
        }
    }
    true
}
