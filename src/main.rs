mod game_frame;
mod user_input;

fn main() {
    let _result = game_frame::draw();
    user_input::listen();
}
