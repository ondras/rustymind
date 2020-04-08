use crate::CODE_CHARS;
use termion::color;

pub fn format_char(ch: char) -> String {
    match ch {
        'A' => format!("{}", color::Fg(color::LightRed)),
        'B' => format!("{}", color::Fg(color::LightGreen)),
        'C' => format!("{}", color::Fg(color::LightBlue)),
        'D' => format!("{}", color::Fg(color::LightYellow)),
        'E' => format!("{}", color::Fg(color::LightMagenta)),
        'F' => format!("{}", color::Fg(color::LightCyan)),
        'G' => format!("{}", color::Fg(color::LightWhite)),
        '?' => format!("{}", color::Fg(color::LightWhite)),
        _ => format!("{}", color::Fg(color::LightBlack)),
    }
}

pub fn min_char() -> char {
    'A'
}

pub fn max_char() -> char {
    (min_char() as u8 + CODE_CHARS - 1) as char
}

pub fn all_chars() -> Vec<char> {
    let mut result = vec![];
    for i in (min_char() as u8)..(max_char() as u8 + 1) {
        result.push(i as char);
    }
    result
}
