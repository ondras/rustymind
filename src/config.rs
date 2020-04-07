use std::env;

pub struct Config {
    pub code_length: u8,
    pub ai: bool,
}

impl Config {
    pub fn parse() -> Result<Self, String> {
        let mut code_length = 4;
        let mut ai = true;
        let mut args: Vec<String> = env::args().skip(1).collect();

        let iguess_idx = args.iter().position(|x| x == "--i-guess");
        if let Some(idx) = iguess_idx {
            args.remove(idx);
            ai = false;
        }

        if let Some(last) = args.pop() {
            if let Ok(parsed) = last.parse::<u8>() {
                let range = 2..6 + 1;
                if range.contains(&parsed) {
                    code_length = parsed;
                } else {
                    return Err(format!(
                        "Pick a value from {} to {} for code length",
                        range.start,
                        range.end - 1
                    ));
                }
            } else {
                return Err(format!("Cannot parse '{}' as a code length", last));
            }
        }

        Ok(Self { code_length, ai })
    }
}
