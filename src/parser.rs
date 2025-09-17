use std::io::ErrorKind;

use crate::state::State;


pub fn poll_port_parse_data(state: &mut State) -> Result<Parsed, ()> {
    let v = match try_get_char(state)? {
        Some(c) => c,
        None => return Ok(Parsed::Nothing)
    };
    let res = if v == 0x1B {
        Parsed::Escape(handle_escape(state)?)
    } else {
        Parsed::Char(v)
    };

    Ok(res)
}

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

fn get_char(state: &mut State) -> Result<u8, ()> {
    loop {
        match try_get_char(state)? {
            Some(v) => return Ok(v),
            None => continue
        }
    }
}

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

pub fn handle_single(byte: u8, seq: Vec<u8>) -> Result<EscapeSequence, ()> {
    let mut seq = seq;
    seq.push(byte);
    return Ok(EscapeSequence::UnknownSeq(seq))
}

pub fn handle_os(byte: u8, seq: Vec<u8>, state: &mut State) -> Result<EscapeSequence, ()> {
    let mut seq = seq;
    seq.push(byte);
    loop {
        let v = get_char(state)?;
        seq.push(v);
        match seq[..seq.len()] {
            [.., 0x07] | [.., 0x9C] | [0x1B, 0x5C] => return Ok(EscapeSequence::UnknownSeq(seq)),
            _ => continue
        }
    }
}

pub fn handle_csi(byte: u8, seq: Vec<u8>, state: &mut State) -> Result<EscapeSequence, ()> {
    let mut seq = seq;
    seq.push(byte);
    loop {
        let v = get_char(state)?;
        seq.push(v);
        if let Some(0x40..0x7E) = seq.last() { 
            return Ok(match_csi(seq))
        }
    }
}

pub fn match_csi(seq: Vec<u8>) -> EscapeSequence {
    let len = seq.len();
    if len != 3 || len != 4 {
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

pub fn handle_nf(byte: u8, seq: Vec<u8>, state: &mut State) -> Result<EscapeSequence, ()> {
    let mut seq = seq;
    seq.push(byte);
    loop {
        let v = get_char(state)?;
        seq.push(v);
        if let Some(0x30..0x7E) = seq.last() {
            return Ok(EscapeSequence::UnknownSeq(seq))
        }
    }
}

pub enum Parsed {
    Char(u8),
    Nothing,
    Escape(EscapeSequence)
}

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
