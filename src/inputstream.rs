use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};

use super::utils::{self, get_ascii_byte};

use console::{Key, Term};

pub struct InputStream {
    _handle: JoinHandle<()>,
    char_recv: Receiver<Key>,
}

impl InputStream {
    pub fn new(escape: u8, shutdown_vals: Vec<u8>) -> InputStream {
        let (char_sender, char_recv) = mpsc::channel::<Key>();
        let shutdown_chars: Vec<char> = shutdown_vals.iter()
            .map(|v| *v as char)
            .collect();

        let handle = thread::spawn(move || input_stream_loop(escape, shutdown_chars, char_sender));

        InputStream { _handle: handle, char_recv }
    }

    pub fn get_char(&self) -> Option<Result<Key, ()>> {
        match self.char_recv.try_recv() {
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => Some(Err(())),
            Ok(c) => Some(Ok(c))
        }
    }
    
    pub fn get_line(&self) -> Result<String, ()> {
        let mut result: Vec<char>  = vec![];
        loop {
            match self.char_recv.recv() {
                Ok(Key::Char(x)) => {
                    result.push(x);
                    utils::put_char(x);
                },
                Ok(Key::Backspace) => {
                    let _ = result.pop();
                    utils::del_char();
                },
                Ok(Key::Enter) =>  return Ok(result.iter().collect()),
                Err(_) => return Err(()),
                _ => continue
            };
        }
    }
}

fn input_stream_loop(escape: u8, shutdown_chars: Vec<char>, char_sender: Sender<Key>) {
    let term = Term::stdout();
    let mut is_escaped = false;
    loop {
        let c = match term.read_key_raw() {
            Ok(c) => c,
            Err(_) => return
        };
        if let Err(_) = char_sender.send(c.clone()) { return };
        if let Key::Char(c) = c {
            if shutdown_chars.contains(&c) && is_escaped { return }
            is_escaped = c == escape as char;
        } else {
            is_escaped = false;
        };
    }
}

#[cfg(target_os = "windows")]
pub fn get_key_sequence(key: Key) -> Vec<u8> {
    match key {
        Key::UnknownEscSeq(s) => s.iter().map(|a| get_ascii_byte(*a)).collect(),
        Key::ArrowLeft => vec![27, 91, 68],
        Key::ArrowRight => vec![27, 91, 67],
        Key::ArrowUp => vec![27, 91, 65],
        Key::ArrowDown => vec![27, 91, 66],
        Key::Enter => vec![13],
        Key::Escape => vec![27],
        Key::Backspace => vec![127],
        Key::Home => vec![27, 91, 49, 126],
        Key::End => vec![27, 91, 52, 126],
        Key::Tab => vec![9],
        Key::BackTab => vec![27, 91, 90],
        Key::Alt => vec![27, 91, 90],
        Key::Del => vec![27, 91, 51, 126],
        Key::Insert => vec![27, 91, 50, 126],
        Key::PageUp => vec![27, 91, 53, 126],
        Key::PageDown => vec![27, 91, 54, 126],
        Key::Char(c) => vec![get_ascii_byte(c)],
        Key::CtrlC => vec![3],
        _ => vec![]
    }
}

#[cfg(not(target_os = "windows"))]
pub fn get_key_sequence(key: Key) -> Vec<u8> {
    match key {
        Key::UnknownEscSeq(s) => s.iter().map(|a| get_ascii_byte(*a)).collect(),
        Key::ArrowLeft => vec![27, 91, 68],
        Key::ArrowRight => vec![27, 91, 67],
        Key::ArrowUp => vec![27, 91, 65],
        Key::ArrowDown => vec![27, 91, 66],
        Key::Enter => vec![13],
        Key::Escape => vec![27],
        Key::Backspace => vec![127],
        Key::Home => vec![1],
        Key::End => vec![5],
        Key::Tab => vec![9],
        Key::BackTab => vec![27, 91, 90],
        Key::Alt => vec![27, 91, 90],
        Key::Del => vec![27, 91, 51, 126],
        Key::Insert => vec![27, 91, 50, 126],
        Key::PageUp => vec![27, 91, 53, 126],
        Key::PageDown => vec![27, 91, 54, 126],
        Key::Char(c) => vec![get_ascii_byte(c)],
        Key::CtrlC => vec![3],
        _ => vec![]
    }
}
