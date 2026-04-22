use Space_invaders;

use macroquad::color::*;
use macroquad::input::{is_key_down, KeyCode};
use macroquad::prelude::draw_rectangle;
use macroquad::rand::ChooseRandom;
use macroquad::shapes::draw_circle;
use macroquad::text::draw_text;
use macroquad::time::get_frame_time;
use macroquad::window::{clear_background, next_frame, screen_height, screen_width};
use rand;
use Space_invaders::functionals::actions;
use Space_invaders::functionals::waveloader::load_waves;
use Space_invaders::gamestate::GameState;
use Space_invaders::objects::player::PlayerStat;
use Space_invaders::objects::upgrade::Upgrade;
use Space_invaders::UI::button::Button;

/// Main entry point for the game application.
/// 
/// Initializes the game state and runs the main game loop, which handles:
/// - Processing user input
/// - Updating game state
/// - Rendering graphics
#[macroquad::main("BasicShapes")]
async fn main() {
    let gamestate = &mut GameState::new(screen_width(), screen_height());
    let mut all_waves = load_waves("waves.json");
    let mut all_upgrades = load_upgrades();
    let mut available_upgrades: Vec<Upgrade> = Vec::new();
    gamestate.wave = all_waves.pop_front().unwrap();
    loop {
        if !gamestate.wave_complete() {
            let delta = get_frame_time();
            handle_inputs(gamestate, delta);
            gamestate.run_state(delta);
            render_gamestate(gamestate);
        }
        else {
            if available_upgrades.is_empty() {
                let mut all_upgrades_copy = all_upgrades.clone();

                // Fisher-Yates shuffle using macroquad's random
                for i in (1..all_upgrades_copy.len()).rev() {
                    let j = macroquad::rand::gen_range(0, (i + 1) as i32) as usize;
                    all_upgrades_copy.swap(i, j);
                }

                available_upgrades = all_upgrades_copy.into_iter().take(3).collect();
            }
            if !take_break(gamestate, &available_upgrades)
            {

            }
            else {
                gamestate.wave = all_waves.pop_front().unwrap();
            }
        }
        next_frame().await
    }
}

fn load_upgrades() -> Vec<Upgrade> {
    let mut upgrades = Vec::new();

    upgrades.push(Upgrade {
        button: Button::new(0.0, 0.0, 200.0, 100.0, "Health", GREEN),
        player_stat: PlayerStat::Health,
        cost: 50,
        upgrade: 10.0,
    });

    upgrades.push(Upgrade {
        button: Button::new(0.0, 0.0, 200.0, 100.0, "Attack Speed", GREEN),
        player_stat: PlayerStat::AttackSpeed,
        cost: 75,
        upgrade: 0.1,
    });

    upgrades.push(Upgrade {
        button: Button::new(0.0, 0.0, 200.0, 100.0, "Bullet Damage", GREEN),
        player_stat: PlayerStat::BulletDamage,
        cost: 100,
        upgrade: 5.0,
    });

    upgrades.push(Upgrade {
        button: Button::new(0.0, 0.0, 200.0, 100.0, "Bullet Speed", GREEN),
        player_stat: PlayerStat::BulletSpeed,
        cost: 60,
        upgrade: 2.0,
    });

    upgrades.push(Upgrade {
        button: Button::new(0.0, 0.0, 200.0, 100.0, "Bullet Radius", GREEN),
        player_stat: PlayerStat::BulletRadius,
        cost: 80,
        upgrade: 0.5,
    });
    upgrades.push(Upgrade {
        button: Button::new(0.0, 0.0, 200.0, 100.0, "Bullet Count", GREEN),
        player_stat: PlayerStat::BulletCount,
        cost: 300,
        upgrade: 1.0,
    });

    upgrades.push(Upgrade {
        button: Button::new(0.0, 0.0, 200.0, 100.0, "Bullet Penetration", GREEN),
        player_stat: PlayerStat::BulletPen,
        cost: 200,
        upgrade: 1.0,
    });
    upgrades
}

fn take_break(gs : &mut GameState, upgrades : &Vec<Upgrade>) -> bool{

    render_shop(gs, upgrades)
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

    draw_text(&format!("{}", gamestate.player.cash), 10.0, 30.0, 20.0, RED
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

fn render_shop(gs : &mut GameState, Upgrades : &Vec<Upgrade>) -> bool
{
    clear_background(BLACK);
    draw_text("Shop Time!", 100.0, 100.0, 50.0, RED );
    false
}