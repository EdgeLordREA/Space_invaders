//! Wave module.
//!
//! Contains the Wave struct for managing groups of enemies
//! that appear together during gameplay.

use serde::Deserialize;
use crate::objects::classic_enemy::ClassicEnemy;

/// A wave of enemies to be spawned and managed together.
/// 
/// Contains a collection of enemies and wave configuration parameters.
#[derive(Debug, Deserialize)]
pub struct Wave{
    /// Collection of enemies in this wave
    enemies : Vec<ClassicEnemy>,
    /// Maximum number of enemies allowed in this wave
    max_enemies : i32,
    /// Maximum duration for this wave in seconds
    max_duration : f32,
}