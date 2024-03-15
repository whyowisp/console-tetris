#![allow(dead_code)]

mod game_tick;
mod user_input;

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::ErrorKind;
use std::io;
use std::io::Error;
use std::thread;

//a change

fn main() -> io::Result<()> {
    loop {
        match user_input::read_character() {
            Ok(c) => {
                println!("you typed: {}", c)
            }

            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
