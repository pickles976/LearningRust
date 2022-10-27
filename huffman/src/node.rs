use std::cmp::Ordering;
use std::option::Option;
use bytebuffer::ByteBuffer;
use fxhash::FxHashMap;

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
    pub fn get_codes(&self, s: String, map: &mut FxHashMap<char, String>){

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
        if let Some(node) = self.r {
            self.r = Some(Box::new(node.populate_tree(byte_buffer)));
        }

        // go left
        if let Some(node) = self.l {
            self.l = Some(Box::new(node.populate_tree(byte_buffer)));
        }

        if let Some(_c) = self.c {
            self.c = char::from_u32(byte_buffer.read_u32());
        }

        self

    }

    // use bytearray to navigate tree until we reach a leaf
    pub fn decode_bytearray(&self, output: &mut String, byte_buffer: &mut ByteBuffer) {

        match self.c {
            Some(c) => {
                output.push(c);
            },
            None => {

                let bit = byte_buffer.read_bit();
    
                if bit { // build leaf
                    if let Some(right) = &self.r {
                        right.decode_bytearray(output, byte_buffer);
                    }
                } else {
                    if let Some(left) = &self.l {
                        left.decode_bytearray(output, byte_buffer);
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