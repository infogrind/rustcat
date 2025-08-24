use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Returns an iterator over the lines of all given files, in order.
fn lines_from_files<I: IntoIterator<Item = String>>(
    files: I,
) -> impl Iterator<Item = io::Result<String>> {
    files.into_iter().flat_map(|path| {
        let file = File::open(&path);
        // Here Box<dyn ...> is needed because the two branches of the match return different
        // underlying types.
        let reader: Box<dyn Iterator<Item = io::Result<String>>> = match file {
            Ok(file) => Box::new(BufReader::new(file).lines()),
            Err(e) => Box::new(std::iter::once(Err(e))),
        };
        reader
    })
}

/// Returns an iterator over the lines of stdin.
fn lines_from_stdin() -> impl Iterator<Item = io::Result<String>> {
    io::stdin().lock().lines()
}

/// Prints the lines from the given iterator to stdout, or an error.
fn cat<I: IntoIterator<Item = io::Result<String>>>(it: I) {
    for line in it {
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

fn main() {
    let file_args: Vec<String> = env::args().skip(1).collect();
    if !file_args.is_empty() {
        cat(lines_from_files(file_args));
    } else {
        cat(lines_from_stdin());
    }
}
