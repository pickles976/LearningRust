use std::collections::HashMap;
use std::fs;
use bytebuffer::ByteBuffer;
use huffman::{count_characters, get_leaves, get_heap, encode_contents, Node, BitSeq};
use std::time::Instant;

fn huffman_encode_string(contents: &String) -> ByteBuffer {

    let now = Instant::now();

    println!("Total number of characters: {}", contents.len());

    // 1. Count characters by frequency
    let map = count_characters(&contents);

    println!("Number of unique characters: {}", map.len());

    // 2. Sort HashMap entries by frequency
    let leaves = get_leaves(map);

    let characters: Vec<char> = leaves.iter().map(|x: &Node| x.c.unwrap()).collect();

    // 3. build heap/tree
    let heap = get_heap(leaves);

    let mut codes: HashMap<char, String> = HashMap::new();

    heap.peek().unwrap().0.get_codes("".to_string(), &mut codes);

    // 4. Encode tree into binary
    let mut binary_string = "".to_string();

    heap.peek().unwrap().0.save_tree(&mut binary_string);

    // 5. Build a buffer
    let byte_buffer: ByteBuffer = encode_contents(&binary_string, characters, &contents, codes);

    println!("Compressed {}kb file in {}ms", contents.len() / 1000, now.elapsed().as_millis());

    byte_buffer

}

fn main() {

    let input = "book.txt".to_string();
    let output = "compressed.txt".to_string();

    // let contents: String = fs::read_to_string(input).expect("File not found!");

    let contents: String = "mmmmaao".to_string();

    let byte_buffer: ByteBuffer = huffman_encode_string(&contents);

    fs::write(&output, byte_buffer.to_bytes()).expect("Unable to write file");


    // 7. Read the file back in and decode back to strings
    let bytes = fs::read(&output).expect("Failed to open file!");

    let mut byte_buffer = ByteBuffer::new();
    byte_buffer.write_bytes(&bytes);

    // 8. Load the Huffman tree structure
    let mut bit_seq = BitSeq::from_bytes(byte_buffer.read_u32());
    let tree = rebuild_tree(&mut bit_seq, &mut byte_buffer);

    // 9. Load the characters into the tree

    // 10. Traverse the tree and decode the source file into a string

    // 11. clean up, separate into modules create command-line utility


}

fn rebuild_tree(bit_seq: &mut BitSeq, byte_buffer: &mut ByteBuffer) -> Node {

    let bit;

    match bit_seq.try_read_bit() {
        Some(num) => bit = num,
        None => { 
            // this should probably be a method
            println!("Reading new byte!");
            bit_seq.value = byte_buffer.read_u32();
            bit_seq.index = 0;
            bit = bit_seq.try_read_bit().unwrap();
        },
    }

    if bit == 1 { // build leaf
        return Node::leaf(0 as char, 0)
    }

    return Node::node(Some(rebuild_tree(bit_seq, byte_buffer)), Some(rebuild_tree(bit_seq, byte_buffer)))
    
}