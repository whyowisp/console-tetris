#![allow(dead_code)]

mod game_tick;
mod user_input;

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::ErrorKind;
use std::io;
use std::io::Error;
use std::thread;

//a change
fn read_character() -> io::Result<char> {
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
}

fn main() -> io::Result<()> {
    loop {
        match read_character() {
            Ok(c) => println!("you typed: {}", c),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
