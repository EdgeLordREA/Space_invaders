pub mod actions;
pub mod gamestate;
mod objects;
pub mod math;
pub mod config;
pub mod constants;

use crate::gamestate::GameState;
use macroquad::color::*;
use macroquad::input::{KeyCode, is_key_down};
use macroquad::prelude::draw_rectangle;
use macroquad::shapes::draw_circle;
use macroquad::text::draw_text;
use macroquad::time::get_frame_time;
use macroquad::window::{clear_background, next_frame, screen_height, screen_width};
#[macroquad::main("BasicShapes")]
async fn main() {
    let gamestate = &mut GameState::new(screen_width(), screen_height());

    loop {
        let delta = get_frame_time();
        handle_inputs(gamestate, delta);
        gamestate.run_state(delta);
        render_gamestate(gamestate);
        next_frame().await
    }
}

fn handle_inputs(state: &mut GameState, delta: f32) {
    if is_key_down(KeyCode::W) {
        actions::do_w(&mut state.player, delta)
    }
    if is_key_down(KeyCode::A) {
        actions::do_a(&mut state.player, delta)
    }
    if is_key_down(KeyCode::S) {
        actions::do_s(&mut state.player, delta)
    }
    if is_key_down(KeyCode::D) {
        actions::do_d(&mut state.player, delta)
    }
    if is_key_down(KeyCode::Space) {
        actions::do_space(state)
    }
}

fn render_gamestate(gamestate: &GameState) {
    let player_pos = gamestate.player.position();

    clear_background(BLACK);
    draw_rectangle(player_pos.x, player_pos.y, gamestate.player.dimensions.x, gamestate.player.dimensions.y, BLUE);
    draw_text(
        &format!("{}", gamestate.bullets.len()),
        10.0,
        10.0,
        20.0,
        RED,
    );

    for x in &gamestate.bullets {
        let pos = x.get_position();
        draw_circle(pos.x, pos.y, x.dimensions.x, RED);

    }
}
