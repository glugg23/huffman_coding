use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

use huffman_coding::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: huffman_coding [filename]");
        return;
    }

    let mut file = File::open(&args[1]).expect("ERROR: File not found");

    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    let tree = generate_tree(text.as_str());

    let mut codes = HashMap::new();
    generate_codes(&tree, vec![0u8; 0], &mut codes);

    let encoded_text = encode_text(text.as_str(), &codes);

    let path = format!("{}.out", &args[1]);
    write_text(path.as_str(), &encoded_text, &tree).expect("ERROR: Failed to write output file");
}
