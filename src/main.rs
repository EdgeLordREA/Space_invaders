pub mod actions;
pub mod gamestate;
mod objects;
mod constants;

use macroquad::color::*;
use macroquad::input::{is_key_down, KeyCode};
use macroquad::prelude::draw_rectangle;
use macroquad::time::get_frame_time;
use macroquad::window::{clear_background, next_frame, screen_height, screen_width};
use crate::gamestate::GameState;
use crate::objects::player::Player;
#[macroquad::main("BasicShapes")]
async fn main() {

    
    let gamestate = &mut GameState::new(screen_width(), screen_height());
    
    loop {
        handle_inputs(&mut gamestate.player);
        gamestate.run_state(get_frame_time());
        clear_background(BLACK);

        draw_rectangle(0.0, 0.0, 200.0, 200.0, RED);
        draw_rectangle(10.0, 10.0, 200.0, 200.0, BLUE);
        next_frame().await
    }
}

fn handle_inputs(player: &mut Player)
{
    if is_key_down(KeyCode::W) {actions::do_w(player)}
    if is_key_down(KeyCode::A) {actions::do_a(player)}
    if is_key_down(KeyCode::S) {actions::do_s(player)}
    if is_key_down(KeyCode::D) {actions::do_d(player)}
}

