use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

type Value = i32;
type Children = Vec<Value>;
type Nodes = HashMap<Value, Children>;

pub struct Tree {
    root: Value,
    nodes: Nodes,
}

impl Tree {
    fn new(root: Value) -> Self {
        Tree {
            root,
            nodes: HashMap::new(),
        }
    }

    fn insert(&mut self, node: Value, children: Children) {
        self.nodes.insert(node, children);
    }

    // fn print(&self) {
    //     for (value, children) in &self.nodes {
    //         print!("{} ", value);
    //         for child in children {
    //             print!("{} ", child);
    //         }
    //         println!("")
    //     }
    // }

    fn build(&mut self, filename: &str) -> io::Result<()> {
        let file = File::open(filename)?;
        let reader = io::BufReader::new(file);
    
        let mut lines: Vec<Vec<i32>> = Vec::new();
    
        for line_result in reader.lines() {
            let line = line_result?;
            let numbers: Vec<i32> = line
                .split(' ')
                .filter_map(|s| s.trim().parse::<i32>().ok())
                .collect();
            lines.push(numbers);
        }

        for line in lines {
            match line.split_first() {
                Some((value, children)) => self.insert(*value, children.to_vec()),
                None => ()
            }
        }

        Ok(())
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

    fn mis_count(&self) -> i32 {
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

fn main() {
    let mut tree = Tree::new(0);

    match tree.build("example.csv") {
        Ok(_) => println!("Mis count: {}", tree.mis_count()),
        Err(e) => eprintln!("Error reading file: {}", e)
    }
}