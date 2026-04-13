/// Wave Manager module.
/// 
/// Contains the WaveManager struct for controlling enemy wave spawning
/// and managing wave progression throughout the game.

use crate::objects::wave;

/// Manages enemy wave spawning and progression.
/// 
/// Tracks current wave state, enemy counts, and spawn timing.
struct WaveManager {
    /// Collection of all wave configurations
    waves : Vec<wave::Wave>,
    /// Index of the currently active wave
    current_wave: i32,
    /// Maximum number of enemies allowed at once
    max_enemies: i32,
    /// Current number of active enemies
    current_enemies: i32,
    /// Delay between enemy spawns in seconds
    spawn_delay : f32,
    /// Current state of wave progression
    pub wave_state : WaveState
}

/// Represents the current state of wave management.
pub enum WaveState {
    /// Currently spawning enemies for the wave
    Spawning,
    /// Wave is active with all enemies spawned
    InProgress,
    /// Transitioning between waves
    Transitioning,
}
impl WaveManager {}