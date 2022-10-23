use std::cmp::{Ordering, Reverse};
use std::collections::{HashMap, BinaryHeap, binary_heap};
use std::fs;
use std::option::Option;

#[derive(PartialEq, Eq, PartialOrd, Debug)]
pub struct Node {
    freq: u32,
    c: Option<char>,
    r: Option<Box<Node>>,
    l: Option<Box<Node>>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.freq.cmp(&other.freq)
    }
}

impl Node {

    fn leaf(c: char, freq: u32) -> Node {
        Node {                         
            freq,
            c: Some(c),
            l: None,
            r: None,
        }
    }

    fn node (right: Option<Node>, left: Option<Node>) -> Node {

        let right = right.unwrap();
        let left = left.unwrap();

        Node{
            freq: right.freq + left.freq,
            c: None,
            l: Some(Box::new(left)),
            r: Some(Box::new(right)),
        }
    }

    // traverse all nodes and save char -> code mappings
    fn traverse(&self, s: String, map: &mut HashMap<char, String>){

        // go right
        if let Some(node) = &self.r {
            let mut temp = s.clone();
            temp.push('1');
            node.traverse(temp, map)
        }

        // go left
        if let Some(node) = &self.l {
            let mut temp = s.clone();
            temp.push('0');
            node.traverse(temp, map)
        }

        if let Some(c) = self.c {
            map.insert(c, s);
        }

    }

}

fn count_characters(filename: String) -> HashMap<char, u32> {

    let mut map: HashMap<char, u32> = HashMap::new();

    let contents = fs::read_to_string(filename).expect("File not found!");

    for line in contents.lines() {

        let chars: Vec<char> = line.chars().collect();

        for c in chars {

            if map.contains_key(&c){
                let mut num: u32 = *map.get(&c).unwrap();
                num += 1;
                map.insert(c, num);
            }else{
                map.insert(c, 1);
            }

        }
    }

    map

}

fn get_leaves(map: HashMap<char, u32>) -> Vec<Node> {

    let mut leaves = Vec::new();
    
    for k in map.keys() {
        leaves.push(Node::leaf(k.clone(), map.get(k).unwrap().clone()));
    }

    leaves.sort();

    leaves
}

fn get_heap(leaves: Vec<Node>) -> BinaryHeap<Reverse<Node>> {

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

fn main() {

    // 1. Read characters into map
    let map = count_characters("book.txt".to_string());

    println!("Number of unique characters: {}", map.len());

    // 2. Sort HashMap entries by frequency
    let leaves = get_leaves(map);

    // println!("{:?}", leaves);

    // 3. build heap/tree
    let heap = get_heap(leaves);

    // let root = &heap.peek().unwrap().0;

    // println!("{:?}", heap.peek());

    let mut codes: HashMap<char, String> = HashMap::new();

    heap.peek().unwrap().0.traverse("".to_string(), &mut codes);

    println!("Huffman codes: {:?}", codes);

}
