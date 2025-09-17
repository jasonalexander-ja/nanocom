use console::Term;
use serialport::SerialPort;

use super::utils::get_ascii_byte;
use super::args::Args;


pub struct State {
    pub escape: char,
    pub escape_code: u8,
    pub noinit: bool,
    pub noreset: bool,
    pub dtr: bool,
    pub rts: bool,
    pub command_mode: bool,
    pub local_echo: bool,
    pub port_name: String,
    pub port: Box<dyn SerialPort>,
    pub term: Term
}

impl State {
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
            command_mode: false,
            local_echo: false,
            port_name: args.port.clone(),
            port: port,
            term
        })
    }

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

fn get_serial_port(args: &Args) -> Result<Box<dyn SerialPort>, ()> {
    let port_builder = if args.noinit { 
        serialport::new(&args.port.clone(), args.baud)
            .preserve_dtr_on_open()
    } else {
        serialport::new(&args.port.clone(), args.baud)
            .flow_control(args.flow.to_serialport())
            .parity(args.parity.to_serialport())
    };

    match port_builder.open() {
        Ok(p) => Ok(p),
        Err(e) => { 
            println!("*** Failed to open serial port, reason: \r\n{}", e.description);
            return Err(());
        }
    }
}
