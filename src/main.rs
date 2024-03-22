#![allow(dead_code)]
mod user_input;

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::ErrorKind;
use std::io;
use std::io::Error;

use std::io::Write;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread; // Multi-producer, single-consumer FIFO queue communication primitives

fn main() -> std::io::Result<()> {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        loop {
            match rx.try_recv() {
                Ok(character) => {
                    println!("Received: {}", character);
                    // Insert your game code here. You can use the `character` variable
                    // as the user input.
                }
                Err(e) => {
                    // No input received, continue with the game loop
                }
            }
        }
    });

    loop {
        let result = user_input::read_character();
        match result {
            Ok(character) => {
                tx.send(character).unwrap();
            }
            Err(e) => {
                eprintln!("Error reading character: {}", e);
            }
        }
    }

    Ok(())
}
