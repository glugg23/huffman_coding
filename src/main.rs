use std::collections::btree_map::BTreeMap;
use std::collections::binary_heap::BinaryHeap;
use std::collections::HashMap;
use std::env;

#[derive(Debug, Eq, PartialEq)]
enum NodeKind {
    Internal(Box<Node>, Box<Node>),
    Leaf(char)
}

#[derive(Debug, Eq, PartialEq)]
struct Node {
    freq: usize,
    kind: NodeKind
}

impl Ord for Node {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        rhs.freq.cmp(&self.freq)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&rhs))
    }
}

fn generate_codes(node: &Node, prefix: Vec<u8>, out_codes: &mut HashMap<char, Vec<u8>>) {
    match node.kind {
        NodeKind::Internal(ref left, ref right) => {
            let mut left_prefix = prefix.clone();
            left_prefix.push(0);
            generate_codes(&left, left_prefix, out_codes);

            let mut right_prefix = prefix;
            right_prefix.push(1);
            generate_codes(&right, right_prefix, out_codes);
        }
        NodeKind::Leaf(ch) => {
            out_codes.insert(ch, prefix);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: huffman_coding [filename]");
        return;
    }

    let text = &args[1];

    let mut freq = BTreeMap::new();

    for ch in text.chars() {
        *freq.entry(ch).or_insert(0) += 1;
    }

    let mut prior_freq = BinaryHeap::new();

    for entry in freq {
        prior_freq.push(Node {
            freq: entry.1,
            kind: NodeKind::Leaf(entry.0)
        });
    }

    while prior_freq.len() > 1 {
        let left = prior_freq.pop().unwrap();
        let right = prior_freq.pop().unwrap();

        prior_freq.push(Node {
            freq: left.freq + right.freq,
            kind: NodeKind::Internal(Box::new(left), Box::new(right))
        });
    }

    let mut codes = HashMap::new();
    generate_codes(prior_freq.peek().unwrap(), vec![0u8; 0], &mut codes);
}
