use std::collections::VecDeque;
use std::fs::{read_to_string, File};
use std::io::Write;
use crate::objects::wave::Wave;

pub fn load_waves(filename: &str) -> VecDeque<Wave> {
    let data = read_to_string(filename)
        .expect("Unable to read wave file");

    // Deserialize the JSON string directly into a Vec<Wave>
    let waves: Vec<Wave> = serde_json::from_str(&data)
        .expect("JSON was not well-formatted");

    waves.into()
}

/// Saves waves to a JSON file with pretty-printing.
pub fn save_waves(waves: &Vec<Wave>, filename: &str) {
    let mut file = File::create(filename)
        .expect("Unable to create wave file");

    // "to_string_pretty" makes the file human-readable
    let json = serde_json::to_string_pretty(waves)
        .expect("Failed to serialize waves");

    file.write_all(json.as_bytes())
        .expect("Failed to write to file");
}