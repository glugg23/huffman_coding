use std::collections::btree_map::BTreeMap;

fn main() {
    let text = "test";

    let mut freq = BTreeMap::new();

    for ch in text.chars() {
        *freq.entry(ch).or_insert(0) += 1;
    }
}
