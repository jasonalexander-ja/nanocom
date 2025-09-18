use std::io;

use crate::key::EscapeSequence;
use crate::state::State;
use crate::utils;


/// Handles escape codes received and performs a standard action on the terminal. 
pub fn handle_escape(parsed: EscapeSequence, state: &mut State) -> Result<(), io::Error> {
    let _ = match parsed {
        EscapeSequence::ArrowDown => state.term.move_cursor_down(1)?,
        EscapeSequence::ArrowRight => state.term.move_cursor_right(1)?,
        EscapeSequence::ArrowLeft => state.term.move_cursor_left(1)?,
        EscapeSequence::ArrowUp => state.term.move_cursor_up(1)?,
        EscapeSequence::Home => state.term.clear_screen()?,
        EscapeSequence::BackTab => state.term.move_cursor_left(utils::TABS)?,
        _ => (),
    };

    Ok(())
}

