use std::collections::HashMap;
use std::fs;
use bytebuffer::ByteBuffer;
use huffman::{count_characters, get_leaves, get_heap, encode_contents, Node};

fn main() {

    // 1. Read characters into map
    let contents: String = fs::read_to_string("book.txt".to_string()).expect("File not found!");

    let map = count_characters(&contents);

    println!("Number of unique characters: {}", map.len());

    // 2. Sort HashMap entries by frequency
    let leaves = get_leaves(map);

    let characters: Vec<char> = leaves.iter().map(|x: &Node| x.c.unwrap()).collect();

    println!("{:?}", leaves);

    // 3. build heap/tree
    let heap = get_heap(leaves);

    let mut codes: HashMap<char, String> = HashMap::new();

    heap.peek().unwrap().0.get_codes("".to_string(), &mut codes);

    // println!("Huffman codes: {:?}", codes);

    // 4. Encode tree into binary
    let mut binary_string = "".to_string();

    heap.peek().unwrap().0.save_tree(&mut binary_string);

    // 5. Build a buffer
    let byte_buffer: ByteBuffer = encode_contents(&binary_string, characters, &contents, codes);


    // println!("{}", binary_string);

    // let mut sum = 0;
    // for c in binary_string.chars() {
    //     sum += c as i32 - 48;
    // }

    // println!("Number of leaves: {}", sum);

    // 6. Write out to the file
    fs::write("output.txt", byte_buffer.to_bytes()).expect("Unable to write file");

    // 6. Read the file back in and decode back to strings

    // 7. Encode and load the Huffman tree

    // 8. clean up, separate into modules create command-line utility


}
