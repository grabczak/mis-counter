use std::collections::{HashMap, HashSet, VecDeque};
use rand::prelude::*;
use rand::rng;
use dashu_int::UBig;
use dashu_macros::ubig;

type Node = usize;
type Children = Vec<Node>;
type Nodes = HashMap<Node, Children>;

pub struct Tree {
    root: Node,
    nodes: Nodes,
}

impl Tree {
    pub fn from_adjacency_list(root: Node, csv: Vec<Vec<Node>>) -> Self {
        let mut nodes: Nodes = HashMap::new();

        for node in csv {
            if let Some((node, children)) = node.split_first() {
                nodes.insert(*node, children.to_vec());
            }
        }

        Tree { root, nodes }
    }

    pub fn nodes(&self) -> Nodes {
        self.nodes.clone()
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    fn children(&self, node: Node) -> Children {
        self.nodes.get(&node).cloned().unwrap_or_else(Vec::new)
    }

    fn grandchildren(&self, node: Node) -> Children {
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

    fn post_order(&self) -> Vec<Node> {
        fn _order(tree: &Tree, node: Node, result: &mut Vec<Node>) {
            for child in tree.children(node) {
                _order(tree, child, result);
            }

            result.push(node);
        }

        let mut result = Vec::new();

        _order(self, self.root, &mut result);

        result
    }

    pub fn count_mis(&self) -> String {
        let mut with_node: HashMap<Node, UBig> = HashMap::new();
        let mut without_node: HashMap<Node, UBig> = HashMap::new();

        let post_order_nodes = self.post_order();

        for node in post_order_nodes {
            let mut m: UBig = ubig!(1);
            let mut n: UBig = ubig!(1);

            for child in self.children(node) {
                m = m * with_node.get(&child).unwrap_or(&ubig!(0));

                n = n * without_node.get(&child).unwrap_or(&ubig!(0));
            }

            without_node.insert(node, m - n);

            let mut k: UBig = ubig!(1);

            for grandchild in self.grandchildren(node) {
                k = k * with_node.get(&grandchild).unwrap_or(&ubig!(0));
            }

            k = k + without_node.get(&node).unwrap_or(&ubig!(0));

            with_node.insert(node, k);
        }

        with_node.get(&self.root).unwrap_or(&ubig!(0)).to_string()
    }

    pub fn print(&self) {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(self.root);
        visited.insert(self.root);

        while let Some(current) = queue.pop_front() {
            let children = self.children(current);

            println!("{} -> [{}]", current, children.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(", "));

            for &child in &children {
                if !visited.contains(&child) {
                    queue.push_back(child);
                    visited.insert(child);
                }
            }
        }
    }

    pub fn generate(node_count: usize, max_children: usize) -> Self {
        let mut thread_rng = rng();
        let mut nodes: Nodes = HashMap::new();
        let all_values: Vec<Node> = Vec::from_iter(0..node_count); // root is 0

        let root = 0;
        let mut index = 1;
        let mut queue = VecDeque::from([0]);

        while !queue.is_empty() && index < node_count {
            let parent = queue.pop_front().unwrap();
            let mut children = Vec::new();

            let remaining = node_count - index;
            let max_assignable = remaining.min(max_children);
            let child_count = thread_rng.random_range(1..=max_assignable);

            for _ in 0..child_count {
                let child = all_values[index];
                index += 1;
                children.push(child);
                queue.push_back(child);
            }

            nodes.insert(parent, children);
        }

        for value in all_values.iter() {
            nodes.entry(*value).or_insert_with(Vec::new);
        }

        Tree { root, nodes }
    }
}
