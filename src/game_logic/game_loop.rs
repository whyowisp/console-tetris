use crate::game_logic::{game_tick, user_input};
use std::thread;

pub fn start() {
    let input_loop = thread::spawn(|| {
        let _result = user_input::listen();
    });
    let game_tick = thread::spawn(|| {
        let _result = game_tick::tick();
    });

    input_loop.join().expect("Input loop panicked");
    game_loop.join().expect("game_loop panicked");
}
