use std::io::Write;

const ALLOWED_CTRL_CHARS: [u8; 3] = [
    13,
    27,
    9
];


pub fn print_char(key: u8) {
    let keychar = key as char;
    if key == 127 || key == 8 {
        print!("\x08 \x08");
        let _ = std::io::stdout().flush();
        return;
    }
    if keychar.is_ascii_control() && !ALLOWED_CTRL_CHARS.contains(&key) {
        return;
    }
    print!("{}", keychar);
    let _ = std::io::stdout().flush();
}


