# MIS Counter

A Rust implementation for counting the number of maximal independent sets (MIS) in a tree, based on a linear-time algorithm by Herbert S. Wilf.

## 📦 Setup

Ensure you have [Rust](https://rustup.rs/) installed on your system. You can verify the installation by checking the version:

```bash
rustc --version
```

### 🔧 Running the Program

To run the program in development mode:

```bash
cargo run
```

### 🚀 Building a Release

To compile a release version of the executable:

```bash
cargo build --release
```

The release binary will be available at:

```bash
target/release/mis-tree
```

You can run the executable directly with:

```bash
./target/release/mis-tree
```

## 📥 Input / 📤 Output Format

The program reads a tree structure from text file and writes the number of _maximal independent sets (MIS)_ to a `.result` file in plain text.

The tree is represented using _adjacency lists_, where each line lists a node and its direct children. The root is assumed to be node `0`.

Each line follows the format:

```
<parent> <child_1> <child_2> ...
```

- Nodes with no children must still appear as singletons (e.g., `4`).
- The tree must be connected and acyclic.
- The order of lines does not matter, as long as the tree structure is valid.

### ✅ Example Input

```
0 1 2 3
1 4 5
2 6 7
3 8
4
5
6
7
8
```

## 🧠 Theoretical Background

Let $G = (V, E)$ be an undirected graph. A subset $V' \subseteq V$ is called _independent_, if no two vertices in $V'$ are adjacent. An independent set $V'$ is _maximal_ if it cannot be extended by adding any other vertex from $V$ while preserving independence.

Such maximal independent sets are referred to as _MIS_.

In the special case where $G$ is a tree, a linear-time algorithm exists to count all MIS. This program implements the algorithm proposed by Herbert S. Wilf in his 1986 paper.

## 📚 References

Wilf, Herbert S. 1986. “The Number of Maximal Independent Sets in a Tree.”
_SIAM Journal on Algebraic Discrete Methods_ 7 (1): 125–30.
