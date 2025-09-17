use std::io;

use crate::parser::EscapeSequence;
use crate::state::State;


pub fn handle_escape(parsed: EscapeSequence, state: &mut State) -> Result<(), io::Error> {
    let _ = match parsed {
        EscapeSequence::ArrowDown => state.term.move_cursor_down(1)?,
        EscapeSequence::ArrowRight => state.term.move_cursor_right(1)?,
        EscapeSequence::ArrowLeft => state.term.move_cursor_left(1)?,
        EscapeSequence::ArrowUp => state.term.move_cursor_up(1)?,
        EscapeSequence::Home => state.term.clear_screen()?,
        _ => (),
    };

    Ok(())
}

