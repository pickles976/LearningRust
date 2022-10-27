pub mod node;
pub mod utils;

use std::time::Instant;
use bytebuffer::ByteBuffer;
use fxhash::FxHashMap;
use crate::utils::{count_characters, get_leaves, get_heap, encode_contents, rebuild_tree };

// Encodes a Huffman String to a bytearray
pub fn encode(contents: &String) -> ByteBuffer {

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
pub fn decode(mut byte_buffer: ByteBuffer) -> String {

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
