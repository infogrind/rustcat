//! Throughput check for rustcat, not run by default since timings vary by
//! machine. Run it explicitly with:
//!
//!   cargo build --release
//!   cargo test --release --test perf_test -- --ignored --nocapture

use std::fs;
use std::io::Write;
use std::process::Command;
use std::time::{Duration, Instant};

const APP_FILE: &str = "target/release/rustcat";
const NUM_LINES: usize = 500_000;

fn mib_per_sec(mib: f64, elapsed: Duration) -> f64 {
    mib / elapsed.as_secs_f64()
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
    let file_mib =
        fs::metadata(&path).unwrap().len() as f64 / (1024.0 * 1024.0);

    let start = Instant::now();
    let output = Command::new(APP_FILE).arg(&path).output().unwrap_or_else(|e| {
        panic!("Failed to run {APP_FILE} ({e}). Did you run `cargo build --release`?")
    });
    let elapsed = start.elapsed();
    assert!(output.status.success(), "rustcat exited with an error");
    let line_count = output.stdout.iter().filter(|&&b| b == b'\n').count();
    assert_eq!(line_count, NUM_LINES, "rustcat did not print all lines");

    println!(
        "rustcat: {NUM_LINES} lines / {file_mib:.1} MiB in {:.3}s ({:.1} MiB/s)",
        elapsed.as_secs_f64(),
        mib_per_sec(file_mib, elapsed)
    );

    let cat_start = Instant::now();
    match Command::new("cat").arg(&path).output() {
        Ok(cat_output) if cat_output.status.success() => {
            let cat_elapsed = cat_start.elapsed();
            println!(
                "cat:     {NUM_LINES} lines / {file_mib:.1} MiB in {:.3}s ({:.1} MiB/s)",
                cat_elapsed.as_secs_f64(),
                mib_per_sec(file_mib, cat_elapsed)
            );
            println!(
                "rustcat is {:.1}x slower than cat here",
                elapsed.as_secs_f64() / cat_elapsed.as_secs_f64()
            );
        }
        _ => println!("(system `cat` not available for comparison)"),
    }

    fs::remove_file(&path).ok();
}
