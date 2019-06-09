use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, BufWriter, Write};

use bitvec::{prelude::BigEndian, vec::BitVec};
use byteorder::ReadBytesExt;
use clap::{App, AppSettings, Arg, crate_authors, crate_name, crate_version};

use huffman_coding::*;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("A program to compress and extract files using Huffman Encoding")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("compress")
                .takes_value(true)
                .short("c")
                .long("compress")
                .value_name("FILE")
                .help("Compress a given file")
                .conflicts_with("extract"),
        )
        .arg(
            Arg::with_name("extract")
                .takes_value(true)
                .short("x")
                .long("extract")
                .value_name("FILE")
                .help("Extract a given file")
                .conflicts_with("compress"),
        )
        .get_matches();

    if matches.is_present("compress") {
        let filename = matches.value_of("compress").unwrap();

        let mut file = File::open(filename).expect("ERROR: File not found");

        let mut text = String::new();
        file.read_to_string(&mut text).unwrap();

        let tree = generate_tree(text.as_str());

        let mut codes = HashMap::new();
        generate_codes(&tree, vec![0u8; 0], &mut codes);

        let encoded_text = encode_text(text.as_str(), &codes);

        let path = format!("{}.out", filename);
        write_text(path.as_str(), &encoded_text, &tree)
            .expect("ERROR: Failed to write output file");
    } else if matches.is_present("extract") {
        let filename = matches.value_of("extract").unwrap();

        let file = File::open(filename).expect("ERROR: File not found");
        let mut reader = BufReader::new(file);

        let tree_size = reader
            .read_u32::<byteorder::BigEndian>()
            .expect("ERROR: Failed to read tree size");

        let mut tree = Vec::new();
        for _ in 0..tree_size {
            tree.push(reader.read_u8().expect("ERROR: Failed to read tree size"));
        }

        let tree: Node = bincode::deserialize(&tree).unwrap();

        let mut encoded_text = Vec::new();
        reader
            .read_to_end(&mut encoded_text)
            .expect("ERROR: Failed to read text");

        println!("Read file");

        let mut encoded_text: BitVec<BigEndian, u8> =
            encoded_text.iter().fold(BitVec::new(), |mut acc, &byte| {
                acc.append::<BigEndian, u8>(&mut BitVec::from_element(byte));
                acc
            });

        println!("Converted file to bits");

        let mut output = Vec::new();
        while encoded_text.len() != 0 {
            println!("{} bits, {} chars", encoded_text.len(), output.len());
            decode_bytes(&tree, &mut encoded_text, &mut output);
        }

        println!("Decoded bits");

        let path = format!("{}.txt", filename);
        let file = File::create(path).expect("ERROR: Failed to create file");
        let mut writer = BufWriter::new(file);

        for ch in output {
            writer.write(&[ch as u8]).unwrap();
        }
    }
}

fn decode_bytes(node: &Node, bits: &mut BitVec<BigEndian, u8>, output: &mut Vec<char>) {
    match node.kind {
        NodeKind::Internal(ref left, ref right) => {
            if bits.len() == 0 {
                //BitVec::remove panics when index is out of range, and this panic cannot be caught
                return;
            } else {
                let bit = bits.remove(0);
                //if bit == 1
                if bit {
                    decode_bytes(right, bits, output);
                } else {
                    decode_bytes(left, bits, output);
                }
            }
        }
        NodeKind::Leaf(ch) => {
            output.push(ch);
        }
    }
}
