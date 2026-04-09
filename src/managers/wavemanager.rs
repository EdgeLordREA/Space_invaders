use crate::objects::wave;
struct WaveManager {
    waves : Vec<wave::Wave>,
    current_wave: i32,
    max_enemies: i32,
    current_enemies: i32,
    spawn_delay : f32,
    pub wave_state : WaveState
}
pub enum WaveState {
    Spawning,
    InProgress,
    Transitioning,
}
impl WaveManager {}