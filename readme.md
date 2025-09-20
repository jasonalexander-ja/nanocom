# nanocom 

[![crates.io](https://img.shields.io/crates/v/nanocom-term)](https://crates.io/crates/nanocom-term)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A cross platform clone of [picocom](https://github.com/npat-efault/picocom) minimal dumb terminal emulator. 

Tested to work in Linux, mac os and Windows. 

## Installation 

Ensure you have cargo installed. 

```
cargo install nanocom-term
```

### Note to picocom/minicom users

As this application uses platform agnostic libraries for serial port access, the `--nolock / -l` 
option is not available. If you need such control then you are likely using a platform that can 
run picocom and I'd suggest using that. 

There will also be some minor differences between how escape sequences and control characters 
are handled, and file send/receive is not yet implemented (still deciding how best to do that). 

## Usage

```$ nanocom [OPTIONS] <PORT>```

#### Arguments:
  `<PORT>`  The name of the serial device to be monitored

#### Options:

*  `-b, --baud <BAUD>`          Defines the baud-rate to set the serial-port (terminal) to [default: 9600]

*  `-f, --flow <FLOW> `         Defines the flow-control mode to set the serial-port to [default: n] 
                                [possible values: x, h, n]
*  `-p, --parity <PARITY>`      Defines the flow-control mode to set the serial-port to [default: n] 
                                [possible values: o, e, n]

*  `-d, --databits <DATABITS>`  Defines the number of data bits in every character [default: 8]

*  `-e, --escape <ESCAPE>`      Defines the character that will make nanocom enter command-mode. 
                                If 'x' is given, then C-x will make nanocom enter command mode [default: a]

*  `-i, --noinit`               If given, nanocom will not initialize, reset, or otherwise meddle 
                                with the serial port at start-up. It will just open it. This is useful, 
                                for example, for connecting nanocom to already-connected modems, 
                                or already configured ports without terminating the connection, 
                                or altering the settings. If required serial port parameters can then 
                                be adjusted at run-time by commands

*  `-r, --noreset`              If given, nanocom will not *reset* the serial port when exiting. 
                                It will just close the filedes and do nothing more. This is useful, 
                                for example, for leaving modems connected when exiting nanocom using the 
                                "Quit" command (instead of "Exit"), which never resets the serial port. 
                                If "--noreset" is given then "Quit" and "Exit" behave essentially the same

*  `-h, --help`                 Print help (see more with `--help`)

*  `-V, --version`              Print version

### Runtime Control

When the application is running, some settings can be changed, this is done by entering command 
mode by pressing `Ctrl + [Escape Key]`, the escape key is determined by the `--escape / -e` flag at 
startup or is default to `a`. 

If you wish to send a `Ctrl` character into the serial port that is the same as the command key, 
pressing the same command key again will send that character to the serial port. 

Once command mode is entered, a command is given by pressing `Ctrl` and then a key mapped to a 
command, for picocom users, **all** of these commands are the same, with `Ctrl s + r + \` being absent. 

A confirmation message will be shown when the command has been executed and the program has excited 
command mode, all commands except `Ctrl b` (change baud-rate) which requires further input, will be 
executed and exit immediately. 

If the escape  key is configured as one of the command keys, then that command will not be available. 

All these commands must be proceeded with `Ctrl [escape key]`

- `Ctrl x`
    Exit the program: if the `--noreset` option was not given then the serial port is 
    reset to its original settings before exiting; if it was given the serial port is not reset. 

- `Ctrl q`
    Quit the program *without* resetting the serial port, regardless of the `--noreset` option. 

- `Ctrl p`
    Pulse the DTR line. Lower it for 1 sec, and then raise it again. 

- `Ctrl t`
    Toggle the DTR line. If DTR is up, then lower it. If it is down, then raise it. 

- `Ctrl g`
    Toggles the RTS line. If RTS is up, then lower it. If it is down, then raise it. 

- `Ctrl u`
    Baud up. Increase the baud-rate to the next highest standard baud-rate. 

- `Ctrl d`
    Baud down. Decrease the baud-rate to the next lowest baud-rate. 

- `Ctrl f`
    Cycle through flow-control settings (`RTS/CTS`, `XON/XOFF`, `none`). 

- `Ctrl y`
    Cycle through parity settings (even, odd, none). 

- `Ctrl i`
    Cycle through databits-number settings (`5`, `6`, `7`, `8`). 

- `Ctrl j`
    Changes the stop bits between 1 and 2. 

- `Ctrl c`
    Toggles local echo on and off. If on the application will print any and all characters typed 
    to the terminal regardless of weather the serial device echoes them back. 

- `Ctrl v`
    Show program options (like baud rate, data bits, etc). Only the options that can be modified 
    online (through commands) are shown, not those that can only be set at the command-line. 

- `Ctrl h`
    Shows a help message of all these commands, with shortened explanations of each command. 

## Development 

I have never written a terminal emulator before, and this has all been a big yak shave originating from 
being stuck with a Windows PC and wishing I could use picocom. I have been reliant upon Wikipedia, 
throwing bytes at other terminal emulators to see how they behave, and reading some really godawful C code. 

As a result there may be numerous bugs, and numerous gaps in my own understanding, but as of writing 
it seems to behave mostly like picocom under a Unix like environment and works pretty much the same in Windows. 

With that said, any bugs or general points raised would be very welcome, feel free to raise a bug, 
open a pull request, message me or however you wish. 

Current roadmap is:

* `--noreset / -r` flag
    * This flag currently does nothing, when closing down nanocom currently just closes the port
    and I'm not sure what the reset should do, it may very well be that this flag also disappears 
    later. 

* File send and receive
    * Picocom does this via invoking other standard unix applications, should we 
    follow this lead (it would be fairly simple to reimplement them cross platform) or bundle the 
    functionality within this application, this will make the interface slightly more different. 

* More rich escape sequence handling 
    * We currently handle a few different cursor controls and tabs, a cursory glance reveals 
    no ANSI escape sequence parsers that would really work well with this application, a custom 
    one shouldn't be too difficult. 

