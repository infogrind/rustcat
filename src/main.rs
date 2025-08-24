use either::Either;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// Returns an iterator over the lines of all given files, in order.
fn lines_from_files<I, P>(files: I) -> impl Iterator<Item = io::Result<String>>
where
    I: IntoIterator<Item = P>,
    P: AsRef<Path>,
{
    files.into_iter().flat_map(|path| {
        let file = File::open(&path);
        match file {
            Ok(file) => Either::Left(BufReader::new(file).lines()),
            Err(e) => Either::Right(std::iter::once(Err(e))),
        }
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
