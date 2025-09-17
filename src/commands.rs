use std::{thread, time::Duration};

use serialport::{DataBits, FlowControl, Parity, StopBits};

use crate::utils::{put_str, BAUDS};
use super::{State, HandleInputError, InputStream};


/// Handles user input if the escape character has been received and the program should enter/is in
/// command mode, dispatching any commands. 
pub(crate) fn handle_escape(seq: &Vec<u8>, state: &mut State, input_stream: &InputStream) -> Result<bool, HandleInputError> {
    if seq.len() != 1 { return Ok(false); }
    let (key, keychar) = (seq[0], seq[0] as char);
    
    if !keychar.is_ascii_control() { 
        state.command_mode = false;
        return Ok(false); 
    }
    if state.command_mode {
        state.command_mode = false;
        let _result = handle_command(key, state, input_stream)?;
        return Ok(true); 
    }
    if key == state.escape_code {
        state.command_mode = true;
        return Ok(true);
    }
    return Ok(false)
}

/// Handles the command input from the user, dispatching the correct routine. 
/// Throws [HandleInputError::Shutdown] is the user sends a shutdown command. 
fn handle_command(command: u8, state: &mut State, input_stream: &InputStream) -> Result<(), HandleInputError> {
    match command {
        24 => Err(HandleInputError::Shutdown),
        17 => {
            state.noreset = true;
            Err(HandleInputError::Shutdown)
        },
        2 => set_baudrate(state, input_stream),
        21 => increase_baudrate(state),
        4 => decrease_baudrate(state),
        9 => change_databits(state),
        10 => change_stopbits(state),
        6 => change_flowcontrol(state),
        25 => change_parity(state),
        16 => pulse_dtr(state),
        20 => toggle_dtr(state),
        7 => toggle_rts(state),
        3 => toggle_local_echo(state),
        22 => show_port_settings(state),
        8 | 127 => help_message(state),
        _ => Ok(())
    }
}

/// Polls the user for a new (valid) baudrate, and updates the serial port settings. 
fn set_baudrate(state: &mut State, input_stream: &InputStream) -> Result<(), HandleInputError> {
    loop {
        put_str("\r\n\r\n*** baud: ");
        let line = match input_stream.get_line() {
            Ok(s) => s,
            Err(_) => {
                println!("\r\n*** Failed to read from console, exiting. \r\n");
                return Err(HandleInputError::Shutdown)
            }
        };
        let baud = match u32::from_str_radix(&line, 10) {
            Ok(v) => v,
            Err(_) => {
                println!("\r\n*** Please enter a valid number\r\n");
                continue;
            }
        };
        return update_baud(baud, state);
    }
}

/// Selects the next highest standard baudrate as defined in [BAUDS], and sets the serial port settings to that. 
fn increase_baudrate(state: &mut State) -> Result<(), HandleInputError> {
    let baud = get_baud(state)?;
    for i in BAUDS.iter().rev() {
        if *i > baud {
            return update_baud(*i, state)
        }
    }
    Ok(())
}

/// Selects the next lowest standard baudrate as defined in [BAUDS], and sets the serial port settings to that. 
fn decrease_baudrate(state: &mut State) -> Result<(), HandleInputError> {
    let baud = get_baud(state)?;
    for i in BAUDS.iter() {
        if *i < baud {
            return update_baud(*i, state)
        }
    }
    Ok(())
}

/// Helper function to set the baudrate, and showing an error message and throwing if failing to do so. 
fn update_baud(baud: u32, state: &mut State) -> Result<(), HandleInputError> {
    match state.port.set_baud_rate(baud) {
        Ok(_) => { 
            println!("\r\n*** baud: {} ***\r\n", baud);
            return Ok(());
        },
        Err(_) => {
            println!("\r\n*** Failed to set baud \r\n");
            return Err(HandleInputError::Recoverable);
        }
    }
}

/// Cycles through to the next character length, showing an error message and 
/// throwing is failing to set it in the settings. 
/// 
/// * [DataBits::Five] => [DataBits::Six]
/// * [DataBits::Six] => [DataBits::Seven]
/// * [DataBits::Seven] => [DataBits::Eight]
/// * [DataBits::Eight] => [DataBits::Five]
fn change_databits(state: &mut State) -> Result<(), HandleInputError> {
    let databits = get_databits(state)?;

    let new_databits = match databits {
        DataBits::Five => DataBits::Six,
        DataBits::Six => DataBits::Seven,
        DataBits::Seven => DataBits::Eight,
        DataBits::Eight => DataBits::Five,
    };

    match state.port.set_data_bits(new_databits) {
        Ok(_) => {
            println!("\r\n*** databits: {} ***\r\n", new_databits);
            Ok(())
        },
        Err(_) => {
            println!("\r\n*** Failed to write data bits \r\n");
            Err(HandleInputError::Recoverable)
        }
    }
}

/// Switches between 1 and 2 stop bits in the serial port settings. 
fn change_stopbits(state: &mut State) -> Result<(), HandleInputError> {
    let stopbits = get_stopbits(state)?;

    let new_stopbits = match stopbits {
        StopBits::One => StopBits::Two,
        StopBits::Two => StopBits::One,
    };

    match state.port.set_stop_bits(new_stopbits) {
        Ok(_) => {
            println!("\r\n*** stopbits: {} ***\r\n", new_stopbits);
            Ok(())
        },
        Err(_) => {
            println!("\r\n*** Failed to write stop bits \r\n");
            Err(HandleInputError::Recoverable)
        }
    }
}

/// Cycles through to the next flow control option, showing an error message and 
/// throwing is failing to set it in the settings. 
/// 
/// * [FlowControl::None] => [FlowControl::Software]
/// * [FlowControl::Software] => [FlowControl::Hardware]
/// * [FlowControl::Hardware] => [FlowControl::None]
fn change_flowcontrol(state: &mut State) -> Result<(), HandleInputError>  {
    let flowcontrol = get_flow_control(state)?;

    let new_flowcontrol = match flowcontrol {
        FlowControl::None => FlowControl::Software,
        FlowControl::Software => FlowControl::Hardware,
        FlowControl::Hardware => FlowControl::None
    };

    match state.port.set_flow_control(new_flowcontrol) {
        Ok(_) => {
            println!("\r\n*** flow: {} ***\r\n", new_flowcontrol);
            Ok(())
        },
        Err(_) => {
            println!("\r\n*** Failed to write flow control \r\n");
            Err(HandleInputError::Recoverable)
        }
    }
}

/// Cycles through to the next parity option, showing an error message and 
/// throwing is failing to set it in the settings. 
/// 
/// * [Parity::None] => [Parity::Odd]
/// * [Parity::Odd] => [Parity::Even]
/// * [Parity::Even] => [Parity::None]
fn change_parity(state: &mut State) -> Result<(), HandleInputError> {
    let parity = get_parity(state)?;

    let new_parity = match parity {
        Parity::None => Parity::Odd,
        Parity::Odd => Parity::Even,
        Parity::Even => Parity::None
    };

    match state.port.set_parity(new_parity) {
        Ok(_) => {
            println!("\r\n*** parity: {} ***\r\n", new_parity);
            Ok(())
        },
        Err(_) => {
            println!("\r\n*** Failed to write parity \r\n");
            Err(HandleInputError::Recoverable)
        }
    }
}

/// Brings DTR (data terminal ready) down on the serial port for 1 second and brings it up. 
fn pulse_dtr(state: &mut State) -> Result<(), HandleInputError> {
    match state.port.write_data_terminal_ready(false) {
        Ok(_) => {
            state.dtr = false;
            println!("\r\n*** dtr: down ***\r\n");
        },
        Err(_) => {
            println!("\r\n*** Failed to lower dtr \r\n");
            return Err(HandleInputError::Recoverable)
        }
    }
    thread::sleep(Duration::from_secs(1));
    match state.port.write_data_terminal_ready(true) {
        Ok(_) => {
            state.dtr = true;
            println!("\r\n*** dtr: up ***\r\n");
            Ok(())
        },
        Err(_) => {
            println!("\r\n*** Failed to raise dtr \r\n");
            Err(HandleInputError::Recoverable)
        }
    }
}

/// Toggles DTR (data terminal ready). 
fn toggle_dtr(state: &mut State) -> Result<(), HandleInputError>  {

    match state.port.write_data_terminal_ready(!state.dtr) {
        Ok(_) => {
            state.dtr = !state.dtr;
            println!("\r\n*** dtr: {} ***\r\n", if state.dtr { "up" } else { "down" });
            Ok(())
        },
        Err(_) => {
            println!("\r\n*** Failed to toggle dtr \r\n");
            Err(HandleInputError::Recoverable)
        }
    }
}

/// Toggles RTS (ready to send).
fn toggle_rts(state: &mut State) -> Result<(), HandleInputError>  {

    match state.port.write_request_to_send(!state.rts) {
        Ok(_) => {
            state.rts = !state.rts;
            println!("\r\n*** rts: {} ***\r\n", if state.rts { "up" } else { "down" });
            Ok(())
        },
        Err(_) => {
            println!("\r\n*** Failed to toggle rts \r\n");
            Err(HandleInputError::Recoverable)
        }
    }
}

/// Toggles local echo on/off, terminal will start printing out all characters typed if on. 
fn toggle_local_echo(state: &mut State) -> Result<(), HandleInputError>  {
    state.local_echo = !state.local_echo;
    let msg = if state.local_echo { "on" } else { "off" };
    println!("\r\n*** local echo: {} ***\r\n", msg);
    Ok(())
}

/// Prints out a message of all the current serial port settings. 
fn show_port_settings(state: &mut State) -> Result<(), HandleInputError> {
    let baud = get_baud(state)?;
    let flowcontrol = get_flow_control(state)?;
    let parity = get_parity(state)?;
    let databits = get_databits(state)?;
    let stopbits = get_stopbits(state)?;
    println!("\r\n\
        *** baud: {}\r\n\
        *** flow: {}\r\n\
        *** parity: {}\r\n\
        *** databits: {}\r\n\
        *** stopbits: {}\r\n\
        *** dtr: {}\r\n\
        *** rts: {}\r\n\
        ",
        baud,
        flowcontrol,
        parity,
        databits,
        stopbits,
        if state.dtr { "up" } else { "down" },
        if state.rts { "up" } else { "down" }
    );
    Ok(())
}

/// Prints out a help message of all the key bindings for each command. 
fn help_message(state: &mut State) -> Result<(), HandleInputError>  {
    println!("\r\n*** nanocom commands (all prefixed by [C-{}])\r\n\
        \r\n\
        *** [C-x] : Exit nanocom\r\n\
        *** [C-q] : Exit without resetting serial port\r\n\
        *** [C-b] : Set baudrate\r\n\
        *** [C-u] : Increase baudrate (baud-up)\r\n\
        *** [C-d] : Decrease baudrate (baud-down)\r\n\
        *** [C-i] : Change number of databits\r\n\
        *** [C-j] : Change number of stopbits\r\n\
        *** [C-f] : Change flow-control mode\r\n\
        *** [C-y] : Change parity mode\r\n\
        *** [C-p] : Pulse DTR\r\n\
        *** [C-t] : Toggle DTR\r\n\
        *** [C-g] : Toggle RTS\r\n\
        *** [C-c] : Toggle local echo\r\n\
        *** [C-v] : Show port settings\r\n\
        *** [C-h] : Show this message\r\n\
    ", state.escape);
    Ok(())
}

/// Gets the current baud setting from the serial port. 
fn get_baud(state: &mut State) -> Result<u32, HandleInputError> {
    match state.port.baud_rate() {
        Ok(v) => Ok(v),
        Err(_) => {
            println!("\r\n*** Failed to read baud \r\n");
            Err(HandleInputError::Recoverable)
        }
    }
}

/// Gets the current flow control setting from the serial port. 
fn get_flow_control(state: &mut State) -> Result<FlowControl, HandleInputError> {
    match state.port.flow_control() {
        Ok(v) => Ok(v),
        Err(_) => {
            println!("\r\n*** Failed to read flow control \r\n");
            Err(HandleInputError::Recoverable)
        }
    }
}

/// Gets the current parity setting from the serial port. 
fn get_parity(state: &mut State) -> Result<Parity, HandleInputError> {
    match state.port.parity() {
        Ok(v) => Ok(v),
        Err(_) => {
            println!("\r\n*** Failed to read parity \r\n");
            Err(HandleInputError::Recoverable)
        }
    }
}

/// Gets the current databits setting from the serial port. 
fn get_databits(state: &mut State) -> Result<DataBits, HandleInputError> {
    match state.port.data_bits() {
        Ok(v) => Ok(v),
        Err(_) => {
            println!("\r\n*** Failed to write data bits \r\n");
            Err(HandleInputError::Recoverable)
        }
    }
}
/// Gets the current stop bits setting from the serial port. 
fn get_stopbits(state: &mut State) -> Result<StopBits, HandleInputError> {
    match state.port.stop_bits() {
        Ok(v) => Ok(v),
        Err(_) => {
            println!("\r\n*** Failed to read stop bits \r\n");
            Err(HandleInputError::Recoverable)
        }
    }
}
