use std::fs::{self, File};
use std::io::{self, BufRead, BufWriter, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

mod tree;
use tree::Tree;

fn read_tree_from_csv(filename: &str) -> io::Result<Vec<Vec<i32>>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut data: Vec<Vec<i32>> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let numbers: Vec<i32> = line
            .split(' ')
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect();
        data.push(numbers);
    }

    Ok(data)
}

fn save_tree_to_csv(tree: &Tree) -> std::io::Result<String> {
    fs::create_dir_all("./gen/")?;

    let filename = format!("./gen/{}.csv", Uuid::new_v4());
    let file = File::create(&filename)?;
    let mut writer = BufWriter::new(file);

    for (parent, children) in &tree.nodes {
        let line = format!("{} {}", parent, children.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" "));
        writeln!(writer, "{}", line.trim())?;
    }

    Ok(filename)
}

#[derive(Serialize, Deserialize, Debug)]
struct Result {
    node_count: String,
    mis_count: String,
    running_time: String,
}

fn save_result_to_json(filename: &str, node_count: usize, mis_count: usize, running_time: u128) -> io::Result<String> {
    let result = Result {
        node_count: node_count.to_string(),
        mis_count: mis_count.to_string(),
        running_time: running_time.to_string(),
    };

    let serialized = serde_json::to_string_pretty(&result).unwrap();

    let result_filename = format!("{filename}.json");

    let mut file = File::create(&result_filename)?;

    file.write_all(serialized.as_bytes())?;

    Ok(result_filename)
}

fn read_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

fn main() {
    loop {
        println!("Pick an option: ");
        println!("1. Read");
        println!("2. Generate");
        println!("3. Quit");

        let option = read_input();

        match option.as_str() {
            "1" => {
                println!("Enter the filename: ");

                let filename = read_input();

                match read_tree_from_csv(filename.as_str()) {
                    Ok(data) => {
                        let tree = Tree::new(0, data);

                        println!("Tree loaded from {}", filename);

                        tree.print();

                        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

                        let mis_count = tree.count_mis();

                        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

                        println!("MIS count: {}", mis_count);

                        match save_result_to_json(&filename, tree.nodes.len(), mis_count, end - start) {
                            Ok(filename) => println!("Result saved in {}", filename),
                            Err(e) => eprintln!("Failed to save result: {}", e),
                        }
                    },
                    Err(e) => eprintln!("Failed to read file: {}", e),
                }
            },
            "2" => {
                println!("Enter node count (default 10): ");

                let node_count = read_input().parse::<usize>().unwrap_or(10);

                println!("Enter max children (default 3): ");

                let max_children = read_input().parse::<usize>().unwrap_or(3);

                println!("Generating a tree with {} nodes, each node with at most {} children", node_count, max_children);

                let tree = Tree::generate(node_count, max_children);

                tree.print();

                match save_tree_to_csv(&tree) {
                    Ok(filename) => {
                        println!("Saved as {}", filename);

                        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

                        let mis_count = tree.count_mis();

                        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

                        println!("MIS count: {}", mis_count);

                        match save_result_to_json(&filename, tree.nodes.len(), mis_count, end - start) {
                            Ok(filename) => println!("Result saved in {}", filename),
                            Err(e) => eprintln!("Failed to save result: {}", e),
                        }
                    },
                    Err(e) => eprintln!("Failed to save: {}", e)
                }
            },
            "3" => {
                break;
            },
            _ => {
                continue;
            },
        }
    }
}