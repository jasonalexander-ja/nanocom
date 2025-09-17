use core::str;
use std::io::Write;

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
pub mod parser;
pub mod escape;


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

    let _ = main_event_loop(&mut state);
}

fn main_event_loop(state: &mut State) -> Result<(), ()> {
    let input_stream = InputStream::new(state.escape_code, vec![24, 27]);

    loop {
        poll_input(state, &input_stream)?;
        let res = parser::poll_port_parse_data(state)?;
        let _ = terminal::print_data_in(res, state);
    }
}

fn poll_input(state: &mut State, input_stream: &InputStream) -> Result<(), ()> {
    let v = match input_stream.get_char() {
        Some(Ok(v)) => v,
        Some(Err(_)) => {
            println!("*** Input stream disconected exiting. ");
            return Err(());
        },
        None => return  Ok(())
    };
    if let Err(HandleError::Shutdown) = handle_input(v, state, &input_stream) {
        return Err(());
    }
    Ok(())
}

fn handle_input(key: Key, 
    state: &mut State, 
    input_stream: &InputStream) -> Result<(), HandleError> 
{
    let seq = inputstream::get_key_sequence(key);
    if seq.len() == 0 { return Ok(()); }
    
    if commands::handle_escape(&seq, state, input_stream)? { return Ok(()) }
    if state.local_echo {
        for i in &seq {
            let _ = terminal::print_char(*i, state);
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
