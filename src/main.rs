use std::thread;

mod game_frame;
mod user_input;

fn main() {
    let input_loop = thread::spawn(|| {
        let _result = user_input::listen();
    });
    let game_loop = thread::spawn(|| {
        let _result = game_frame::draw();
    });

    input_loop.join().expect("Input loop panicked");
    game_loop.join().expect("game_loop panicked");
}
