use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::io::{self, Result};

pub fn listen() -> io::Result<char> {
    loop {
        if let Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        }) = event::read()?
        {
            return Ok(c);
        }
    }
}
