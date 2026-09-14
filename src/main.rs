use anyhow::{Context, Error};
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

/// Size of the output buffer. `io::copy` reads straight into a `BufWriter`'s
/// spare capacity, so this is also the size of each read and write syscall.
const BUF_SIZE: usize = 128 * 1024;

/// Returns an iterator that opens each of the given files, in order.
///
/// Files are opened lazily, so nothing past the first error is touched.
fn readers_from_files<I, P>(
    files: I,
) -> impl Iterator<Item = Result<File, Error>>
where
    I: IntoIterator<Item = P>,
    P: AsRef<Path> + std::fmt::Display,
{
    files.into_iter().map(|path| {
        File::open(&path)
            .with_context(|| format!("Error opening file {}", path))
    })
}

/// Copies the bytes of each reader to stdout, or prints the first error.
///
/// Bytes are copied as-is, like `cat` does: no per-line allocation, no UTF-8
/// validation, and no newline added at the end of an input that lacks one.
fn cat<R, I>(readers: I)
where
    R: Read,
    I: IntoIterator<Item = Result<R, Error>>,
{
    let mut out = io::BufWriter::with_capacity(BUF_SIZE, io::stdout().lock());
    readers
        .into_iter()
        .try_for_each(|reader| -> Result<(), Error> {
            io::copy(&mut reader?, &mut out).context("Error copying input")?;
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
        cat(readers_from_files(file_args));
    } else {
        cat(std::iter::once(Ok(io::stdin().lock())));
    }
}
