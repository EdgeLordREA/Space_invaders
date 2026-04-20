pub mod gamestate;
mod objects;
pub mod constants;
pub mod managers;
pub mod functionals;

use crate::gamestate::GameState;
use macroquad::color::*;
use macroquad::input::{is_key_down, KeyCode};
use macroquad::prelude::draw_rectangle;
use macroquad::shapes::draw_circle;
use macroquad::text::draw_text;
use macroquad::time::get_frame_time;
use macroquad::window::{clear_background, next_frame, screen_height, screen_width};
use functionals::actions;

/// Main entry point for the game application.
/// 
/// Initializes the game state and runs the main game loop, which handles:
/// - Processing user input
/// - Updating game state
/// - Rendering graphics
#[macroquad::main("BasicShapes")]
async fn main() {
    let gamestate = &mut GameState::new(screen_width(), screen_height());
    gamestate.load_enemies();
    loop {
        let delta = get_frame_time();
        handle_inputs(gamestate, delta);
        gamestate.run_state(delta);
        render_gamestate(gamestate);
        next_frame().await
    }
}

/// Handles player input and updates the game state accordingly.
/// 
/// # Arguments
/// * `state` - Mutable reference to the current GameState
/// * `delta` - Time elapsed since the last frame (used for delta-time movement)
/// 
/// Maps WASD keys to player movement directions and Space to shooting.
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

/// Renders the current game state to the screen.
/// 
/// # Arguments
/// * `gamestate` - Reference to the current GameState to render
/// 
/// Draws the player as a blue rectangle, bullets as red circles,
/// and displays the bullet count in the top-left corner.
fn render_gamestate(gamestate: &GameState) {

    clear_background(BLACK);
    let player_rect = gamestate.player.shape;
    draw_rectangle(player_rect.x, player_rect.y, player_rect.w, player_rect.h, BLUE);
    draw_text(
        &format!("{}", gamestate.bullets.len()),
        10.0,
        10.0,
        20.0,
        RED,
    );

    for x in &gamestate.bullets {
        let pos = x.get_position();
        draw_circle(pos.x, pos.y, x.radius, RED);

    }


    for x in &gamestate.enemies {
        let pos = x.shape.top_left();
        draw_rectangle(pos.x, pos.y, x.shape.w, x.shape.h, RED);
    }
}
