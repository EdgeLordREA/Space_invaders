pub mod actions;
pub mod player;
pub mod bullet;
pub mod gamestate;

use macroquad::*;
use macroquad::input::{is_key_down, KeyCode};

fn main() {
    println!("Hello, world!");
}

fn handle_inputs()
{
    if is_key_down(KeyCode::W) {actions::do_w()}
    if is_key_down(KeyCode::A) {actions::do_a()}
    if is_key_down(KeyCode::S) {actions::do_s()}
    if is_key_down(KeyCode::D) {actions::do_d()}
}

