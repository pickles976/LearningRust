use std::cmp::Reverse;
use std::collections::BinaryHeap;
use fxhash::FxHashMap;
use bytebuffer::ByteBuffer;
use crate::node::Node;

pub fn count_characters(contents: &String) -> FxHashMap<char, u32> {

    let mut map: FxHashMap<char, u32> = FxHashMap::default();

    contents.chars().for_each(|c| *map.entry(c).or_default() += 1 );

    map

}

pub fn get_leaves(map: FxHashMap<char, u32>) -> Vec<Node> {

    let mut leaves = Vec::new();
    
    for k in map.keys() {
        leaves.push(Node::leaf(k.clone(), map.get(k).unwrap().clone()));
    }

    leaves.sort();

    leaves
}

pub fn get_heap(leaves: Vec<Node>) -> BinaryHeap<Reverse<Node>> {

    let mut heap = BinaryHeap::new();

    for node in leaves {
        heap.push(Reverse(node));
    }

    while heap.len() > 1 {

        let right = heap.pop().unwrap().0; // is there a better way to do this than unwrapping???
        let left = heap.pop().unwrap().0; // and zeroing
        
        let new_node = Node::node(Some(right), Some(left));

        heap.push(Reverse(new_node));

    }

    heap

}

pub fn encode_contents(binary_string: &String, characters: &String, contents: &String, codes: FxHashMap<char, String>) -> ByteBuffer {

    let mut byte_buffer : ByteBuffer = ByteBuffer::new();
    
    // add size to byte buffer
    byte_buffer.write_u32(contents.chars().count() as u32);

    // add tree to byte buffer
    binary_string.chars().for_each(|bit| byte_buffer.write_bit(bit == '1'));

    for _i in 0..3 { byte_buffer.flush_bit() };

    // add chars to byte buffer
    characters.chars().for_each(|c| byte_buffer.write_u32(c.clone() as u32) );

    // for char in file
    contents.chars().for_each(|c| {
        let bits = codes.get(&c).unwrap();
        bits.chars().for_each(|bit| byte_buffer.write_bit(bit == '1'));
    });

    for _i in 0..3 { byte_buffer.flush_bit() };
    byte_buffer

}

pub fn rebuild_tree(byte_buffer: &mut ByteBuffer) -> Node {

    if byte_buffer.read_bit() { // build leaf
        return Node::leaf(0 as char, 0)
    }

    return Node::node(Some(rebuild_tree(byte_buffer)), Some(rebuild_tree(byte_buffer)))
    
}