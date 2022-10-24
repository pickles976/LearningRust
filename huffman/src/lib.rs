use std::cmp::{Ordering, Reverse};
use std::collections::{HashMap, BinaryHeap};
use std::option::Option;
use bytebuffer::ByteBuffer;


#[derive(PartialEq, Eq, PartialOrd, Debug)]
pub struct Node {
    pub freq: u32,
    pub c: Option<char>,
    r: Option<Box<Node>>,
    l: Option<Box<Node>>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.freq.cmp(&other.freq)
    }
}

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

}

pub fn count_characters(contents: &String) -> HashMap<char, u32> {

    let mut map: HashMap<char, u32> = HashMap::new();

    for c in contents.chars() {

        if map.contains_key(&c){
            let mut num: u32 = *map.get(&c).unwrap();
            num += 1;
            map.insert(c, num);
        }else{
            map.insert(c, 1);
        }

    }

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

    println!("Heap size: {}", heap.len());
    println!("Smallest element is: {:?}", heap.peek().unwrap().0);

    while heap.len() > 1 {

        let right = heap.pop().unwrap().0; // is there a better way to do this than unwrapping???
        let left = heap.pop().unwrap().0; // and zeroing
        
        let new_node = Node::node(Some(right), Some(left));

        heap.push(Reverse(new_node));

    }

    heap

}

// TODO: This feels really messy and inelegant. Is there a better way to do this?
pub fn encode_contents(binary_string: &String, characters: Vec<char>, contents: &String, codes: HashMap<char, String>) -> ByteBuffer {

    let mut bit_buffer : BitSeq = BitSeq::new();
    let mut byte_buffer : ByteBuffer = ByteBuffer::new();

    // add tree to byte buffer
    for bit in binary_string.chars() {

        if !bit_buffer.try_add_bit(bit as u64 - 48) {

            // Add the u32 in bit_buffer to some sort of bytearray
            byte_buffer.write_u64(bit_buffer.value);

            bit_buffer = BitSeq::new();

            bit_buffer.try_add_bit(bit as u64 - 48);
        }
        
    }

    byte_buffer.write_u64(bit_buffer.value);
    bit_buffer = BitSeq::new();

    // add chars to byte buffer
    for c in characters.iter() {

        byte_buffer.write_u64(c.clone() as u64);

    }

    // for char in file
    for c in contents.chars() {

        let bits = codes.get(&c).unwrap();

        for bit in bits.chars() {

            if !bit_buffer.try_add_bit(bit as u64 - 48) {

                // Add the u32 in bit_buffer to some sort of bytearray
                byte_buffer.write_u64(bit_buffer.value);

                bit_buffer = BitSeq::new();

                bit_buffer.try_add_bit(bit as u64 - 48);
            }
        }
    }

    byte_buffer.write_u64(bit_buffer.value);
    byte_buffer

}

pub struct BitSeq {
    pub value: u64,
    pub index: u64,
}

impl BitSeq {
    pub fn new() -> BitSeq {
        BitSeq { value: 0, index: 0 }
    }

    // insert bit if space is left
    pub fn try_add_bit(&mut self, bit : u64) -> bool {
        if self.index < 64 {
            if bit == 1 {
                self.value += bit << self.index;
            }
            self.index += 1;
            return true
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use bytebuffer::ByteBuffer;

    use crate::{count_characters, get_leaves, get_heap, Node, encode_contents};

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
        assert_eq!('o', right_right.c.unwrap())

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

    }

    #[test]
    fn test_codes() {

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

    }

    #[test]
    fn test_encoding() {

        let contents = "mmmmaao".to_string();
        let map = count_characters(&contents);
        let leaves = get_leaves(map);
        let characters: Vec<char> = leaves.iter().map(|x: &Node| x.c.unwrap()).collect();
        let heap = get_heap(leaves);
        let mut codes: HashMap<char, String> = HashMap::new();
        heap.peek().unwrap().0.get_codes("".to_string(), &mut codes);
        let mut binary_string = "".to_string();
        heap.peek().unwrap().0.save_tree(&mut binary_string);



        let mut byte_buffer: ByteBuffer = encode_contents(&binary_string, characters, &contents, codes);
        let mut byte;

        // 8 (tree) + 8 * 3 (chars) + 8 (data) = 40 bytes
        assert_eq!(40, byte_buffer.len());
        
        // tree 00111 -> 28
        byte = byte_buffer.read_u64();

        assert_eq!(28, byte);

        // chars 'o', 'a', 'm'
        byte = byte_buffer.read_u64();

        assert_eq!('o' as u64, byte);

        byte = byte_buffer.read_u64();

        assert_eq!('a' as u64, byte);

        byte = byte_buffer.read_u64();

        assert_eq!('m' as u64, byte);

        // text 0000101011 -> 336
        byte = byte_buffer.read_u64();

        assert_eq!(848, byte);

    }

}