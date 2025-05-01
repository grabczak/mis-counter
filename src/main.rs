use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::collections::HashMap;

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

    fn print(&self) {
        for (value, children) in &self.nodes {
            print!("{} ", value);
            for child in children {
                print!("{} ", child);
            }
            println!("")
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
            "3" => {
                break;
            },
            _ => {
                continue;
            },
        }
    }
}