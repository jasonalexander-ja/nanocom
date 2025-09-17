//! # nanocom 
//! 
//! A cross platform clone of [picocom](https://github.com/npat-efault/picocom) minimal dumb terminal emulator. 
//! 
//! Tested to work in Linux, mac os and Windows. 
//! 
//! ### Note to picocom/minicom users
//! 
//! As this application uses platform agnostic libraries for serial port access, the `--nolock / -l` 
//! option is not available. If you need such control then you are likely using a platform that can 
//! run picocom and I'd suggest using that. 
//! 
//! There will also be some minor differences between how escape sequences and control characters are handled. 
//! 
//! ## Usage
//! 
//! ```$ nanocom [OPTIONS] <PORT>```
//! 
//! #### Arguments:
//!   `<PORT>`  The name of the serial device to be monitored
//! 
//! #### Options:
//! *  `-b, --baud <BAUD>`          Defines the baud-rate to set the serial-port (terminal) to [default: 9600]
//! *  `-f, --flow <FLOW> `         Defines the flow-control mode to set the serial-port to [default: n] [possible values: x, h, n]
//! *  `-p, --parity <PARITY>`      Defines the flow-control mode to set the serial-port to [default: n] [possible values: o, e, n]
//! *  `-d, --databits <DATABITS>`  Defines the number of data bits in every character [default: 8]
//! *  `-e, --escape <ESCAPE>`      Defines the character that will make nanocom enter command-mode. If 'x' is given, then C-x will make nanocom enter command mode [default: a]
//! *  `-i, --noinit`               If given, nanocom will not initialize, reset, or otherwise meddle with the serial port at start-up. It will just open it. This is useful, for example, for connecting nanocom to already-connected modems, or already configured ports without terminating the connection, or altering the settings. If required serial port parameters can then be adjusted at run-time by commands
//! *  `-r, --noreset`              If given, nanocom will not *reset* the serial port when exiting. It will just close the filedes and do nothing more. This is useful, for example, for leaving modems connected when exiting nanocom using the "Quit" command (instead of "Exit"), which never resets the serial port. If "--noreset" is given then "Quit" and "Exit" behave essentially the same
//! *  `-h, --help`                 Print help (see more with '--help')
//! *  `-V, --version`              Print version
//! 
//! 

use core::str;
use std::io::Write;

use clap::Parser;

use args::Args;
use console::Key;
use state::State;
use inputstream::InputStream;

/// Contains the types for parsing the args at startup. 
pub mod args;
/// Collection of functions and tools for handling run time user commands. 
pub mod commands;
/// General utility types, functions, and constants used in various places. 
pub mod utils;
/// Contains types and methods for manipulating the program state at runtime. 
pub mod state;
/// Contains types and methods for reading in keys and data from the user.  
pub mod inputstream;
/// Contains types and methods for handling output to the terminal. 
pub mod terminal;
/// Contains types and methods for reading data received over the serial port and parsing escape sequences. 
pub mod serial_in;
/// Contains types and methods for acting upon escape sequences. 
pub mod escape_handlers;


/// The entrypoint (duh)
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

/// Main event loop, continuously polls user and serial port input, processing and forwarding data between the 2. 
/// Exiting when a quit command is received or an unrecoverable error is encountered. 
fn main_event_loop(state: &mut State) -> Result<(), ()> {
    let input_stream = InputStream::new(state.escape_code, vec![24, 27]);

    loop {
        poll_input(state, &input_stream)?;
        let res = serial_in::poll_port_parse_data(state)?;
        let _ = terminal::print_data_in(res, state);
    }
}

/// Checks if there is any user input received from the input stream, acting upon it if so or returning if not. 
fn poll_input(state: &mut State, input_stream: &InputStream) -> Result<(), ()> {
    let v = match input_stream.get_char() {
        Some(Ok(v)) => v,
        Some(Err(_)) => {
            println!("*** Input stream disconnected exiting. ");
            return Err(());
        },
        None => return  Ok(())
    };
    if let Err(HandleInputError::Shutdown) = handle_input(v, state, &input_stream) {
        return Err(());
    }
    Ok(())
}

/// Handles a key received from the input. 
fn handle_input(key: Key, 
    state: &mut State, 
    input_stream: &InputStream) -> Result<(), HandleInputError> 
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
            Err(HandleInputError::FailedToWrite)
        }
    }
}

/// Type used when an error is encountered during handling a key in. 
enum HandleInputError {
    /// Unrecoverable error is encountered or user has signalled a shutdown. 
    Shutdown,
    /// An error has been encountered, but the program should continue. 
    Recoverable,
    /// Failed to write to the serial port. 
    FailedToWrite,
}
