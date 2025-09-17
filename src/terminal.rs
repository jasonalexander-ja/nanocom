use std::io::{self, Write};

use crate::state::State;
use crate::utils::put_char;
use crate::utils::put_string;
use crate::parser::Parsed;
use crate::escape;
use crate::utils::TABS;


pub fn print_data_in(data: Parsed, state: &mut State) -> Result<(), io::Error> {
    match data {
        Parsed::Nothing => return Ok(()),
        Parsed::Char(c) => print_char(c, state),
        Parsed::Escape(e) => escape::handle_escape(e, state)
    }
}

pub fn print_char(key: u8, state: &mut State) -> Result<(), io::Error> {
    if key < 32 || key == 127 { return handle_control_char(key, state) }
    let keychar = key as char;
    put_string(format!("{}", keychar));
    Ok(())
}

fn handle_control_char(key: u8, state: &mut State) -> Result<(), io::Error> {
    let keychar = key as char;
    match keychar {
        '\x09' => state.term.move_cursor_right(TABS)?,
        '\x7f' | '\x08' => state.term.clear_chars(1)?,
        '\x0d' => state.term.write_line("")?,
        '\x0A' => state.term.write(b"\x0A").map(|_| ())?,
        _ => put_char(key as char),
    }
    state.term.flush()
}
