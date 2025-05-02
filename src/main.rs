use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

mod tree;
use tree::Tree;

mod csv;
use csv::{read_tree_from_csv, save_tree_to_csv};

mod serialize;
use serialize::save_result_to_json;

fn read_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

fn display_mis_count(filename: String) -> () {
    match read_tree_from_csv(filename.as_str()) {
        Ok(data) => {
            let tree = Tree::new(0, data);

            println!("Tree loaded from {}", filename);

            let node_count = tree.node_count();

            if node_count <= 100 {
                println!();
                tree.print();
                println!();
            } else {
                println!("Tree too large to display");
            }

            let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

            let mis_count = tree.count_mis();

            let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

            println!("MIS count: {}", mis_count);

            let running_time = end - start;

            println!("Completed in {} ms", running_time);

            match save_result_to_json(&filename, node_count.to_string(), mis_count.to_string(), running_time.to_string()) {
                Ok(filename) => println!("Result saved as {}", filename),
                Err(e) => eprintln!("Failed to save result: {}", e),
            }
        },
        Err(e) => eprintln!("Failed to read file: {}", e),
    }
}

fn main() {
    loop {
        println!("1. Read");
        println!("2. Generate");
        println!("3. Quit");

        let option = read_input();

        println!();

        match option.as_str() {
            "1" => {
                println!("Enter the filename: ");

                let filename = read_input();

                display_mis_count(filename);
            },
            "2" => {
                println!("Enter node count (default 10): ");

                let node_count = read_input().parse::<usize>().unwrap_or(10);

                println!("Enter max children (default equal to node count): ");

                let max_children = read_input().parse::<usize>().unwrap_or(node_count).clamp(1, node_count);

                println!("Generating a tree with {} nodes, each node with at most {} children", node_count, max_children);

                let tree = Tree::generate(node_count, max_children);

                match save_tree_to_csv(tree.get_nodes()) {
                    Ok(filename) => {
                        println!("Tree saved as {}", filename);

                        display_mis_count(filename);
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

        println!();
    }
}