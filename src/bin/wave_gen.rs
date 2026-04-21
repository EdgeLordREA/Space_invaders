use Space_invaders::objects::wave::Wave;
use Space_invaders::objects::classic_enemy::{ClassicEnemy, Direction};
use Space_invaders::functionals::vec2::Vec2;
use Space_invaders::functionals::waveloader::save_waves;

fn main() {
    let mut all_waves = Vec::new();

    // Configuration for 10 Unique Waves
    let wave_configs = vec![
        // (count, health, speed, rate, label)
        (5,  50.0,  1.2, 1.0, "Wave 1: The Scouts"),
        (8,  40.0,  1.5, 1.5, "Wave 2: The Vanguard"),
        (12, 30.0,  2.0, 2.5, "Wave 3: The Swarm"),
        (4,  150.0, 0.8, 0.5, "Wave 4: Heavy Armor"),
        (10, 60.0,  1.4, 1.2, "Wave 5: Reinforcements"),
        (15, 25.0,  2.5, 3.0, "Wave 6: Sonic Blitz"),
        (6,  100.0, 1.1, 0.8, "Wave 7: The Guardians"),
        (20, 20.0,  2.2, 4.0, "Wave 8: Locust Storm"),
        (3,  300.0, 0.5, 0.3, "Wave 9: The Goliaths"),
        (25, 45.0,  1.8, 2.0, "Wave 10: The Final Push"),
    ];

    for (idx, (count, hp, speed, rate, _label)) in wave_configs.into_iter().enumerate() {
        let mut wave = Wave::new();
        wave.set_max_enemies(count);
        wave.set_spawn_rate(rate);

        for i in 0..count {
            // Alternating starting directions for even/odd waves
            let start_dir = if idx % 2 == 0 { Direction::Right } else { Direction::Left };

            // Staggering the vertical starting height slightly for variety
            let start_y = 50.0 + (idx as f32 * 10.0);

            let enemy = ClassicEnemy::new(
                hp,
                speed,
                Vec2::new(100.0, start_y),
                Vec2::new(40.0, 20.0),
                start_dir,
                (hp * 0.5) as i32, // Cash value scaled to health
                i
            );
            wave.add_enemies(enemy);
        }
        all_waves.push(wave);
    }

    // Save using your existing waveloader logic
    save_waves(&all_waves, "waves.json");
    println!("Generated 10 unique waves in waves.json successfully!");
}