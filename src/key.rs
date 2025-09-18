use console::Key;

use crate::utils;


/// Data from reading the serial port
#[derive(Debug, Clone)]
pub enum KeyIn {
    Char(u8),
    Nothing,
    Escape(EscapeSequence)
}



impl KeyIn {
    /// Gives the correct [SerialData] from [Key].
    #[cfg(target_os = "windows")]
    pub fn from_console_key(c: &Key) -> Self {
        match c {
            Key::Char(c) => KeyIn::Char(utils::get_ascii_byte(*c)),
            Key::CtrlC => KeyIn::Char(3),
            Key::Tab => KeyIn::Char(9),
            Key::Enter => KeyIn::Char(13),
            Key::Escape => KeyIn::Char(27),
            Key::Backspace => KeyIn::Char(127),
            s => KeyIn::Escape(EscapeSequence::from_console_key(s.clone()))
        }
    }
    /// Gives the correct [SerialData] from [Key].
    #[cfg(not(target_os = "windows"))]
    pub fn from_console_key(c: &Key) -> Self {
        match c {
            Key::Char(c) => KeyIn::Char(utils::get_ascii_byte(*c)),
            Key::Home => KeyIn::Char(1),
            Key::CtrlC => KeyIn::Char(3),
            Key::End => KeyIn::Char(5),
            Key::Tab => KeyIn::Char(9),
            Key::Enter => KeyIn::Char(13),
            Key::Escape => KeyIn::Char(27),
            Key::Backspace => KeyIn::Char(127),
            s => KeyIn::Escape(EscapeSequence::from_console_key(s.clone()))
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::Char(c) => vec![*c],
            Self::Nothing => vec![],
            Self::Escape(e) => e.to_bytes(),
        }
    }
}

/// Possible escape sequences returned from the parser. 
#[derive(Debug, Clone)]
pub enum EscapeSequence {
    Invalid,
    UnknownSeq(Vec<u8>),
    ArrowLeft, 
    ArrowRight,
    ArrowUp,
    ArrowDown,
    BackTab,
    Tab,
    Alt,
    Home,
    End,
    Del,
    Insert,
    PageUp,
    PageDown,
}

impl EscapeSequence {

    /// Gives the correct [EscapeSequence] from [Key].
    pub fn from_console_key(c: Key) -> Self {
        match c {
            Key::UnknownEscSeq(s) => 
                EscapeSequence::UnknownSeq(s.iter().map(|e| utils::get_ascii_byte(*e)).collect()),
            Key::ArrowLeft => EscapeSequence::ArrowLeft,
            Key::ArrowRight => EscapeSequence::ArrowRight,
            Key::ArrowUp => EscapeSequence::ArrowUp,
            Key::ArrowDown => EscapeSequence::ArrowDown,
            Key::BackTab => EscapeSequence::BackTab,
            Key::Alt => EscapeSequence::Alt,
            Key::Home => EscapeSequence::Home,
            Key::End => EscapeSequence::End,
            Key::Del => EscapeSequence::Del,
            Key::Insert => EscapeSequence::Insert,
            Key::PageUp => EscapeSequence::PageUp,
            Key::PageDown => EscapeSequence::PageDown,
            _ => EscapeSequence::Invalid,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::BackTab => vec![27, 91, 90],
            Self::Alt => vec![27, 91, 90],
            Self::ArrowLeft => vec![27, 91, 68],
            Self::ArrowRight => vec![27, 91, 67],
            Self::ArrowUp => vec![27, 91, 65],
            Self::ArrowDown => vec![27, 91, 66],
            Self::Home => vec![27, 91, 49, 126],
            Self::End => vec![27, 91, 52, 126],
            Self::Del => vec![27, 91, 51, 126],
            Self::Insert => vec![27, 91, 50, 126],
            Self::PageUp => vec![27, 91, 53, 126],
            Self::PageDown => vec![27, 91, 54, 126],
            _ => vec![]
        }
    }
}
