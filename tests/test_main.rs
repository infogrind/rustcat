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
