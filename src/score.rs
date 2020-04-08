use crate::guess::Guess;
use std::fmt;
use termion::{color, style};


#[derive(Debug, PartialEq)]
pub struct Score {
    black: u8,
    white: u8,
    is_won: bool,
}

impl Score {
    pub fn compute(attempt: &Guess, code: &Guess) -> Self {
        let mut black: u8 = 0;
        let mut white: u8 = 0;
        let mut attempt_mask = attempt.data.clone();
        let mut code_mask = code.data.clone();

        for (i, attempt_ch) in attempt.data.iter().enumerate() {
            if let Some(code_ch) = code_mask.get(i) {
                if code_ch == attempt_ch {
                    black += 1;
                    code_mask[i] = ' ';
                    attempt_mask[i] = ' ';
                }
            }
        }

        for attempt_ch in attempt_mask.iter().filter(|ch| **ch != ' ') {
            if let Some(code_i) = code_mask.iter().position(|code_ch| code_ch == attempt_ch) {
                white += 1;
                code_mask[code_i] = ' ';
            }
        }

        let is_won = { black == code.len() };

        Self {
            black,
            white,
            is_won,
        }
    }

    pub fn is_won(&self) -> bool {
        self.is_won
    }
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let reset_color = format!("{}{}", color::Fg(color::Reset), color::Bg(color::Reset));
        let black = format!(
            "{}{} {} {}",
            color::Fg(color::Black),
            color::Bg(color::White),
            self.black.to_string(),
            reset_color
        );

        let white = format!(
            "{}{} {} {}",
            color::Fg(color::LightWhite),
            color::Bg(color::Reset),
            self.white.to_string(),
            reset_color
        );

        write!(f, "{}", style::Bold)?;
        write!(f, "{} {}", black, white)?;
        write!(f, "{}", style::Reset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_score_no_repeat() {
        let code = Guess::new(vec!['a', 'b', 'c', 'd']);

        let score = Score::compute(&Guess::new(vec![]), &code);
        assert_eq!(
            score,
            Score {
                black: 0,
                white: 0,
                is_won: false
            }
        );

        let score = Score::compute(&Guess::new(vec!['a']), &code);
        assert_eq!(
            score,
            Score {
                black: 1,
                white: 0,
                is_won: false
            }
        );

        let score = Score::compute(&Guess::new(vec!['b']), &code);
        assert_eq!(
            score,
            Score {
                black: 0,
                white: 1,
                is_won: false
            }
        );

        let score = Score::compute(&Guess::new(vec!['a', 'x', 'b']), &code);
        assert_eq!(
            score,
            Score {
                black: 1,
                white: 1,
                is_won: false
            }
        );

        let score = Score::compute(&Guess::new(vec!['b', 'a', 'd', 'c']), &code);
        assert_eq!(
            score,
            Score {
                black: 0,
                white: 4,
                is_won: false
            }
        );

        let score = Score::compute(&Guess::new(vec!['a', 'b', 'c']), &code);
        assert_eq!(
            score,
            Score {
                black: 3,
                white: 0,
                is_won: false
            }
        );

        let score = Score::compute(&Guess::new(vec!['a', 'a', 'a']), &code);
        assert_eq!(
            score,
            Score {
                black: 1,
                white: 0,
                is_won: false
            }
        );

        let score = Score::compute(&Guess::new(vec!['x', 'a', 'a']), &code);
        assert_eq!(
            score,
            Score {
                black: 0,
                white: 1,
                is_won: false
            }
        );
    }

    #[test]
    fn test_score_repeat() {
        let code = Guess::new(vec!['a', 'b', 'a', 'c']);

        let score = Score::compute(&Guess::new(vec![]), &code);
        assert_eq!(
            score,
            Score {
                black: 0,
                white: 0,
                is_won: false
            }
        );

        let score = Score::compute(&Guess::new(vec!['a']), &code);
        assert_eq!(
            score,
            Score {
                black: 1,
                white: 0,
                is_won: false
            }
        );

        let score = Score::compute(&Guess::new(vec!['b']), &code);
        assert_eq!(
            score,
            Score {
                black: 0,
                white: 1,
                is_won: false
            }
        );

        let score = Score::compute(&Guess::new(vec!['a', 'x', 'b']), &code);
        assert_eq!(
            score,
            Score {
                black: 1,
                white: 1,
                is_won: false
            }
        );

        let score = Score::compute(&Guess::new(vec!['b', 'a', 'c', 'a']), &code);
        assert_eq!(
            score,
            Score {
                black: 0,
                white: 4,
                is_won: false
            }
        );

        let score = Score::compute(&Guess::new(vec!['a', 'b', 'a']), &code);
        assert_eq!(
            score,
            Score {
                black: 3,
                white: 0,
                is_won: false
            }
        );

        let score = Score::compute(&Guess::new(vec!['a', 'a', 'a']), &code);
        assert_eq!(
            score,
            Score {
                black: 2,
                white: 0,
                is_won: false
            }
        );

        let score = Score::compute(&Guess::new(vec!['x', 'a', 'x', 'a']), &code);
        assert_eq!(
            score,
            Score {
                black: 0,
                white: 2,
                is_won: false
            }
        );

        let score = Score::compute(&Guess::new(vec!['c', 'a', 'b', 'a']), &code);
        assert_eq!(
            score,
            Score {
                black: 0,
                white: 4,
                is_won: false
            }
        );

        let score = Score::compute(&Guess::new(vec!['c', 'a', 'a']), &code);
        assert_eq!(
            score,
            Score {
                black: 1,
                white: 2,
                is_won: false
            }
        );
    }
}
