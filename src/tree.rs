use std::collections::{HashMap, VecDeque};
use rand::prelude::*;
use rand::rng;

type Value = i32;
type Children = Vec<Value>;
type Nodes = HashMap<Value, Children>;

pub struct Tree {
    pub root: Value,
    pub nodes: Nodes,
}

impl Tree {
    pub fn new(root: Value, data: Vec<Vec<Value>>) -> Self {
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

    pub fn children(&self, node: Value) -> Vec<Value> {
        self.nodes.get(&node).cloned().unwrap_or_else(Vec::new)
    }

    pub fn grandchildren(&self, node: Value) -> Vec<Value> {
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

    pub fn post_order(&self) -> Vec<Value> {
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

    pub fn count_mis(&self) -> usize {
        let mut mu: HashMap<Value, usize> = HashMap::new();
        let mut nu: HashMap<Value, usize> = HashMap::new();

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

    pub fn print(&self) {
        let mut queue = VecDeque::new();
        let mut visited = std::collections::HashSet::new();

        queue.push_back(self.root);
        visited.insert(self.root);

        while let Some(current) = queue.pop_front() {
            let children = self.nodes.get(&current).cloned().unwrap_or_default();
            println!("{} -> {}", current, children.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" "));
            for &child in &children {
                if !visited.contains(&child) {
                    queue.push_back(child);
                    visited.insert(child);
                }
            }
        }
    }

    pub fn generate(node_count: usize, max_children: usize) -> Self {
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
}
