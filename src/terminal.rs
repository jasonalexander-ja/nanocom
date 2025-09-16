use crate::utils::{del_char, put_string};

const ALLOWED_CTRL_CHARS: [u8; 2] = [
    27,
    9
];

pub fn print_char(key: u8) {
    let keychar = key as char;
    match key {
        127 | 8 => return del_char(),
        13 => return println!(),
        _ if keychar.is_ascii_control() && !ALLOWED_CTRL_CHARS.contains(&key) => return,
        _ => ()
    }
    put_string(format!("{}", keychar));
}
