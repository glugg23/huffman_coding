use std::collections::binary_heap::BinaryHeap;
use std::collections::btree_map::BTreeMap;
use std::collections::HashMap;
use std::fs::File;
use std::hash::BuildHasher;
use std::io::{BufWriter, Result, Write};

use byteorder::{BigEndian, WriteBytesExt};
use serde::{Deserialize, Serialize};

pub fn generate_tree(text: &str) -> Node {
    let mut freq = BTreeMap::new();

    for ch in text.chars() {
        *freq.entry(ch).or_insert(0) += 1;
    }

    let mut prior_freq = BinaryHeap::new();

    for entry in freq {
        prior_freq.push(Node {
            freq: entry.1,
            kind: NodeKind::Leaf(entry.0),
        });
    }

    while prior_freq.len() > 1 {
        let left = prior_freq.pop().unwrap();
        let right = prior_freq.pop().unwrap();

        prior_freq.push(Node {
            freq: left.freq + right.freq,
            kind: NodeKind::Internal(Box::new(left), Box::new(right)),
        });
    }

    prior_freq.pop().unwrap()
}

pub fn generate_codes<S: BuildHasher>(
    node: &Node,
    prefix: Vec<u8>,
    out_codes: &mut HashMap<char, Vec<u8>, S>,
) {
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

pub fn encode_text<S: BuildHasher>(text: &str, codes: &HashMap<char, Vec<u8>, S>) -> Vec<u8> {
    let mut encoded_text = Vec::new();

    for ch in text.chars() {
        encoded_text.append(&mut codes[&ch].clone());
    }

    encoded_text
        .chunks(8)
        .map(|chunk| {
            let mut output = 0b0000_0000;

            for (i, &bit) in chunk.iter().enumerate() {
                if bit == 1 {
                    match i {
                        0 => output |= 0b1000_0000,
                        1 => output |= 0b0100_0000,
                        2 => output |= 0b0010_0000,
                        3 => output |= 0b0001_0000,
                        4 => output |= 0b0000_1000,
                        5 => output |= 0b0000_0100,
                        6 => output |= 0b0000_0010,
                        7 => output |= 0b0000_0001,
                        _ => (),
                    }
                }
            }

            output
        })
        .collect::<Vec<u8>>()
}

pub fn write_text(path: &str, encoded_text: &[u8], tree: &Node) -> Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    //Encode tree into binary
    let tree = bincode::serialize(&tree).unwrap();

    //Write the size of the tree in bytes to the start of the file
    writer.write_u32::<BigEndian>(tree.len() as u32)?;

    for byte in tree {
        writer.write_all(&[byte])?;
    }

    for &byte in encoded_text {
        writer.write_all(&[byte])?;
    }

    Ok(())
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum NodeKind {
    Internal(Box<Node>, Box<Node>),
    Leaf(char),
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub freq: usize,
    pub kind: NodeKind,
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
