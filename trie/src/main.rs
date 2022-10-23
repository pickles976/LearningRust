use std::fs;
use trie::Trie;

fn main() {

    let mut trie = Trie::new();

    let contents = fs::read_to_string("dictionary.txt").expect("File not found!");

    for word in contents.lines() {
        trie.insert(word.to_string());
    }

    // trie.print_all();

    println!("Successfully indexed all words in the english language!");

}