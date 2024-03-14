use crossterm::event::{read, Event};
use std::io::Result;

pub fn listen() -> Result<bool> {
    loop {
        match read()? {
            Event::FocusGained => println!("FocusGained"),
            Event::FocusLost => println!("FocusLost"),
            Event::Key(event) => println!("{:?}", event),
            Event::Mouse(event) => println!("{:?}", event),
            Event::Paste(data) => println!("{:?}", data), // Add this arm
            Event::Resize(width, height) => println!("New size {}x{}", width, height),
        }
    }
}
