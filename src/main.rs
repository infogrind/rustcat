use anyhow::{Context, Error};
use either::Either;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
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
            // `.lines()` is the main remaining source of latency vs. `cat`:
            // it allocates a new heap `String` per line and validates every
            // line's bytes as UTF-8, work `cat` never does because it just
            // copies raw bytes without interpreting them. Replacing this
            // with a raw `std::io::copy` from a `BufReader<File>` straight
            // into stdout would remove both costs, at the price of some
            // behavior: invalid-UTF-8/binary input would pass through
            // instead of erroring, and concatenating a file with no
            // trailing newline into the next would merge their first/last
            // lines like real `cat` does, instead of always separating them.
            Ok(file) => Either::Left(
                BufReader::new(file).lines().map(|i| i.map_err(Error::from)),
            ),
            Err(e) => Either::Right(std::iter::once(Err(e))),
        }
    })
}

/// Returns an iterator over the lines of stdin.
fn lines_from_stdin() -> impl Iterator<Item = Result<String, Error>> {
    // Same per-line allocation + UTF-8 validation cost as in
    // `lines_from_files` above.
    io::stdin()
        .lock()
        .lines()
        .map(|l| l.context("Error reading line"))
}

/// Prints the lines from the given iterator to stdout, or an error.
///
/// `Stdout` is line-buffered internally, so printing line by line via
/// `println!` flushes on every line. Locking it once and wrapping it in a
/// `BufWriter` batches those writes into a handful of larger syscalls.
fn cat<I: IntoIterator<Item = Result<String, Error>>>(it: I) {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    it.into_iter()
        .try_for_each(|line| -> Result<(), Error> {
            writeln!(out, "{}", line?)?;
            Ok(())
        })
        .and_then(|_| out.flush().map_err(Error::from))
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
