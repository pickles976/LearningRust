use std::env;
use std::process;
use minigrep::Config;

fn main() {

    let config = Config::new(&args).unwrap_or_else(|err| { // unwrap or else unwraps by default otherwise runs code block
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) { // if let destructures run -> Result<Ok, Err> and matches to Err enum
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

}