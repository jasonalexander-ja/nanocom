use std::io::ErrorKind;

use console::Key;

use crate::{state::State, utils};


/// Polls the serial port for any data, parsing any escape sequences. 
pub fn poll_port_parse_data(state: &mut State) -> Result<SerialData, ()> {
    let v = match try_get_char(state)? {
        Some(c) => c,
        None => return Ok(SerialData::Nothing)
    };
    let res = if v == 0x1B {
        let res = handle_escape(state)?;
        SerialData::Escape(res)
    } else {
        SerialData::Char(v)
    };

    Ok(res)
}

/// Polls the serial port for a byte, displays an error message and throws if error with reading. 
fn try_get_char(state: &mut State) -> Result<Option<u8>, ()> {
    let mut buf = [0u8];
    match state.port.read(&mut buf) {
        Ok(0) => Ok(None),
        Ok(1..) => Ok(Some(buf[0])),
        Err(e) if e.kind() == ErrorKind::UnexpectedEof => Ok(None),
        Err(e) if e.kind() == ErrorKind::TimedOut => Ok(None),
        Err(e) => {
            println!("\r\n*** Failed to read from port, exiting \r\n{} ", e);
            Err(())
        }
    }
}

/// Blocking polls the serial port for a byte. 
fn get_char(state: &mut State) -> Result<u8, ()> {
    loop {
        match try_get_char(state)? {
            Some(v) => return Ok(v),
            None => continue
        }
    }
}

/// Dispatches the correct escape sequence parser for the type of escape sequence. 
pub fn handle_escape(state: &mut State) -> Result<EscapeSequence, ()> {
    let seq = vec![0x1B];
    let v = get_char(state)?;
    match v {
        0x20..=0x2F => handle_nf(v, seq, state),
        0x5B => handle_csi(v, seq, state),
        0x5D => handle_os(v, seq, state),
        0x30..=0x7E if v != 0x5B || v != 0x5D => handle_single(v, seq),
        _ => Ok(EscapeSequence::Invalid)
    }
}

/// Parses a single byte escape sequence. 
pub fn handle_single(byte: u8, seq: Vec<u8>) -> Result<EscapeSequence, ()> {
    let mut seq = seq;
    seq.push(byte);
    return Ok(EscapeSequence::UnknownSeq(seq))
}

/// Parses an [OS command](https://en.wikipedia.org/wiki/ANSI_escape_code#Operating_System_Command_sequences) escape sequence. 
pub fn handle_os(byte: u8, seq: Vec<u8>, state: &mut State) -> Result<EscapeSequence, ()> {
    let mut seq = seq;
    seq.push(byte);
    loop {
        let v = get_char(state)?;
        seq.push(v);
        match seq[..seq.len()] {
            [.., 0x07] | [.., 0x9C] | [0x1B, 0x5C] => 
                return Ok(EscapeSequence::UnknownSeq(seq)),
            _ => continue
        }
    }
}

/// Handles [Control Sequence Introducer](https://en.wikipedia.org/wiki/ANSI_escape_code#Control_Sequence_Introducer_commands) sequences 
pub fn handle_csi(byte: u8, seq: Vec<u8>, state: &mut State) -> Result<EscapeSequence, ()> {
    let mut seq = seq;
    seq.push(byte);
    loop {
        let v = get_char(state)?;
        seq.push(v);
        if let Some(0x40..=0x7E) = seq.last() { 
            return Ok(match_csi(seq))
        }
    }
}

/// Returns the correct [EscapeSequence] option for a given CSI escape sequence. 
pub fn match_csi(seq: Vec<u8>) -> EscapeSequence {
    let len = seq.len();
    if len != 3 && len != 4 {
        return EscapeSequence::End;
    }
    match seq[..seq.len()] {
        [0x1B, 0x5B, 0x44] => EscapeSequence::ArrowLeft,
        [0x1B, 0x5B, 0x43] => EscapeSequence::ArrowRight,
        [0x1B, 0x5B, 0x41] => EscapeSequence::ArrowUp,
        [0x1B, 0x5B, 0x42] => EscapeSequence::ArrowDown,
        [0x1B, 0x5B, 0x5A] => EscapeSequence::BackTab,
        [0x1B, 0x5B, 0x31, 0x7E] => EscapeSequence::Home,
        [0x1B, 0x5B, 0x34, 0x7E] => EscapeSequence::End,
        [0x1B, 0x5B, 0x33, 0x7E] => EscapeSequence::Del,
        [0x1B, 0x5B, 0x32, 0x7E] => EscapeSequence::Insert,
        [0x1B, 0x5B, 0x35, 0x7E] => EscapeSequence::PageUp,
        [0x1B, 0x5B, 0x36, 0x7E] => EscapeSequence::PageDown,
        _ => EscapeSequence::UnknownSeq(seq)
    }
}

/// Handles [nF](https://en.wikipedia.org/wiki/ANSI_escape_code#nF_Escape_sequences) escape sequences. 
pub fn handle_nf(byte: u8, seq: Vec<u8>, state: &mut State) -> Result<EscapeSequence, ()> {
    let mut seq = seq;
    seq.push(byte);
    loop {
        let v = get_char(state)?;
        seq.push(v);
        if let Some(0x30..=0x7E) = seq.last() {
            return Ok(EscapeSequence::UnknownSeq(seq))
        }
    }
}

/// Data from reading the serial port
pub enum SerialData {
    Char(u8),
    Nothing,
    Escape(EscapeSequence)
}

impl SerialData {
    /// Gives the correct [SerialData] from [Key].
    pub fn from_console_key(c: Key) -> Self {
        match c {
            Key::Char(c) => SerialData::Char(utils::get_ascii_byte(c)),
            s => SerialData::Escape(EscapeSequence::from_console_key(s))
        }
    }
}

/// Possible escape sequences returned from the parser. 
#[derive(Debug)]
pub enum EscapeSequence {
    Invalid,
    UnknownSeq(Vec<u8>),
    ArrowLeft, 
    ArrowRight,
    ArrowUp,
    ArrowDown,
    BackTab,
    Alt,
    Home,
    End,
    Del,
    Insert,
    PageUp,
    PageDown,
}

impl EscapeSequence {

    /// Gives the correct [EscapeSequence] from [Key].
    pub fn from_console_key(c: Key) -> Self {
        match c {
            Key::UnknownEscSeq(s) => 
                EscapeSequence::UnknownSeq(s.iter().map(|e| utils::get_ascii_byte(*e)).collect()),
            Key::ArrowLeft => EscapeSequence::ArrowLeft,
            Key::ArrowRight => EscapeSequence::ArrowRight,
            Key::ArrowUp => EscapeSequence::ArrowUp,
            Key::ArrowDown => EscapeSequence::ArrowDown,
            Key::BackTab => EscapeSequence::BackTab,
            Key::Alt => EscapeSequence::Alt,
            Key::Home => EscapeSequence::Home,
            Key::End => EscapeSequence::End,
            Key::Del => EscapeSequence::Del,
            Key::Insert => EscapeSequence::Insert,
            Key::PageUp => EscapeSequence::PageUp,
            Key::PageDown => EscapeSequence::PageDown,
            _ => EscapeSequence::Invalid,
        }
    }
}
