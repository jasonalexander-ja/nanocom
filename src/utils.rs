use std::io::Write;


/// Number of spaces in tabs 
pub const TABS: usize = 8;

/// A list of standard bauds. 
pub const BAUDS: [u32; 30] = [
    4000000,
    3500000,
    3000000,
    2500000,
    2000000,
    1500000,
    1152000,
    1000000,
    921600,
    576000,
    500000,
    460800,
    230400,
    115200,
    57600,
    38400,
    19200,
    9600,
    4800,
    2400,
    1800,
    1200,
    600,
    300,
    200,
    150,
    134,
    110,
    75,
    50
];

/// Puts a character on the terminal and flushes the buffer. 
pub fn put_char(c: char) {
    print!("{}", c);
    let _ = std::io::stdout().flush();
}

/// Puts an str on the terminal and flushes the buffer. 
pub fn put_str(c: &str) {
    print!("{}", c);
    let _ = std::io::stdout().flush();
}

/// Puts a string on the terminal and flushes the buffer. 
pub fn put_string(c: String) {
    print!("{}", c);
    let _ = std::io::stdout().flush();
}

/// Erases the last character printed to the terminal, flushes the buffer. 
pub fn del_char() {
    print!("\x08 \x08");
    let _ = std::io::stdout().flush();
}

/// Gets an ascii byte from a char. Char *must* be ascii. 
pub fn get_ascii_byte(c: char) -> u8 {
    let mut res = [0x00u8];
    c.encode_utf8(&mut res);
    res[0]
}
