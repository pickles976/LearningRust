use std::cmp::{Ordering, Reverse};
use std::collections::{HashMap, BinaryHeap};
use std::option::Option;
use bytebuffer::ByteBuffer;


#[derive(PartialEq, Eq, PartialOrd, Debug)]
pub struct Node {
    pub freq: u32,
    pub c: Option<char>,
    pub r: Option<Box<Node>>,
    pub l: Option<Box<Node>>,
}

impl Ord for Node {

    // TODO: why is this cmp implementation not working?
    // Using Reverse is ugly and annoying
    fn cmp(&self, other: &Self) -> Ordering {
        self.freq.cmp(&other.freq)
    }
}

// TODO: possibly create a tree struct that holds the root node and some cleaner interfaces?
impl Node {

    pub fn leaf(c: char, freq: u32) -> Node {
        Node {                         
            freq,
            c: Some(c),
            l: None,
            r: None,
        }
    }

    pub fn node (right: Option<Node>, left: Option<Node>) -> Node {

        let right = right.unwrap();
        let left = left.unwrap();

        Node{
            freq: right.freq + left.freq,
            c: None,
            l: Some(Box::new(left)),
            r: Some(Box::new(right)),
        }
    }

    // get_codes all nodes and save char -> code mappings
    pub fn get_codes(&self, s: String, map: &mut HashMap<char, String>){

        // go right
        if let Some(node) = &self.r {
            let mut temp = s.clone();
            temp.push('1');
            node.get_codes(temp, map)
        }

        // go left
        if let Some(node) = &self.l {
            let mut temp = s.clone();
            temp.push('0');
            node.get_codes(temp, map)
        }

        if let Some(c) = self.c {
            map.insert(c, s);
        }

    }

    // write tree to string of ones and zeros
    pub fn save_tree(&self, s: &mut String){

        if self.c.is_some() {
            s.push('1');
        } else {
            s.push('0');
        }

        // go right
        if let Some(node) = &self.r {
            node.save_tree(s);
        }

        // go left
        if let Some(node) = &self.l {
            node.save_tree(s);
        }

    }

    // populate the tree leaves with characters
    pub fn populate_tree(mut self, byte_buffer: &mut ByteBuffer) -> Self {

        // go right
        if let Some(mut node) = self.r {
            self.r = Some(Box::new(node.populate_tree(byte_buffer)));
        }

        // go left
        if let Some(mut node) = self.l {
            self.l = Some(Box::new(node.populate_tree(byte_buffer)));
        }

        if let Some(c) = self.c {
            self.c = char::from_u32(byte_buffer.read_u32());
        }

        self

    }

    // use bytearray to navigate tree until we reach a leaf
    pub fn decode_bytearray(&self, output: &mut String, bit_seq: &mut BitSeq, byte_buffer: &mut ByteBuffer) {

        match self.c {
            Some(c) => {
                output.push(c);
            },
            None => {

                let bit;
    
                match bit_seq.try_read_bit() {
                    Some(num) => bit = num,
                    None => { 
                        // this should probably be a method
                        bit_seq.value = byte_buffer.read_u32();
                        bit_seq.index = 0;
                        bit = bit_seq.try_read_bit().unwrap();
                    },
                }
    
                if bit == 1 { // build leaf
                    if let Some(right) = &self.r {
                        right.decode_bytearray(output, bit_seq, byte_buffer);
                    }
                } else {
                    if let Some(left) = &self.l {
                        left.decode_bytearray(output, bit_seq, byte_buffer);
                    }
                }
    
            },
        }
            
    }

    pub fn get_character_order(&self, characters: &mut String){

        match self.c {
            Some(c) => characters.push(c),
            None => {
                if let Some(right) = &self.r {
                    right.get_character_order(characters);
                }

                if let Some(left) = &self.l {
                    left.get_character_order(characters);
                }
            }
        }

    }

}

pub fn count_characters(contents: &String) -> HashMap<char, u32> {

    let mut map: HashMap<char, u32> = HashMap::new();

    contents.chars().for_each(|c| *map.entry(c).or_default() += 1 );

    map

}

pub fn get_leaves(map: HashMap<char, u32>) -> Vec<Node> {

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

    // println!("Heap size: {}", heap.len());
    // println!("Smallest element is: {:?}", heap.peek().unwrap().0);

    while heap.len() > 1 {

        let right = heap.pop().unwrap().0; // is there a better way to do this than unwrapping???
        let left = heap.pop().unwrap().0; // and zeroing
        
        let new_node = Node::node(Some(right), Some(left));

        heap.push(Reverse(new_node));

    }

    heap

}

// TODO: This feels really messy and inelegant. Is there a better way to do this?
pub fn encode_contents(binary_string: &String, characters: &String, contents: &String, codes: HashMap<char, String>) -> ByteBuffer {

    let mut bit_buffer : BitSeq = BitSeq::new();
    let mut byte_buffer : ByteBuffer = ByteBuffer::new();

    // add tree to byte buffer
    for bit in binary_string.chars() {

        if !bit_buffer.try_add_bit(bit as u32 - 48) {

            // Add the u32 in bit_buffer to some sort of bytearray
            byte_buffer.write_u32(bit_buffer.value);

            bit_buffer = BitSeq::new();

            bit_buffer.try_add_bit(bit as u32 - 48);
        }
        
    }

    byte_buffer.write_u32(bit_buffer.value);
    bit_buffer = BitSeq::new();

    // add chars to byte buffer
    characters.chars().for_each(|c| byte_buffer.write_u32(c.clone() as u32) );

    // TODO: default hashmap implementation is SLOOOOOOOOOW
    // for char in file
    contents.chars().for_each(|c| {
        let bits = codes.get(&c).unwrap();

        for bit in bits.chars() {

            if !bit_buffer.try_add_bit(bit as u32 - 48) {

                // Add the u32 in bit_buffer to some sort of bytearray
                byte_buffer.write_u32(bit_buffer.value);

                bit_buffer = BitSeq::new();

                bit_buffer.try_add_bit(bit as u32 - 48);
            }
        }
    });

    byte_buffer.write_u32(bit_buffer.value);
    byte_buffer.write_u32(0); // EOF buffer
    byte_buffer

}

pub fn rebuild_tree(bit_seq: &mut BitSeq, byte_buffer: &mut ByteBuffer) -> Node {

    let bit;

    match bit_seq.try_read_bit() {
        Some(num) => bit = num,
        None => { 
            // this should probably be a method
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

// Why use this weird middleman for writing bits into the byte buffer?
// It's much easier to read/write bytes with some padding.
// It also makes it easier to test since I know that reading 
// 8 bytes at a time wont result in some sort of weird data overlap
// "But muh overhead!" -- writing bits directly to ByteBuffer is the same
// speed. This is awkward but at least I can debug easier.
pub struct BitSeq {
    pub value: u32,
    pub index: u32,
}

impl BitSeq {
    pub fn new() -> BitSeq {
        BitSeq { value: 0, index: 0 }
    }

    pub fn from_bytes(bytes: u32) -> BitSeq {
        BitSeq { value: bytes, index: 0 }
    }

    // insert bit if space is left
    pub fn try_add_bit(&mut self, bit : u32) -> bool {
        if self.index < 32 {
            if bit == 1 {
                self.value += bit << self.index;
            }
            self.index += 1;
            return true
        }
        false
    }

    pub fn try_read_bit(&mut self) -> Option<u32> {
        if self.index < 32 {
            let bit = Some((self.value >> self.index) & 1);
            self.index += 1;
            return bit
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use bytebuffer::ByteBuffer;

    use crate::{count_characters, get_leaves, get_heap, Node, encode_contents, rebuild_tree, BitSeq};

    #[test]
    fn test_counting() {
        let map = count_characters(&"mmmmaao".to_string());
        assert_eq!(4, map.get(&'m').unwrap().clone());
        assert_eq!(2, map.get(&'a').unwrap().clone());
        assert_eq!(1, map.get(&'o').unwrap().clone());
        assert!(map.get(&'f').is_none());
    }

    #[test]
    fn test_tree() {

        /*

            Test that tree shape is:

                None, 7
                /     \
            m, 4    None, 3
                    /       \
                  a, 2      o, 1
        */

        // basic tree
        let contents = "mmmmaao".to_string();
        let map = count_characters(&contents);
        let leaves = get_leaves(map);
        let heap = get_heap(leaves);

        let root = &heap.peek().unwrap().0;

        assert!(root.c.is_none());

        let left = root.l.as_ref().unwrap();
        assert_eq!('m' , left.c.unwrap());

        let right = root.r.as_ref().unwrap();
        assert!(right.c.is_none());

        let right_left = right.l.as_ref().unwrap();
        assert_eq!('a', right_left.c.unwrap());

        let right_right = right.r.as_ref().unwrap();
        assert_eq!('o', right_right.c.unwrap());

        /*

                    Test that tree shape is:

                        None, 11
                        /       \
                None, 7       None, 4
                /    \      /       \
            None, 4   ' '  ?, 2      ?, 2
            /     \
          ?, 2    ?, 2
        */

        // more complex tree
        let contents = "ll tt oo aa".to_string();
        let map = count_characters(&contents);
        let leaves = get_leaves(map);
        let heap = get_heap(leaves);

        let root = &heap.peek().unwrap().0;

        assert!(root.c.is_none());

        let left = root.l.as_ref().unwrap();
        assert!(left.c.is_none());

        let right = root.r.as_ref().unwrap();
        assert!(right.c.is_none());

        let right_left = right.l.as_ref().unwrap();
        assert!(right_left.c.is_some());

        let right_right = right.r.as_ref().unwrap();
        assert!(right_right.c.is_some());

        let left_right = left.r.as_ref().unwrap();
        assert_eq!(' ', left_right.c.unwrap());

        let left_left = left.l.as_ref().unwrap();
        assert!(left_left.c.is_none());

        let left_left_left = left_left.l.as_ref().unwrap();
        assert!(left_left_left.c.is_some());

        let left_left_right = left_left.r.as_ref().unwrap();
        assert!(left_left_right.c.is_some());

    }

    #[test]
    fn test_tree_representation() {

        let contents = "mmmmaao".to_string();
        let map = count_characters(&contents);
        let leaves = get_leaves(map);
        let heap = get_heap(leaves);

        let mut tree_string = "".to_string();

        let _= &heap.peek().unwrap().0.save_tree(&mut tree_string);

        assert_eq!("00111", tree_string);

        let contents = "ll tt oo aa".to_string();
        let map = count_characters(&contents);
        let leaves = get_leaves(map);
        let heap = get_heap(leaves);

        let mut tree_string = "".to_string();

        let _= &heap.peek().unwrap().0.save_tree(&mut tree_string);

        assert_eq!("001101011", tree_string);

    }

    #[test]
    fn test_codes() {

        // easy encoding
        let contents = "mmmmaao".to_string();
        let map = count_characters(&contents);
        let leaves = get_leaves(map);
        let heap = get_heap(leaves);

        let mut codes: HashMap<char, String> = HashMap::new();
        heap.peek().unwrap().0.get_codes("".to_string(), &mut codes);

        assert_eq!("0", codes.get(&'m').unwrap());
        assert_eq!("10", codes.get(&'a').unwrap());
        assert_eq!("11", codes.get(&'o').unwrap());
        assert!(codes.get(&'f').is_none());

        // harder encoding
        let contents = "ll aa oo tt".to_string();
        let map = count_characters(&contents);
        let leaves = get_leaves(map);
        let heap = get_heap(leaves);

        let mut codes: HashMap<char, String> = HashMap::new();
        heap.peek().unwrap().0.get_codes("".to_string(), &mut codes);

        assert_eq!("01", codes.get(&' ').unwrap());
        assert!(codes.get(&'f').is_none());

    }

    #[test]
    fn test_encoding() {

        // easy tree
        let contents = "mmmmaao".to_string();
        let map = count_characters(&contents);
        let leaves = get_leaves(map);
        let heap = get_heap(leaves);
        let mut characters: String = "".to_string();
        heap.peek().unwrap().0.get_character_order(&mut characters);
        let mut codes: HashMap<char, String> = HashMap::new();
        heap.peek().unwrap().0.get_codes("".to_string(), &mut codes);
        let mut binary_string = "".to_string();
        heap.peek().unwrap().0.save_tree(&mut binary_string);

        let mut byte_buffer: ByteBuffer = encode_contents(&binary_string, &characters, &contents, codes);
        let mut byte;

        // 4 (tree) + 4 * 3 (chars) + 4 (data) + 4 (padding) = 24 bytes
        assert_eq!(24, byte_buffer.len());
        
        // tree 00111 -> 28
        byte = byte_buffer.read_u32();

        assert_eq!(28, byte);

        // chars 'o', 'a', 'm'
        byte = byte_buffer.read_u32();

        assert_eq!('o' as u32, byte);

        byte = byte_buffer.read_u32();

        assert_eq!('a' as u32, byte);

        byte = byte_buffer.read_u32();

        assert_eq!('m' as u32, byte);

        // text 0000101011 -> 336
        byte = byte_buffer.read_u32();

        assert_eq!(848, byte);

        // hard tree
        let contents = "ll tt oo aa".to_string();
        let map = count_characters(&contents);
        let leaves = get_leaves(map);
        let heap = get_heap(leaves);
        let mut characters: String = "".to_string();
        heap.peek().unwrap().0.get_character_order(&mut characters);
        let mut codes: HashMap<char, String> = HashMap::new();
        heap.peek().unwrap().0.get_codes("".to_string(), &mut codes);
        let mut binary_string = "".to_string();
        heap.peek().unwrap().0.save_tree(&mut binary_string);

        let mut byte_buffer: ByteBuffer = encode_contents(&binary_string, &characters, &contents, codes);
        let mut byte;



        // 4 (tree) + 4 * 5 (chars) + 4 (data) + 4 (padding) = 32 bytes
        assert_eq!(32, byte_buffer.len());
        
        // tree 001101011 -> 428
        byte = byte_buffer.read_u32();

        assert_eq!(428, byte);

        // chars '?', '?', ' ', '?', '?'
        byte = byte_buffer.read_u32();
        byte = byte_buffer.read_u32();
        byte = byte_buffer.read_u32();

        assert_eq!(' ' as u32, byte);

    }

    #[test]
    fn test_tree_from_bytes() {

        /*

            Test that tree shape is:

                None, 7
                /     \
            m, 4    None, 3
                    /       \
                  a, 2      o, 1
        */

        let bytes = [ 0, 0, 0, 28, 0, 0, 0, 'o' as u8, 0, 0, 0, 'a' as u8, 0, 0, 0, 'm' as u8 ];

        let mut byte_buffer = ByteBuffer::new();
        byte_buffer.write_bytes(&bytes);
        let mut bit_seq = BitSeq::from_bytes(byte_buffer.read_u32());
        let mut root = rebuild_tree(&mut bit_seq, &mut byte_buffer);
        root = root.populate_tree(&mut byte_buffer);

        assert!(root.c.is_none());

        let left = root.l.as_ref().unwrap();
        assert_eq!('m' , left.c.unwrap());

        let right = root.r.as_ref().unwrap();
        assert!(right.c.is_none());

        let right_left = right.l.as_ref().unwrap();
        assert_eq!('a', right_left.c.unwrap());

        let right_right = right.r.as_ref().unwrap();
        assert_eq!('o', right_right.c.unwrap());

        /*
                    Test that tree shape is:

                        None, 11
                        /       \
                None, 7       None, 4
                /    \      /       \
            None, 4   ' '  o, 2      a, 2
            /     \
          l, 2    t, 2
        */

        let bytes = [ 0, 0, 1, 172, 0, 0, 0, 'a' as u8, 0, 0, 0, 'o' as u8, 0, 0, 0, ' ' as u8, 0, 0, 0, 'l' as u8, 0, 0, 0, 't' as u8 ];

        let mut byte_buffer = ByteBuffer::new();
        byte_buffer.write_bytes(&bytes);
        let mut bit_seq = BitSeq::from_bytes(byte_buffer.read_u32());
        let mut root = rebuild_tree(&mut bit_seq, &mut byte_buffer);
        root = root.populate_tree(&mut byte_buffer);

        assert!(root.c.is_none());

        let right = root.r.as_ref().unwrap();
        assert!(right.c.is_none());

        let left = root.l.as_ref().unwrap();
        assert!(left.c.is_none());

        let right_right = right.r.as_ref().unwrap();
        assert_eq!('a', right_right.c.unwrap());

        let right_left = right.l.as_ref().unwrap();
        assert_eq!('o', right_left.c.unwrap());

        let left_right = left.r.as_ref().unwrap();
        assert_eq!(' ', left_right.c.unwrap());

        let left_left = left.l.as_ref().unwrap();
        assert!(left_left.c.is_none());

        let left_left_right = left_left.r.as_ref().unwrap();
        assert_eq!('l', left_left_right.c.unwrap());

        let left_left_left = left_left.l.as_ref().unwrap();
        assert_eq!('t', left_left_left.c.unwrap());

    }

}