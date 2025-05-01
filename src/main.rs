use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use std::path::Path;
use std::collections::HashMap;
use rand::prelude::*;
use rand::rng;
use uuid::Uuid;

type Value = i32;
type Children = Vec<Value>;
type Nodes = HashMap<Value, Children>;

pub struct Tree {
    root: Value,
    nodes: Nodes,
}

impl Tree {
    fn new(root: Value, data: Vec<Vec<Value>>) -> Self {
        let mut nodes: Nodes = HashMap::new();

        for node in data {
            match node.split_first() {
                Some((value, children)) => {
                    nodes.insert(*value, children.to_vec());
                },
                None => ()
            }
        }

        Tree { root, nodes }
    }

    pub fn print(&self) {
        println!("Root: {}", self.root);
        for (node, children) in &self.nodes {
            println!("{} -> {:?}", node, children);
        }
    }

    fn children(&self, node: Value) -> Vec<Value> {
        self.nodes.get(&node).cloned().unwrap_or_else(Vec::new)
    }

    fn grandchildren(&self, node: Value) -> Vec<Value> {
        let mut result = Vec::new();

        if let Some(children) = self.nodes.get(&node) {
            for &child in children {
                if let Some(grandchildren) = self.nodes.get(&child) {
                    result.extend(grandchildren.iter().cloned());
                }
            }
        }

        result
    }

    fn post_order(&self) -> Vec<Value> {
        fn _order(tree: &Tree, node: Value, result: &mut Vec<Value>) {
            if let Some(children) = tree.nodes.get(&node) {
                for &child in children {
                    _order(tree, child, result);
                }
            }

            result.push(node);
        }

        let mut result = Vec::new();

        _order(self, self.root, &mut result);

        result
    }

    fn count_mis(&self) -> i32 {
        let mut mu: HashMap<Value, i32> = HashMap::new();
        let mut nu: HashMap<Value, i32> = HashMap::new();

        let post_order_nodes = self.post_order();

        for node in post_order_nodes {
            let mut m = 1;
            let mut n = 1;

            for child in self.children(node) {
                m = m * mu.get(&child).unwrap_or(&0);

                n = n * nu.get(&child).unwrap_or(&0);
            }

            nu.insert(node, m - n);

            let mut k = 1;

            for grandchild in self.grandchildren(node) {
                k = k * mu.get(&grandchild).unwrap_or(&0);
            }

            k = k + nu.get(&node).unwrap_or(&0);

            mu.insert(node, k);
        }

        *mu.get(&self.root).unwrap_or(&0)
    }

    pub fn generate_random_tree(node_count: usize, max_children: usize) -> Self {
        assert!(node_count >= 1, "Tree must have at least one node");

        let mut rng = rng();
        let mut nodes: Nodes = HashMap::new();
        let all_values: Vec<Value> = (0..node_count as i32).collect(); // root is 0

        let root = 0;
        let mut index = 1; // Start from 1 (0 is root)
        let mut queue = vec![root];

        while !queue.is_empty() && index < node_count {
            let parent = queue.remove(0);
            let mut children = Vec::new();

            let remaining = node_count - index;
            let max_assignable = remaining.min(max_children);
            let child_count = rng.random_range(1..=max_assignable);

            for _ in 0..child_count {
                let child = all_values[index];
                index += 1;
                children.push(child);
                queue.push(child);
            }

            nodes.insert(parent, children);
        }

        for value in all_values.iter() {
            nodes.entry(*value).or_insert_with(Vec::new);
        }

        Tree { root, nodes }
    }

    pub fn save_to_file_as_csv(&self) -> std::io::Result<String> {
        let filename = format!("{}.csv", Uuid::new_v4());
        let file = File::create(&filename)?;
        let mut writer = BufWriter::new(file);

        for (parent, children) in &self.nodes {
            let line = format!("{} {}", parent, children.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" "));
            writeln!(writer, "{}", line)?;
        }

        Ok(filename)
    }
}

fn read_file(filename: &str) -> io::Result<Vec<Vec<i32>>> {
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

fn save_file(filename: &str, value: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(value.as_bytes())?;
    Ok(())
}

fn insert_result_suffix(filename: &str) -> String {
    let path = Path::new(filename);
    let parent = path.parent().unwrap_or_else(|| Path::new(""));

    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    let extension = path.extension().unwrap_or_default().to_string_lossy();

    let new_filename = if extension.is_empty() {
        format!("{stem}-result")
    } else {
        format!("{stem}-result.{extension}")
    };

    parent.join(new_filename).to_string_lossy().to_string()
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

                match read_file(filename) {
                    Ok(data) => {
                        let tree = Tree::new(0, data);

                        println!("Loaded tree from {}", filename);

                        tree.print();

                        let mis_count = tree.count_mis();

                        println!("MIS count: {}", mis_count);

                        let result_filename = insert_result_suffix(filename);

                        match save_file(&result_filename, &mis_count.to_string()) {
                            Ok(()) => println!("Result saved in {}", result_filename),
                            Err(e) => eprintln!("Failed to save result: {}", e),
                        }
                    },
                    Err(e) => eprintln!("Failed to read file: {}", e),
                }
            },
            "2" => {
                let tree = Tree::generate_random_tree(10, 3);

                println!("Generated tree: ");

                tree.print();

                match tree.save_to_file_as_csv() {
                    Ok(filename) => println!("Saved as {}", filename),
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