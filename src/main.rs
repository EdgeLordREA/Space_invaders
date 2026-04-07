use macroquad::input::is_key_down;
use macroquad::miniquad::KeyCode;

fn main() {
    println!("Hello, world!");
    let r = is_key_down(KeyCode::A);
}

fn handle_inputs()
{
    if is_key_down(KeyCode::A) {  }
}