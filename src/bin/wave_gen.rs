use Space_invaders::objects::wave::Wave;
use Space_invaders::objects::classic_enemy::{ClassicEnemy, Direction};
use Space_invaders::functionals::vec2::Vec2;
use Space_invaders::functionals::waveloader::save_waves;

fn main() {
    let mut all_waves = Vec::new();

    // Configuration: (count, health, speed, rate, line_height, label)
    let wave_configs = vec![
        (5,  50.0,  1.2, 1.0, 30.0, "Wave 1: The Scouts"),
        (8,  40.0,  1.5, 1.5, 30.0, "Wave 2: The Vanguard"),
        (12, 30.0,  2.0, 2.5, 25.0, "Wave 3: The Swarm"),
        (4,  150.0, 0.8, 0.5, 40.0, "Wave 4: Heavy Armor"),
        (10, 60.0,  1.4, 1.2, 30.0, "Wave 5: Reinforcements"),
        (15, 25.0,  2.5, 3.0, 20.0, "Wave 6: Sonic Blitz"),
        (6,  100.0, 1.1, 0.8, 35.0, "Wave 7: The Guardians"),
        (20, 20.0,  2.2, 4.0, 25.0, "Wave 8: Locust Storm"),
        (3,  300.0, 0.5, 0.3, 50.0, "Wave 9: The Goliaths"),
        (25, 45.0,  1.8, 2.0, 30.0, "Wave 10: The Final Push"),
    ];

    for (idx, (count, hp, speed, rate, line_h, _label)) in wave_configs.into_iter().enumerate() {
        let mut wave = Wave::new();
        wave.set_max_enemies(count);
        wave.set_spawn_rate(rate);
        wave.set_wave_line_height(line_h); // Now setting the specific height for this wave

        for i in 0..count {
            let start_dir = if idx % 2 == 0 { Direction::Right } else { Direction::Left };
            let start_y = 50.0 + (idx as f32 * 10.0);

            let enemy = ClassicEnemy::new(
                hp,
                speed,
                Vec2::new(100.0, start_y),
                Vec2::new(40.0, 20.0),
                start_dir,
                (hp * 0.5) as i32,
                i
            );
            wave.add_enemies(enemy);
        }
        all_waves.push(wave);
    }

    save_waves(&all_waves, "waves.json");
    println!("Generated 10 unique waves with custom line heights!");
}