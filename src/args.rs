use std::fmt::{self, Display};
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Defines the baud-rate to set the serial-port (terminal) to. 
    #[arg(long, default_value_t = 9600, short)]
    pub baud: u32,
    /// Defines the flow-control mode to set the serial-port to.
    #[arg(long, default_value_t = FlowControl::n, short)]
    pub flow: FlowControl,
    /// Defines the flow-control mode to set the serial-port to.
    #[arg(long, default_value_t = Parity::n, short)]
    pub parity: Parity,
    /// Defines the number of data bits in every character.
    #[arg(value_parser = clap::value_parser!(u8).range(5..=8), long, default_value_t = 8, short)]
    pub databits: u8,
    /// Defines the character that will make nanocom enter command-mode. 
    /// If 'x' is given, then C-x will make nanocom enter command mode.
    #[arg(long, default_value_t = 'a', short)]
    pub escape: char,
    /// If given, nanocom will not initialize, reset, or otherwise meddle with the serial port at start-up. 
    /// It will just open it. This is useful, for example, for connecting nanocom to already-connected modems, 
    /// or already configured ports without terminating the connection, or altering the settings. If required 
    /// serial port parameters can then be adjusted at run-time by commands. 
    #[arg(long, short = 'i')]
    pub noinit: bool,
    /// If given, nanocom will not *reset* the serial port when exiting. It will just close the filedes 
    /// and do nothing more. This is useful, for example, for leaving modems connected when exiting nanocom 
    /// using the "Quit" command (instead of "Exit"), which never resets the serial port. If "--noreset" is 
    /// given then "Quit" and "Exit" behave essentially the same. 
    #[arg(long, short = 'r')]
    pub noreset: bool,
    /// If given, nanocom will *not* attempt to lock the serial port before opening it. Normally nanocom 
    /// attempts to get a UUCP-style lock-file (e.g. "/var/lock/LCK..ttyS0") before opening the port. 
    /// Failing to do so, results in the program exiting after emitting an error-message. It is possible 
    /// that your nanocom binary is compiled without this option. 
    #[arg(long, short = 'l')]
    pub nolock: bool,
    /// The name of the serial device to be monitored. 
    pub port: String
}

impl Args {
    pub fn show_state(&self) -> String {
        format!(
            "port is        : {}\n\r\
            flowcontrol    : {}\r\n\
            baudrate is    : {}\r\n\
            parity is      : {}\r\n\
            databits are   : {}\r\n\
            escape is      : C-{}\r\n\
            noinit is      : {}\r\n\
            noreset is     : {}\r\n\
            nolock is      : {}\r\n
            ",
            self.port,
            self.flow.show(),
            self.baud,
            self.parity.show(),
            self.databits,
            self.escape,
            if self.noinit { "no" } else { "yes" },
            if self.noreset { "no" } else { "yes" },
            if self.nolock { "no" } else { "yes" }
        )
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Parser)]
pub enum FlowControl {
    /// xon/xoff (software) mode
    x,
    /// hardware flow control (RTS/CTS)
    h,
    /// no flow control 
    n,
}

impl FlowControl {
    pub fn to_serialport(&self) -> serialport::FlowControl {
        match self {
            FlowControl::h => serialport::FlowControl::Hardware,
            FlowControl::x => serialport::FlowControl::Software,
            FlowControl::n => serialport::FlowControl::None
        }
    }

    pub fn show(&self) -> &str {
        match self {
            FlowControl::x => "xon/xoff",
            FlowControl::h => "hardware",
            FlowControl::n => "none",
        }
    }
}

impl Display for FlowControl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlowControl::x => write!(f, "x"),
            FlowControl::h => write!(f, "h"),
            FlowControl::n => write!(f, "n"),
        }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Parity {
    /// odd parity mode
    o,
    /// for even parity mode
    e,
    /// no parity mode
    n,
}

impl Parity {
    pub fn to_serialport(&self) -> serialport::Parity {
        match self {
            Parity::o => serialport::Parity::Odd,
            Parity::e => serialport::Parity::Even,
            Parity::n => serialport::Parity::None
        }
    }

    pub fn show(&self) -> &str {
        match self {
            Parity::o => "odd",
            Parity::e => "even",
            Parity::n => "none",
        }
    }
}

impl Display for Parity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Parity::o => write!(f, "o"),
            Parity::e => write!(f, "e"),
            Parity::n => write!(f, "n"),
        }
    }
}
