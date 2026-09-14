//! Throughput check for rustcat, not run by default since timings vary by
//! machine. Run it explicitly with:
//!
//!   cargo build --release
//!   cargo test --release --test perf_test -- --ignored --nocapture

use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

const APP_FILE: &str = "target/release/rustcat";
const NUM_LINES: usize = 500_000;
const RUNS: usize = 5;

fn mib_per_sec(mib: f64, elapsed: Duration) -> f64 {
    mib / elapsed.as_secs_f64()
}

/// Runs `program` on `path` once untimed, then `RUNS` times, and returns the
/// fastest run along with its stdout, or `None` if the program failed.
///
/// The untimed warm-up matters: the first launch of a freshly built binary
/// can be much slower than later ones (e.g. macOS scans new executables on
/// first run), which would otherwise dominate the measurement.
fn best_run(program: &str, path: &Path) -> Option<(Duration, Vec<u8>)> {
    let run = || {
        let start = Instant::now();
        let output = Command::new(program).arg(path).output().ok()?;
        let elapsed = start.elapsed();
        output.status.success().then_some((elapsed, output.stdout))
    };
    run()?;
    (0..RUNS)
        .map(|_| run())
        .collect::<Option<Vec<_>>>()?
        .into_iter()
        .min_by_key(|(elapsed, _)| *elapsed)
}

#[test]
#[ignore]
fn bench_cat_large_file() {
    let path = std::env::temp_dir().join("rustcat_perf_test.txt");
    {
        let mut f =
            fs::File::create(&path).expect("Failed to create test file.");
        for i in 0..NUM_LINES {
            writeln!(
                f,
                "Line {i}: the quick brown fox jumps over the lazy dog."
            )
            .expect("Failed to write test file.");
        }
    }
    let expected = fs::read(&path).expect("Failed to read test file.");
    let file_mib = expected.len() as f64 / (1024.0 * 1024.0);

    let (elapsed, stdout) = best_run(APP_FILE, &path).unwrap_or_else(|| {
        panic!("Failed to run {APP_FILE}. Did you run `cargo build --release`?")
    });
    assert!(stdout == expected, "rustcat output differs from the input");

    println!(
        "rustcat: {NUM_LINES} lines / {file_mib:.1} MiB in {:.3}s ({:.1} MiB/s), best of {RUNS}",
        elapsed.as_secs_f64(),
        mib_per_sec(file_mib, elapsed)
    );

    match best_run("cat", &path) {
        Some((cat_elapsed, _)) => {
            println!(
                "cat:     {NUM_LINES} lines / {file_mib:.1} MiB in {:.3}s ({:.1} MiB/s), best of {RUNS}",
                cat_elapsed.as_secs_f64(),
                mib_per_sec(file_mib, cat_elapsed)
            );
            println!(
                "rustcat takes {:.1}x as long as cat here",
                elapsed.as_secs_f64() / cat_elapsed.as_secs_f64()
            );
        }
        None => println!("(system `cat` not available for comparison)"),
    }

    fs::remove_file(&path).ok();
}
