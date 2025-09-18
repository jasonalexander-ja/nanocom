use std::io::{self, Write};

use crate::state::State;
use crate::utils::put_char;
use crate::utils::put_string;
use crate::key::KeyIn;
use crate::escape_handlers;
use crate::utils::TABS;


/// Dispatches the correct routine for printing data received from the serial port. 
pub fn print_data_in(data: KeyIn, state: &mut State) -> Result<(), io::Error> {
    match data {
        KeyIn::Nothing => return Ok(()),
        KeyIn::Char(c) => print_char(c, state),
        KeyIn::Escape(e) => escape_handlers::handle_escape(e, state)
    }
}

/// Prints a character or actions a control code. 
pub fn print_char(key: u8, state: &mut State) -> Result<(), io::Error> {
    if key < 32 || key == 127 { return handle_control_char(key, state) }
    let keychar = key as char;
    put_string(format!("{}", keychar));
    Ok(())
}

/// Actions a control code on to the terminal. 
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
