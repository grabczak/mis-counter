use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufRead, BufWriter, Write};
use std::path::{Path, PathBuf};
use chrono::Local;

pub fn read_tree_from_csv(path: PathBuf) -> io::Result<Vec<Vec<usize>>> {
    let file = File::open(path)?;
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

pub fn save_tree_to_csv(nodes: HashMap<usize, Vec<usize>>) -> io::Result<PathBuf> {
    fs::create_dir_all("gen")?;

    let path = Path::new("gen").join(Local::now().format("%Y-%m-%d-%H-%M-%S-%3f.csv").to_string());

    let file = File::create(&path)?;
    let mut writer = BufWriter::new(file);

    let mut items: Vec<_> = nodes.into_iter().collect();
    items.sort_by_key(|(k, _)| *k);

    for (parent, children) in items {
        let line = format!("{} {}", parent, children.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" "));
        writeln!(writer, "{}", line)?;
    }

    Ok(path)
}

pub fn save_result_to_file(path: PathBuf, result: String) -> io::Result<PathBuf> {
    let result_path = insert_result_suffix(path);

    let mut file = File::create(&result_path)?;

    writeln!(file, "{}", result)?;

    Ok(result_path)
}

fn insert_result_suffix(path: PathBuf) -> PathBuf {
    let parent = path.parent().unwrap_or_else(|| Path::new(""));

    let stem = path.file_stem().unwrap_or_default().to_string_lossy();

    let new_path = format!("{stem}.result");

    parent.join(new_path)
}

