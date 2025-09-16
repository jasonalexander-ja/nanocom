use std::io::Write;

use crate::utils::del_char;

const ALLOWED_CTRL_CHARS: [u8; 3] = [
    13,
    27,
    9
];

pub fn print_char(key: u8) {
    let keychar = key as char;
    if key == 127 || key == 8 {
        del_char();
        return;
    }
    if keychar.is_ascii_control() && !ALLOWED_CTRL_CHARS.contains(&key) {
        return;
    }
    print!("{}", keychar);
    let _ = std::io::stdout().flush();
}
