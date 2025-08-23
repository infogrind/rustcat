use std::env;
use std::io::{self, BufRead};
use std::process;

fn main() {
    let _args: Vec<String> = env::args().collect();
    if env::args().len() > 1 {
        eprintln!("rustcat only supports reading from stdin");
        process::exit(1);
    }
    for line in io::stdin().lock().lines() {
        match line {
            Ok(text) => {
                println!("{}", text);
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
            }
        }
    }
}
