use console::Term;
use serialport::{DataBits, FlowControl, SerialPort};

use super::utils::get_ascii_byte;
use super::args::Args;


/// Model containing all the settings and mutable aspects of the program state. 
pub struct State {
    /// The escape character to enter command mode. 
    pub escape: char,
    /// The ascii byte code of the escape character to enter command mode. 
    pub escape_code: u8,
    /// If true, initialize the serial port. 
    pub noinit: bool,
    /// Don't reset the serial port. 
    pub noreset: bool,
    /// Is the data terminal read up. 
    pub dtr: bool,
    /// Is the ready to send up. 
    pub rts: bool,
    /// The current selected flow control, this may be different from what is being used
    /// if it's unsupported by the serial device, this is mainly for tracking cycling through 
    /// options. 
    pub flow: FlowControl,
    /// Is program in command mode. 
    pub command_mode: bool,
    /// Local echo (send characters to terminal as they're typed).
    pub local_echo: bool,
    /// Name of the port on the OS.
    pub port_name: String,
    /// The serial port wrapper. 
    pub port: Box<dyn SerialPort>,
    /// Terminal interface wrapper. 
    pub term: Term
}

impl State {

    /// Generates a new state from the given start arguments. 
    pub fn new_from_args(args: &Args) -> Result<State, ()> {
        let port = match get_serial_port(&args) {
            Ok(v) => v,
            Err(_) => return Err(())
        };
        let escape_code = get_ascii_byte(args.escape.to_ascii_lowercase()) - 96;
        let term = Term::stdout();


        Ok(State {
            escape: args.escape,
            escape_code,
            noinit: args.noinit,
            noreset: args.noreset,
            dtr: false,
            rts: false,
            flow: args.flow.to_serialport(),
            command_mode: false,
            local_echo: false,
            port_name: args.port.clone(),
            port: port,
            term
        })
    }

    /// Generates a human readable message string of all the configurable port settings. 
    pub fn port_settings(&self) -> serialport::Result<String> {
        let res = format!("*** baud: {}\r\n\
            **** flow: {}\r\n\
            **** parity: {}\r\n\
            **** databits: {}\r\n\
            **** stopbits: {}\r\n\
            **** dtr: {}\r\n\
            **** rts: {}\r\n\
            ",
            self.port.baud_rate()?,
            self.port.flow_control()?,
            self.port.parity()?,
            self.port.data_bits()?,
            self.port.stop_bits()?,
            if self.dtr { "up" } else { "down" },
            if self.rts { "up" } else { "down" },
        );
        Ok(res)
    }
}

/// Tries to configure and open a serial port based on the passed settings. 
fn get_serial_port(args: &Args) -> Result<Box<dyn SerialPort>, ()> {
    let databits = match args.databits {
        5 => DataBits::Five,
        6 => DataBits::Six,
        7 => DataBits::Seven,
        _ => DataBits::Eight,
    };

    let port_builder = if args.noinit { 
        serialport::new(&args.port.clone(), args.baud)
            .preserve_dtr_on_open()
    } else {
        serialport::new(&args.port.clone(), args.baud)
            .flow_control(args.flow.to_serialport())
            .parity(args.parity.to_serialport())
            .dtr_on_open(true)
            .data_bits(databits)
    };

    match port_builder.open() {
        Ok(p) => Ok(p),
        Err(e) => { 
            println!("*** Failed to open serial port, reason: \r\n{}", e.description);
            return Err(());
        }
    }
}
