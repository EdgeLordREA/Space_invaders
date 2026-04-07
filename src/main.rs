pub mod actions;
pub mod gamestate;
mod objects;

use macroquad::*;
use macroquad::color::*;
use macroquad::input::{is_key_down, KeyCode};
use macroquad::prelude::draw_rectangle;
use macroquad::window::{clear_background, next_frame};

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_rectangle(0.0, 0.0, 200.0, 200.0, RED);
        draw_rectangle(10.0, 10.0, 200.0, 200.0, BLUE);
        next_frame().await
    }
}

fn handle_inputs()
{
    if is_key_down(KeyCode::W) {actions::do_w()}
    if is_key_down(KeyCode::A) {actions::do_a()}
    if is_key_down(KeyCode::S) {actions::do_s()}
    if is_key_down(KeyCode::D) {actions::do_d()}
}

