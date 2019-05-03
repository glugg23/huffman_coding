use std::collections::btree_map::BTreeMap;
use std::collections::binary_heap::BinaryHeap;

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

fn main() {
    let text = "test";

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
}
