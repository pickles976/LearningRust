use std::sync::{Arc, Mutex};
use std::thread;
use rand::Rng;
use rayon::prelude::*;

fn main() {

    fn fitness(val: i32) -> String {

        let num = rand::thread_rng().gen_range(0..100);

        

        val.to_string()
    }

    let stuff = vec![0, 1, 2, 3, 4, 5];

    let words: Vec<String> = stuff.par_iter().map(|&i| fitness(i)).collect();

    println!("{:?}", words);
    // println!("{:?}", words.lock().unwrap());
}
