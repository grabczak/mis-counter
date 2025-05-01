use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

mod tree;
use tree::Tree;

#[derive(Serialize, Deserialize, Debug)]
struct Result {
    node_count: i32,
    mis_count: i32,
    running_time: i32,
}

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
    let filename = format!("{}.csv", Uuid::new_v4());
    let file = File::create(&filename)?;
    let mut writer = BufWriter::new(file);

    for (parent, children) in &tree.nodes {
        let line = format!("{} {}", parent, children.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" "));
        writeln!(writer, "{}", line)?;
    }

    Ok(filename)
}

fn save_result_to_json(filename: &str) -> io::Result<String> {
    let result = Result {
        node_count: 0,
        mis_count: 0,
        running_time: 0,
    };

    let serialized = serde_json::to_string_pretty(&result).unwrap();

    let result_filename = format!("{filename}.json");

    let mut file = File::create(&result_filename)?;

    file.write_all(serialized.as_bytes())?;

    Ok(result_filename)
}

fn main() {
    loop {
        println!("Pick an option: ");
        println!("1. Read");
        println!("2. Generate");
        println!("3. Quit");

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read filename");

        let option = option.trim();

        match option {
            "1" => {
                println!("Enter the filename: ");

                let mut filename = String::new();

                io::stdin()
                    .read_line(&mut filename)
                    .expect("Failed to read filename");

                let filename = filename.trim();

                match read_tree_from_csv(filename) {
                    Ok(data) => {
                        let tree = Tree::new(0, data);

                        println!("Loaded tree from {}", filename);

                        tree.print();

                        let mis_count = tree.count_mis();

                        println!("MIS count: {}", mis_count);

                        match save_result_to_json(&filename) {
                            Ok(filename) => println!("Result saved in {}", filename),
                            Err(e) => eprintln!("Failed to save result: {}", e),
                        }
                    },
                    Err(e) => eprintln!("Failed to read file: {}", e),
                }
            },
            "2" => {
                println!("Enter node count: ");

                let mut node_count = String::new();

                io::stdin()
                    .read_line(&mut node_count)
                    .expect("Failed to read node count");

                println!("Enter max children: ");

                let mut max_children = String::new();

                io::stdin()
                    .read_line(&mut max_children)
                    .expect("Failed to read max children");

                let node_count = node_count.trim().parse::<usize>().unwrap_or(10);
                let max_children = max_children.trim().parse::<usize>().unwrap_or(3);

                println!("node_count: {}", node_count);
                println!("max_children: {}", max_children);

                let tree = Tree::generate_random_tree(node_count, max_children);

                println!("Generated tree: ");

                tree.print();

                match save_tree_to_csv(&tree) {
                    Ok(filename) => {
                        println!("Saved as {}", filename);

                        let mis_count = tree.count_mis();

                        println!("MIS count: {}", mis_count);

                        match save_result_to_json(&filename) {
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