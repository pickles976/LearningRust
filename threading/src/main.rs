use rand::Rng;
use rayon::prelude::*;

fn main() {

    let num: usize = rand::thread_rng().gen_range(5..10);
    let thetas: Vec<i32> = vec![0; num];

    let solver = Solver::new(thetas);

    let stuff = vec![0, 1, 2, 3, 4, 5];

    let closure = solver.generate_fitness();

    let words: Vec<String> = stuff.par_iter().map(|&i| closure(i)).collect();

    println!("{:?}", words);
    // println!("{:?}", words.lock().unwrap());
}

struct Solver {
    thetas: Vec<i32>,
}

impl Solver {
    pub fn new(thetas: Vec<i32>) -> Solver {
        Solver {
            thetas: thetas
        }
    }

    pub fn generate_fitness(&self) -> Box<dyn Fn(i32) -> String + Send + Sync + 'static> {

        let thetas = self.thetas.to_vec();

        // Create closure that uses numbers
        let closure = move |val: i32| -> String {
            let sum: i32 = thetas.iter().sum();
            let new_val: i32 = val + sum;
            new_val.to_string()
        };

        Box::new(closure)

    }
}