# rustcat - Basic `cat` implementation in Rust

Don't expect much from this program. Use
[`bat`](https://github.com/sharkdp/bat) for a better `cat` replacement.

From standard input:

```shell
echo "Foo" | rustcat
```

From files:

```shell
rustcat foo.txt bar.txt
```

## Technical Details

This small program does almost nothing but already illustrates a number of Rust concepts:

- Iterators, including lazily opening files one at a time
- `AsRef`
- Generic functions over the `Read` trait, so files and stdin share one code
  path
- Passing everything as a `Result` type until the point of copying the input
  to `stdout`.

## Performance

Like the system `cat`, `rustcat` copies raw bytes straight from its input to
a locked, buffered stdout, without splitting input into lines, allocating
per line, or validating UTF-8. Its throughput on large files matches the
system `cat`.

This also means input is passed through unchanged: binary or non-UTF-8 data
is copied as-is, and no newline is added after a file that doesn't end with
one.

To check throughput on your own machine, run the perf test (it's marked
`#[ignore]` since timings vary by machine, so it's excluded from the normal
`cargo test` run):

```shell
cargo build --release
cargo test --release --test perf_test -- --ignored --nocapture
```
