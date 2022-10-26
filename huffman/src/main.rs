use fxhash::FxHashMap;
use std::fs;
use bytebuffer::ByteBuffer;
use huffman::{count_characters, get_leaves, get_heap, encode_contents, rebuild_tree};
use std::time::Instant;

// Encodes a Huffman String to a bytearray
fn huffman_encode_string(contents: &String) -> ByteBuffer {

    let now = Instant::now();

    // 1. Count characters by frequency
    let map = count_characters(&contents);

    // 2. Sort HashMap entries by frequency
    let leaves = get_leaves(map);

    // 3. build heap/tree
    let heap = get_heap(leaves);

    // 3a. get codes
    let mut codes: FxHashMap<char, String> = FxHashMap::default();
    heap.peek().unwrap().0.get_codes("".to_string(), &mut codes);

    // 3a. get characters
    let mut characters: String = "".to_string();
    heap.peek().unwrap().0.get_character_order(&mut characters);

    // 4. Encode tree  structureinto binary
    let mut binary_string = "".to_string();
    heap.peek().unwrap().0.save_tree(&mut binary_string);

    // 5. Build a buffer
    let byte_buffer: ByteBuffer = encode_contents(&binary_string, &characters, &contents, codes);

    println!("Compressed {}kb file in {}ms", contents.len() / 1000, now.elapsed().as_millis());

    byte_buffer

}

// Decodes a bytearray containing the tree and compressed data
fn huffman_decode_bytes(mut byte_buffer: ByteBuffer) -> String {

    let now = Instant::now();

    let len = byte_buffer.read_u32();

    let mut tree = rebuild_tree(&mut byte_buffer);
    for _i in 0..3 { byte_buffer.flush_bit() };

    // 2. Load the characters into the tree
    tree = tree.populate_tree(&mut byte_buffer);

    // 3. Traverse the tree and decode the source file into a string
    let mut out_string = "".to_string();

    for _i in 0..len - 1 {tree.decode_bytearray(&mut out_string, &mut byte_buffer)};

    println!("Decompressed {}kb file in {}ms", byte_buffer.len() / 1000, now.elapsed().as_millis());

    out_string
}

fn main() {

    let input = "book.txt".to_string();
    let output = "compressed.txt".to_string();

    let contents: String = fs::read_to_string(input).expect("File not found!");

    let byte_buffer: ByteBuffer = huffman_encode_string(&contents);

    fs::write(&output, byte_buffer.to_bytes()).expect("Unable to write file");


    let bytes = fs::read(&output).expect("Failed to open file!");

    let byte_buffer = ByteBuffer::from_bytes(&bytes);

    let decoded = huffman_decode_bytes(byte_buffer);

    fs::write("decompressed.txt", decoded).expect("Unable to write data");

}