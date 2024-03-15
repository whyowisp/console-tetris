use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};
use std::io;
//use std::io::Error;

pub fn read_character() -> io::Result<char> {
    if let event::Event::Key(KeyEvent {
        code, modifiers, ..
    }) = event::read()?
    {
        // Check if it's a regular key (no modifiers)
        if modifiers == KeyModifiers::NONE {
            match code {
                KeyCode::Char(c) => return Ok(c),
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "No character entered",
                    ))
                }
            }
        }
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        "No character entered",
    ))
}

/*
pub fn read_character() -> io::Result<char> {
    // Read a single character from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Extract the first character (if any)
    if let Some(c) = input.chars().next() {
        Ok(c)
    } else {
        Err(Error::new(
            io::ErrorKind::InvalidInput,
            "No character found",
        ))
    }
}*/
