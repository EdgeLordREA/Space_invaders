pub mod actions;
pub mod gamestate;
mod objects;
mod constants;

use macroquad::color::*;
use macroquad::input::{is_key_down, KeyCode};
use macroquad::prelude::draw_rectangle;
use macroquad::shapes::draw_circle;
use macroquad::time::get_frame_time;
use macroquad::window::{clear_background, next_frame, screen_height, screen_width};
use crate::gamestate::GameState;
use crate::objects::player::Player;
#[macroquad::main("BasicShapes")]
async fn main() {

    
    let gamestate = &mut GameState::new(screen_width(), screen_height());
    
    loop {
        let delta = get_frame_time();
        handle_inputs(&mut gamestate.player, delta);
        gamestate.run_state(delta);
        render_gamestate(gamestate);
        next_frame().await
    }
}

fn handle_inputs(player: &mut Player, delta : f32)
{
    if is_key_down(KeyCode::W) {actions::do_w(player, delta)}
    if is_key_down(KeyCode::A) {actions::do_a(player, delta)}
    if is_key_down(KeyCode::S) {actions::do_s(player, delta)}
    if is_key_down(KeyCode::D) {actions::do_d(player, delta)}
}

fn render_gamestate(gamestate: &GameState) {
    let playerpos = gamestate.player.position();


    clear_background(BLACK);
    draw_rectangle(playerpos.x, playerpos.y, 30.0, 20.0, BLUE);

    for x in  &gamestate.bullets {
        let pos = x.get_position();
        draw_circle(pos.x, pos.y, 4.0, RED);
    }
}
