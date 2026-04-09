use crate::gamestate::GameState;
use crate::objects::player::Player;

pub fn do_w(player: &mut Player, delta: f32) {
    player.position.y -= player.speed * delta;
}
pub fn do_a(player: &mut Player, delta: f32) {
    player.position.x -= player.speed * delta;
}

pub fn do_s(player: &mut Player, delta: f32) {
    player.position.y += player.speed * delta;
}

pub fn do_d(player: &mut Player, delta: f32) {
    player.position.x += player.speed * delta;
}

pub fn do_space(game_state : &mut GameState) {
    if game_state.player.can_attack() {
        game_state.player.attack(&mut game_state.bullets)
    }
}
