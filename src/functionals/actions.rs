/// Player movement and action handlers.
/// 
/// This module provides functions to handle keyboard input
/// and translate it into player actions.

use crate::gamestate::GameState;
use crate::objects::player::Player;

/// Moves the player upward (W key).
/// 
/// # Arguments
/// * `player` - Mutable reference to the player to move
/// * `delta` - Time elapsed since last frame for delta-time movement
pub fn do_w(player: &mut Player, delta: f32) {
    player.position.y -= player.speed * delta;
}

/// Moves the player leftward (A key).
/// 
/// # Arguments
/// * `player` - Mutable reference to the player to move
/// * `delta` - Time elapsed since last frame for delta-time movement
pub fn do_a(player: &mut Player, delta: f32) {
    player.position.x -= player.speed * delta;
}

/// Moves the player downward (S key).
/// 
/// # Arguments
/// * `player` - Mutable reference to the player to move
/// * `delta` - Time elapsed since last frame for delta-time movement
pub fn do_s(player: &mut Player, delta: f32) {
    player.position.y += player.speed * delta;
}

/// Moves the player rightward (D key).
/// 
/// # Arguments
/// * `player` - Mutable reference to the player to move
/// * `delta` - Time elapsed since last frame for delta-time movement
pub fn do_d(player: &mut Player, delta: f32) {
    player.position.x += player.speed * delta;
}

/// Handles the player's attack action (Space key).
/// 
/// # Arguments
/// * `game_state` - Mutable reference to the current GameState
/// 
/// Creates a new bullet if the player's attack cooldown has expired.
pub fn do_space(game_state : &mut GameState) {
    if game_state.player.can_attack() {
        game_state.player.attack(&mut game_state.bullets)
    }
}
