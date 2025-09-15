use std::io::Write;
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};

use super::utils::get_ascii_byte;

use console::{Key, Term};

pub struct InputStream {
    handle: JoinHandle<()>,
    char_recv: Receiver<Key>,
    shtdwn_sender: Sender<()>
}

impl InputStream {
    pub fn new() -> InputStream {
        let (char_sender, char_recv) = mpsc::channel::<Key>();
        let (shtdwn_sender, shtdwn_recv) = mpsc::channel::<()>();

        let handle = thread::spawn(move || {
            let term = Term::stdout();
            loop {
                let c = match term.read_key_raw() {
                    Ok(c) => c,
                    Err(_) => return
                };
                match shtdwn_recv.try_recv() {
                    Err(TryRecvError::Disconnected) | Ok(_) => return,
                    _ => () 
                }
                if let Err(_) = char_sender.send(c) { return };
            }
        });

        InputStream { handle, char_recv, shtdwn_sender }
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
                    print!("{}", x);
                    let _ = std::io::stdout().flush();
                },
                Ok(Key::Backspace) => {
                    let _ = result.pop();
                    print!("\x08 \x08");
                    let _ = std::io::stdout().flush();
                },
                Ok(Key::Enter) =>  return Ok(result.iter().collect()),
                Err(_) => return Err(()),
                _ => continue
            };
        }
    }

    pub fn cleanup(self) {
        let _ = self.shtdwn_sender.send(());
        let _ = self.handle.join();
    }
}

pub fn get_sequence(key: Key) -> Vec<u8> {
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

