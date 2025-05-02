use std::fs::File;
use std::io::{self, Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Result {
    node_count: String,
    mis_count: String,
    running_time: String,
}

pub fn save_result_to_json(filename: &str, node_count: String, mis_count: String, running_time: String) -> io::Result<String> {
    let result = Result {
        node_count,
        mis_count,
        running_time,
    };

    let serialized = serde_json::to_string_pretty(&result).unwrap();

    let result_filename = format!("{filename}.json");

    let mut file = File::create(&result_filename)?;

    file.write_all(serialized.as_bytes())?;

    Ok(result_filename)
}