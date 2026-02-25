use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

use crate::task::Task;

const FILE_PATH: &str = "tasks.json";

pub fn load_tasks() -> Vec<Task> {
    if !Path::new(FILE_PATH).exists() {
        return Vec::new();  // Return an empty list if the file doesn't exist yet
    }

    let mut file = File::open(FILE_PATH).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    if contents.is_empty() {
        return Vec::new();
    }

    serde_json::from_str(&contents).expect("Failed to parse JSON")
}

pub fn save_tasks(tasks: &[Task]) {
    // Convert the vector of tasks back into a JSON string
    let json = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks");
    
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE_PATH)
        .expect("Failed to open file");

    file.write_all(json.as_bytes()).expect("Failed to write to file");
}