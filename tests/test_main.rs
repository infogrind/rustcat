use std::process::Command;
use assertables::assert_contains;

const APP_FILE: &str = "target/debug/rustcat";

#[test]
fn test_not_implemented() {
    let output = Command::new(APP_FILE)
        .output().expect("Error running command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_contains!(stdout, "not implemented");
}

#[test]
fn test_error_with_args() {
    let output = Command::new("target/debug/rustcat")
        .arg("foo")
        .output()
        .expect("Error running command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.is_empty());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_contains!(stderr, "rustcat only supports reading from stdin");
}
