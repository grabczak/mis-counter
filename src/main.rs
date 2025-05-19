use std::io::{self, Write};
use std::time::Instant;
use std::path::PathBuf;

mod csv;
use csv::{read_tree_from_csv, save_tree_to_csv, save_result_to_file};

mod tree;
use tree::Tree;

fn read_line_input() -> String {
    let mut input = String::new();

    let _ = io::stdout().flush();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

fn display_mis_count(path: &PathBuf) {
    println!("\nLoading tree...");

    match read_tree_from_csv(path.to_path_buf()) {
        Ok(data) => {
            let tree = Tree::from_adjacency_list(0, data);

            println!("Tree loaded from {}", path.display());

            let node_count = tree.node_count();

            print!("\nTree has {} node(s). Display tree? (y/n) >> ", node_count);

            let ask_display_tree = read_line_input();

            if ask_display_tree.to_lowercase() == "y" {
                println!();
                tree.print();
            }

            println!("\nCounting MIS...");

            let start = Instant::now();

            let mis_count = tree.count_mis();

            let duration = start.elapsed().as_millis();

            println!("Completed in {} ms", duration);

            let mis_count_length = mis_count.len();

            print!("\nMIS count has {} digit(s). Display MIS count? (y/n) >> ", mis_count_length);

            let ask_display_mis_count = read_line_input();

            if ask_display_mis_count.to_lowercase() == "y" {
                println!("\n{}", mis_count);
            }

            println!("\nSaving result...");

            match save_result_to_file(path.to_path_buf(), mis_count) {
                Ok(path) => println!("Result saved as {}", path.display()),
                Err(e) => eprintln!("Failed to save result >> {}", e),
            }
        },
        Err(e) => eprintln!("Failed to read file >> {}", e),
    }
}

fn main() {
    loop {
        println!("MIS COUNTER VER 1.0");
        println!("1. Read");
        println!("2. Generate");
        println!("3. Quit");
        print!("Pick an option to proceed >> ");

        let option = read_line_input();

        match option.as_str() {
            "1" => {
                print!("\nPath to file >> ");

                let path = read_line_input();

                display_mis_count(&path.into());
            },
            "2" => {
                print!("\nNode count (default 10) >> ");

                let node_count = read_line_input().parse::<usize>().unwrap_or(10).max(1);

                print!("Max children (default equal to node count) >> ");

                let max_children = read_line_input().parse::<usize>().unwrap_or(node_count).clamp(1, node_count);

                println!("\nGenerating a tree with {} node(s), each node with at most {} child(ren)...", node_count, max_children);

                let tree = Tree::generate(node_count, max_children);

                println!("Tree generated successfully");

                println!("\nSaving tree...");

                match save_tree_to_csv(tree.nodes()) {
                    Ok(path) => {
                        println!("Tree saved as {}", path.display());

                        display_mis_count(&path);
                    },
                    Err(e) => eprintln!("Failed to save >> {}", e)
                }
            },
            "3" => {
                println!("\nGoodbye\n");
                break;
            },
            _ => {
                println!("\nInvalid option\n");
                continue;
            },
        }

        println!();
    }
}