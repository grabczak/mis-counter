use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufRead, BufWriter, Write};
use std::path::Path;
use chrono::Local;

pub fn read_tree_from_csv(filename: &str) -> io::Result<Vec<Vec<usize>>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut data: Vec<Vec<usize>> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let numbers: Vec<usize> = line
            .split(|c| c == ' ' || c == ',' || c == '\t')
            .filter_map(|s| s.trim().parse::<usize>().ok())
            .collect();
        data.push(numbers);
    }

    Ok(data)
}

pub fn save_tree_to_csv(nodes: HashMap<usize, Vec<usize>>) -> io::Result<String> {
    fs::create_dir_all("./gen/")?;

    let filename = format!("./gen/{}.csv", Local::now().format("%Y-%m-%d-%H-%M-%S-%3f"));
    let file = File::create(&filename)?;
    let mut writer = BufWriter::new(file);

    let mut items: Vec<_> = nodes.into_iter().collect();
    items.sort_by_key(|(k, _)| *k);

    for (parent, children) in items {
        let line = format!("{} {}", parent, children.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" "));
        writeln!(writer, "{}", line)?;
    }

    Ok(filename)
}

pub fn save_result_to_file(filename: &str, value: String) -> io::Result<String> {
    let result_filename = insert_result_suffix(filename);

    let mut file = File::create(&result_filename)?;

    writeln!(file, "{}", value)?;

    Ok(result_filename)
}

fn insert_result_suffix(filename: &str) -> String {
    let path = Path::new(filename);
    let parent = path.parent().unwrap_or_else(|| Path::new(""));

    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    let extension = path.extension().unwrap_or_default().to_string_lossy();

    let new_filename = if extension.is_empty() {
        format!("{stem}.result")
    } else {
        format!("{stem}.result.{extension}")
    };

    parent.join(new_filename).to_string_lossy().to_string()
}

