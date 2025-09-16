use std::io::{ErrorKind, Write};

use clap::Parser;

use args::Args;
use console::Key;
use state::State;
use inputstream::InputStream;

/// Contains types for parsing the args at startup. 
pub mod args;
pub mod commands;
pub mod utils;
pub mod state;
pub mod inputstream;
pub mod terminal;


fn main() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("nanocom v{}\r\n", VERSION);

    let args = Args::parse();

    let mut state = match State::new_from_args(&args) {
        Ok(v) => v,
        Err(_) => return
    };

    println!("{}", args.show_state());

    println!("Type [C-{}] [C-h] to see available commands", args.escape);
    println!("Terminal ready");

    main_event_loop(&mut state);
}

fn main_event_loop(state: &mut State) {
    let input_stream = InputStream::new(state.escape_code, vec![24, 27]);

    loop {
        if let Some(v) = input_stream.get_char() {
            let c = match v {
                Ok(c) => c,
                Err(_) => {
                    println!("*** Input stream disconected exiting. ");
                    break;
                }
            };
            let result = handle_input(c, state, &input_stream);
            if let Err(HandleError::Shutdown) = result {
                break;
            }
        }
        match try_get_char(state) {
            Ok(Some(c)) => terminal::print_char(c),
            Ok(None) => continue,
            Err(_) => break
        }
    }
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

fn handle_input(key: Key, 
    state: &mut State, 
    input_stream: &InputStream) -> Result<(), HandleError> 
{
    let seq = inputstream::get_key_sequence(key);
    if seq.len() == 0 { return Ok(()); }
    if vec![27, 91, 50, 56, 126] == seq  { return Ok(()); }
    if commands::handle_escape(&seq, state, input_stream)? { return Ok(()) }
    if state.local_echo {
        for i in seq.iter() {
            terminal::print_char(*i);
        }
    }
    match state.port.write(&seq) {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("\r\n*** Failed to write to port, reason: \r\n{}", e);
            Err(HandleError::FailedToWrite)
        }
    }
}

pub enum HandleError {
    Shutdown,
    Recoverable,
    FailedToWrite,
}
