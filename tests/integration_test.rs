use std::process::Command;
use std::fs;
use std::io::Write;

#[test]
fn test_cli_stdin_input() {
    let input = r#"thread 'main' panicked at 'test'
   at main (src/main.rs:10:5)"#;
    
    let output = Command::new("cargo")
        .args(["run", "--"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
            child.wait_with_output()
        });
    
    assert!(output.is_ok());
    let out = output.unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Stack Trace Format"));
    assert!(stdout.contains("Rust"));
}

#[test]
fn test_cli_file_input() {
    let temp_file = "test_trace.txt";
    let input = r#"Error: test error
    at testFunc (/app/test.js:5:10)"#;
    
    fs::write(temp_file, input).unwrap();
    
    let output = Command::new("cargo")
        .args(["run", "--", temp_file])
        .output();
    
    fs::remove_file(temp_file).ok();
    
    assert!(output.is_ok());
    let stdout = String::from_utf8_lossy(&output.unwrap().stdout);
    assert!(stdout.contains("Node.js"));
}

#[test]
fn test_cli_filter_flag() {
    let input = r#"thread 'main' panicked
   at std::panic::panic_any (std/panic.rs:10:5)
   at my_code src/main.rs:20:10"#;
    
    let output = Command::new("cargo")
        .args(["run", "--", "--filter"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
            child.wait_with_output()
        });
    
    assert!(output.is_ok());
    let stdout = String::from_utf8_lossy(&output.unwrap().stdout);
    assert!(stdout.contains("Filtered frames"));
}

#[test]
fn test_cli_no_color_flag() {
    let input = r#"thread 'main' panicked at 'test'
   at main (src/main.rs:10:5)"#;
    
    let output = Command::new("cargo")
        .args(["run", "--", "--no-color"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
            child.wait_with_output()
        });
    
    assert!(output.is_ok());
    let stdout = String::from_utf8_lossy(&output.unwrap().stdout);
    assert!(!stdout.contains("\x1b["));
}

#[test]
fn test_cli_invalid_input() {
    let input = "Not a stack trace";
    
    let output = Command::new("cargo")
        .args(["run", "--"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
            child.wait_with_output()
        });
    
    assert!(output.is_ok());
    let stdout = String::from_utf8_lossy(&output.unwrap().stdout);
    assert!(stdout.contains("No stack traces detected"));
}

#[test]
fn test_cli_nonexistent_file() {
    let output = Command::new("cargo")
        .args(["run", "--", "nonexistent_file.txt"])
        .output();
    
    assert!(output.is_ok());
    assert!(!output.unwrap().status.success());
}
