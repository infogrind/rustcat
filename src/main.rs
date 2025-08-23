use std::env;
use std::process;

fn main() {
    let _args: Vec<String> = env::args().collect();
    if env::args().len() > 1 {
        eprintln!("rustcat only supports reading from stdin");
        process::exit(1);
    }
    println!("not implemented yet")
}
