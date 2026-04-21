use Space_invaders::objects::wave::Wave;
use Space_invaders::objects::classic_enemy::{ClassicEnemy, Direction};
use Space_invaders::functionals::vec2::Vec2;
use Space_invaders::functionals::waveloader::save_waves;

fn main() {
    let mut all_waves = Vec::new();

    // Create Wave 1
    let mut wave1 = Wave::new();
    wave1.set_max_enemies(5);
    wave1.set_spawn_rate(1.0);

    for i in 0..5 {
        let enemy = ClassicEnemy::new(
            50.0, 1.2,
            Vec2::new(100.0, 100.0),
            Vec2::new(40.0, 20.0),
            Direction::Right, 10, i
        );
        wave1.add_enemies(enemy);
    }

    all_waves.push(wave1);

    // Save using your existing waveloader logic
    save_waves(&all_waves, "waves.json");
    println!("Generated waves.json successfully!");
}