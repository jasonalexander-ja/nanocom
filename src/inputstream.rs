use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};

use super::utils;
use super::key::KeyIn;

use console::Term;

/// Struct to hold the handle for the user key in polling routine and the receiver from that
/// routine. 
pub struct InputStream {
    _handle: JoinHandle<()>,
    char_recv: Receiver<KeyIn>,
    _is_connected_sender: Sender<()>
}

impl InputStream {

    /// Started a new input stream service. 
    /// 
    /// #
    /// 
    /// * `escape` - The escape to enter command mode. 
    /// * `shutdown_vals` - A list of bytes that would trigger a shutdown. 
    pub fn new(escape: u8, shutdown_vals: Vec<u8>) -> InputStream {
        let (char_sender, char_recv) = mpsc::channel::<KeyIn>();
        let (is_connected_sender, is_connected_rev) = mpsc::channel::<()>();
        let shutdown_chars: Vec<char> = shutdown_vals.iter()
            .map(|v| *v as char)
            .collect();

        let handle = thread::spawn(move || 
            input_stream_loop(escape, shutdown_chars, char_sender, is_connected_rev));

        InputStream { 
            _handle: handle, 
            char_recv, 
            _is_connected_sender: is_connected_sender 
        }
    }

    /// Non-blocking polls the receivers for if a key has been received. 
    pub fn get_char(&self) -> Option<Result<KeyIn, ()>> {
        match self.char_recv.try_recv() {
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => Some(Err(())),
            Ok(c) => Some(Ok(c))
        }
    }
    
    /// Blocking gets a line from the user input. 
    pub fn get_line(&self) -> Result<String, ()> {
        let mut result: Vec<char>  = vec![];
        loop {
            if get_char_blocking(&mut result, &self.char_recv)? {
                return Ok(result.iter().collect())
            }
        }
    }
}

/// Blocking gets a character from the character receiver and adds it to a buffer, returning whether 
/// it was an enter. 
fn get_char_blocking(result: &mut Vec<char>, char_recv: &Receiver<KeyIn>) -> Result<bool, ()> {
    match char_recv.recv() {
        Ok(KeyIn::Char(13)) | Ok(KeyIn::Char(10)) =>  return Ok(true),
        Ok(KeyIn::Char(127)) | Ok(KeyIn::Char(8)) => {
            let _ = result.pop();
            utils::del_char();
        },
        Ok(KeyIn::Char(x)) => {
            result.push(x as char);
            utils::put_char(x as char);
        },
        Err(_) => return Err(()),
        _ => return Ok(false)
    };
    return Ok(false)
}

/// Main loop which sends any keys received from the user input to a channel. 
/// * `escape` - The escape to enter command mode. 
/// * `shutdown_vals` - A list of bytes that would trigger a shutdown (so it knows to exit). 
fn input_stream_loop(escape: u8, shutdown_chars: Vec<char>, char_sender: Sender<KeyIn>, is_connected_rev: Receiver<()>) {
    let term = Term::stdout();
    let mut is_escaped = false;
    loop {
        if let Err(TryRecvError::Disconnected) = is_connected_rev.try_recv() { return };
        let c = match term.read_key_raw() {
            Ok(c) => c,
            Err(_) => return
        };
        let key = KeyIn::from_console_key(&c);
        if let Err(_) = char_sender.send(key.clone()) { return };
        if let KeyIn::Char(c) = key {
            if shutdown_chars.contains(&(c as char)) && is_escaped { return }
            is_escaped = !is_escaped && c == escape;
        } else {
            is_escaped = false;
        };
    }
}
