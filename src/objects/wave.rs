//! Wave module.
//!
//! Contains the Wave struct for managing groups of enemies
//! that appear together during gameplay.

use std::collections::VecDeque;
use serde::{Deserialize, Serialize};
use crate::objects::classic_enemy::ClassicEnemy;

/// A wave of enemies to be spawned and managed together.
/// 
/// Contains a collection of enemies and wave configuration parameters.
#[derive(Debug, Deserialize, Serialize)]
pub struct Wave{
    /// Collection of enemies in this wave
    pub enemies : VecDeque<ClassicEnemy>,
    /// Maximum number of enemies allowed in this wave
    max_enemies : i32,
    ///enemies per second
    spawn_rate : f32,
    last_spawn_time : f32,
    /// Maximum duration for this wave in seconds
    max_duration : f32,
}

impl Wave {
    pub fn new() -> Wave {
        Wave{
            enemies : VecDeque::new(),
            max_enemies : 0,
            spawn_rate : 0.0,
            last_spawn_time : 0.0,
            max_duration : 0.0
        }
    }

    pub fn deserialize(enemies : Vec<ClassicEnemy>, max_enemies : i32, spawn_rate : f32, max_duration : f32) -> Wave
    {
        let mut wv = Self::new();
        wv.set_enemies(enemies);
        wv.set_max_enemies(max_enemies);
        wv.set_spawn_rate(spawn_rate);
        wv.set_max_duration(max_duration);
        wv.last_spawn_time = 0.0;
        wv
    }



    pub fn run_wave(&mut self, delta : f32, enemy_count : i32) -> Option<ClassicEnemy>
    {
        if self.enemies.is_empty() {
            return None;
        }
        self.last_spawn_time += delta;
        let seconds_per_enemy = 1.0 / self.spawn_rate;
        if self.last_spawn_time > seconds_per_enemy && enemy_count < self.max_enemies{
            self.last_spawn_time -= seconds_per_enemy;
            self.enemies.pop_front()
        }
        else{
            None
        }
    }

    pub fn set_enemies(&mut self, enemies : Vec<ClassicEnemy>)
    {
        self.enemies = enemies.into();
    }

    pub fn add_enemies(&mut self, enemy : ClassicEnemy)
    {
        self.enemies.push_back(enemy)
    }
    pub fn set_max_enemies(&mut self, max_enemies : i32)
    {
        self.max_enemies = max_enemies;
    }

    pub fn set_spawn_rate(&mut self, spawn_rate : f32)
    {
        self.spawn_rate = spawn_rate;
    }

    pub fn set_max_duration(&mut self, max_duration : f32)
    {
        self.max_duration = max_duration;
    }

    pub fn is_complete(&self) -> bool
    {
        self.enemies.is_empty()
    }
}