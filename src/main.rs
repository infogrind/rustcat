use anyhow::{Context, Error};
use either::Either;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// Returns an iterator over the lines of all given files, in order.
fn lines_from_files<I, P>(
    files: I,
) -> impl Iterator<Item = Result<String, Error>>
where
    I: IntoIterator<Item = P>,
    P: AsRef<Path> + std::fmt::Display,
{
    files.into_iter().flat_map(|path| {
        let file: Result<File, Error> = File::open(&path)
            .with_context(|| format!("Error opening file {}", path));
        match file {
            Ok(file) => Either::Left(
                BufReader::new(file).lines().map(|i| i.map_err(Error::from)),
            ),
            Err(e) => Either::Right(std::iter::once(Err(e))),
        }
    })
}

/// Returns an iterator over the lines of stdin.
fn lines_from_stdin() -> impl Iterator<Item = Result<String, Error>> {
    io::stdin()
        .lock()
        .lines()
        .map(|l| l.context("Error reading line"))
}

/// Prints the lines from the given iterator to stdout, or an error.
fn cat<I: IntoIterator<Item = Result<String, Error>>>(it: I) {
    it.into_iter()
        .try_for_each(|line| {
            println!("{}", line?);
            Ok(())
        })
        .unwrap_or_else(|e: Error| {
            eprintln!("{:?}", e);
        })
}

fn main() {
    let file_args: Vec<String> = env::args().skip(1).collect();
    if !file_args.is_empty() {
        cat(lines_from_files(file_args));
    } else {
        cat(lines_from_stdin());
    }
}
