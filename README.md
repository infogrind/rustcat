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

- Iterators
- `AsRef`
- `Either` for `match` expressions whose branches return different types
- Passing everything as a `Result` type until the point of printing the lines
  to `stdout`.

## Performance

`rustcat` locks stdout once and writes through a `BufWriter`, so its
throughput on large files is now in the same ballpark as the system `cat`.

To check throughput on your own machine, run the perf test (it's marked
`#[ignore]` since timings vary by machine, so it's excluded from the normal
`cargo test` run):

```shell
cargo build --release
cargo test --release --test perf_test -- --ignored --nocapture
```
