use googletest::prelude::*;
use std::{
    io::Write,
    process::{Command, Stdio},
};

const APP_FILE: &str = "target/debug/rustcat";

#[gtest]
fn test_prints_stdin() {
    let mut child = Command::new(APP_FILE)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn app process.");

    // Write to stdin
    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin.");
        writeln!(stdin, "Hello, world!").expect("Failed to write to stdin.");
        writeln!(stdin, "Hello, furz!").expect("Failed to write to stdin.");
        writeln!(stdin, "😂").expect("Failed to write to stdin.");

        // stdin closes here, sending EOD.
    }

    let output = child.wait_with_output().expect("Failed to read stdout.");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    expect_that!(lines, container_eq(["Hello, world!", "Hello, furz!", "😂"]));
}

#[gtest]
fn test_prints_from_file_input() {
    let output = Command::new(APP_FILE)
        .arg("testdata/foo.txt")
        .arg("testdata/bar.txt")
        .output()
        .expect("Error running application.");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    expect_that!(
        lines,
        container_eq(["Hello, world", "How are you?", "Bongo", "This is bar"])
    );
}

#[gtest]
fn test_file_not_found_error_contains_filename() {
    let output = Command::new(APP_FILE)
        .arg("invalid_file.txt")
        .output()
        .expect("Could not run app.");

    let stderr = String::from_utf8_lossy(&output.stderr);
    expect_that!(stderr, contains_substring("invalid_file.txt"))
}

/// Writes `contents` to a fresh file in the temp dir and returns its path.
fn temp_file(name: &str, contents: &[u8]) -> std::path::PathBuf {
    let path = std::env::temp_dir()
        .join(format!("rustcat_test_{}_{name}", std::process::id()));
    std::fs::write(&path, contents).expect("Failed to write temp file.");
    path
}

#[gtest]
fn test_copies_invalid_utf8_bytes_unchanged() {
    let contents: &[u8] = b"binary \xff\xfe\x00 data\r\nmore\n";
    let path = temp_file("invalid_utf8.bin", contents);

    let output = Command::new(APP_FILE)
        .arg(&path)
        .output()
        .expect("Error running application.");
    std::fs::remove_file(&path).ok();

    expect_that!(output.stdout, eq(contents));
    expect_that!(output.stderr, is_empty());
}

#[gtest]
fn test_does_not_add_newline_between_files() {
    let first = temp_file("no_newline_1.txt", b"no trailing newline");
    let second = temp_file("no_newline_2.txt", b" continues here\n");

    let output = Command::new(APP_FILE)
        .arg(&first)
        .arg(&second)
        .output()
        .expect("Error running application.");
    std::fs::remove_file(&first).ok();
    std::fs::remove_file(&second).ok();

    expect_that!(
        String::from_utf8_lossy(&output.stdout),
        eq("no trailing newline continues here\n")
    );
}

#[gtest]
fn test_copies_stdin_bytes_unchanged() {
    let contents: &[u8] = b"line\r\n\xff\xfeend without newline";
    let mut child = Command::new(APP_FILE)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn app process.");
    child
        .stdin
        .take()
        .expect("Failed to open stdin.")
        .write_all(contents)
        .expect("Failed to write to stdin.");

    let output = child.wait_with_output().expect("Failed to read stdout.");
    expect_that!(output.stdout, eq(contents));
}
