use std::fs::File;
use std::fs;

fn main() {
    let f = File::open("Hello.txt").expect("Failed to open Hello.txt");

    println!("{:?}", f);

    let s : String = fs::read_to_string("Hello.txt").expect("Failed to read file to string");
    println!("{}", s);
}
