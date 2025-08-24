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

`rustcat` is two orders of magnitude _slower_ than your standard `cat`. It has
about the same performance as `bat` (tested by catting a file of several
megabytes to `/dev/null`).
