use std::io;
use std::time::Instant;

mod csv;
use csv::{read_tree_from_csv, save_tree_to_csv, save_result_to_file};

mod tree;
use tree::Tree;

fn read_line_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

fn display_mis_count(filename: String) {
    println!("Loading tree...");

    match read_tree_from_csv(filename.as_str()) {
        Ok(data) => {
            let tree = Tree::from_adjacency_list(0, data);

            println!("Tree loaded from {}", filename);

            let node_count = tree.node_count();

            if node_count <= 100 {
                tree.print();
            } else {
                println!("Tree too large to display");
            }

            println!("Counting MIS...");

            let start = Instant::now();

            let mis_count = tree.count_mis();

            let duration = start.elapsed().as_millis();

            if mis_count.len() <= 100 {
                println!("MIS count: {}", mis_count);
            } else {
                println!("MIS count too large to display");
            }

            println!("Completed in {} ms", duration);

            println!("Saving result...");

            match save_result_to_file(&filename, mis_count) {
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

        let option = read_line_input();

        match option.as_str() {
            "1" => {
                println!("Filename:");

                let filename = read_line_input();

                display_mis_count(filename);
            },
            "2" => {
                println!("Node count (default 10):");

                let node_count = read_line_input().parse::<usize>().unwrap_or(10);

                println!("Max children (default equal to node count):");

                let max_children = read_line_input().parse::<usize>().unwrap_or(node_count).clamp(1, node_count);

                println!("Generating a tree with {} nodes, each node with at most {} children...", node_count, max_children);

                let tree = Tree::generate(node_count, max_children);

                println!("Tree generated successfully");

                println!("Saving tree...");

                match save_tree_to_csv(tree.nodes()) {
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

        println!("");
    }
}