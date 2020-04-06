use crate::CODE_CHARS;
use crate::util;
use crate::guess::Guess;


type Data = Vec<char>;

pub struct Generator {
	state: Data,
}

impl Generator {
    pub fn new(code_length: u8) -> Self {
        let mut state = vec![];
        for i in 0..code_length {
            let ch = (util::min_char() as u8) + (i % CODE_CHARS);
            state.push(ch as char);
        }

        Self::from(state)
    }

    pub fn get(&self) -> Guess {
        Guess::new(self.state.clone())
    }

	pub fn advance(&mut self) -> Result<(), ()> {
        let mut carry = false;
        let min = util::min_char() as u8;

        for (i, ch) in self.state.iter_mut().enumerate() {
            let overflow = (i as u8) % CODE_CHARS; // this index overflows at this value
            let mut value = (*ch as u8) - min;
            value = (value + 1) % CODE_CHARS;
            *ch = (value + min) as char;
            carry = { value == overflow };
            if !carry { break; }
        }

        if carry { Err(()) } else { Ok(()) }
    }
}

impl From<Data> for Generator {
    fn from(state: Data) -> Self {
		Self { state }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        assert_eq!(Generator::new(5).get().data, vec!['A', 'B', 'C', 'D', 'E']);

        let last = Generator::new(CODE_CHARS+1).get().data.last().copied();
        assert_eq!(last.unwrap(), 'A');
    }

    #[test]
    fn test_from() {
        let vec = vec!['B', 'C', 'D'];
        assert_eq!(Generator::from(vec.clone()).get().data, vec);
    }

    #[test]
    fn test_advance() {
        let mut gen = Generator::from(vec!['A', 'B', 'C']);

        gen.advance().expect("Can advance");
        assert_eq!(gen.get().data, vec!['B', 'B', 'C']);

        gen.advance().expect("Can advance");
        assert_eq!(gen.get().data, vec!['C', 'B', 'C']);
    }

    #[test]
    fn test_no_advance() {
        let max = util::max_char();

        let mut gen = Generator::from(vec![max, 'A', 'B']);
        let result = gen.advance();
        assert!(result.is_err());
    }

    #[test]
    fn test_advance_carry() {
        let max = util::max_char();

        let mut gen = Generator::from(vec![max, 'B', 'C']);
        gen.advance().expect("Can advance");
        assert_eq!(gen.get().data, vec!['A', 'C', 'C']);

        let mut gen = Generator::from(vec![max, 'A', 'C']);
        gen.advance().expect("Can advance");
        assert_eq!(gen.get().data, vec!['A', 'B', 'D']);
    }

    #[test]
    fn test_advance_count() {
        let code_length = 3;
        let mut gen = Generator::new(code_length);
        let mut count = 0;

        loop {
            count += 1;
            let result = gen.advance();
            if result.is_err() { break; }
        }

        assert_eq!(count, (CODE_CHARS as u32).pow(code_length as u32));
    }
}
