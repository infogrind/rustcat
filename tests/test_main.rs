use googletest::prelude::*;
use std::process::Command;

const APP_FILE: &str = "target/debug/rustcat";

#[gtest]
fn test_not_implemented() {
    let output = Command::new(APP_FILE)
        .output()
        .expect("Error running command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    expect_that!(stdout, contains_substring("not implemented"));
}

#[gtest]
fn test_error_with_args() {
    let output = Command::new("target/debug/rustcat")
        .arg("foo")
        .output()
        .expect("Error running command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    expect_that!(stdout, eq(""));

    let stderr = String::from_utf8_lossy(&output.stderr);
    expect_that!(
        stderr,
        contains_substring("rustcat only supports reading from stdin")
    );
}
