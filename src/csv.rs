use std::fs::{self, File};
use std::io::{self, BufRead, BufWriter, Write};
use std::collections::HashMap;
use chrono::Local;

pub fn read_tree_from_csv(filename: &str) -> io::Result<Vec<Vec<usize>>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut data: Vec<Vec<usize>> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let numbers: Vec<usize> = line
            .split(' ')
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

    for (parent, children) in nodes {
        let line = format!("{} {}", parent, children.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" "));
        writeln!(writer, "{}", line.trim())?;
    }

    Ok(filename)
}
