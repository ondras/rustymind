use ansi_term::Colour;
use crate::CODE_CHARS;

pub fn letter_colour(ch: char) -> Colour {
	match ch {
		'A' => Colour::Red,
		'B' => Colour::Green,
		'C' => Colour::Blue,
		'D' => Colour::Yellow,
		'E' => Colour::Purple,
		'F' => Colour::Cyan,
		'G' => Colour::White,
		_ => Colour::Black
	}
}

pub fn pad(s: String) -> String {
	format!(" {} ", s)
}

pub fn min_char() -> char {
	'A'
}

pub fn max_char() -> char {
	(min_char() as u8 + CODE_CHARS - 1) as char
}

pub fn all_chars() -> Vec<char> {
	let mut result = vec![];
	for i in (min_char() as u8)..(max_char() as u8) {
		result.push(i as char);
	}
	result
}
