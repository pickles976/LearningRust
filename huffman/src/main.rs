use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

#[derive(PartialEq, Eq, PartialOrd, Debug)]
pub struct Pair {
    c: char,
    freq: u32,
}

impl Ord for Pair {
    fn cmp(&self, other: &Pair) -> Ordering {
        (self.freq).cmp(&other.freq)
    }
}

impl Pair {
    fn new(c: char, freq: u32) -> Pair {
        Pair {
            c,
            freq,
        }
    }
}

fn main() {

    let mut map: HashMap<char, u32> = HashMap::new();

    let contents = fs::read_to_string("book.txt").expect("File not found!");

    let mut total : u32 = 0;

    // 1. Read characters into map
    for line in contents.lines() {

        let chars: Vec<char> = line.chars().collect();

        for c in chars {

            total += 1;

            if map.contains_key(&c){
                let mut num: u32 = *map.get(&c).unwrap();
                num += 1;
                map.insert(c, num);
            }else{
                map.insert(c, 1);
            }

        }
    }

    println!("Total characters in source text: {}", total);

    let mut pairs = Vec::new();

    // 2. Sort HashMap entries by frequency
    for k in map.keys() {
        pairs.push(Pair::new(k.clone(), map.get(k).unwrap().clone()));
    }

    pairs.sort();

    println!("{:?}", pairs);


}
