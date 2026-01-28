use panic_pretty::parser::{StackTrace, TraceFormat};

#[test]
fn test_parse_rust_panic() {
    let input = r#"thread 'main' panicked at 'assertion failed', src/main.rs:10:5
stack backtrace:
   at rust_panic::main (src/main.rs:10:5)
   at core::ops::function::FnOnce::call_once (core/ops/function.rs:250:5)"#;

    let trace = StackTrace::parse(input).expect("Failed to parse Rust trace");
    assert_eq!(trace.format, TraceFormat::Rust);
    assert!(!trace.frames.is_empty());
    assert_eq!(trace.frames[0].function, "rust_panic::main");
    assert_eq!(trace.frames[0].file, Some("src/main.rs".to_string()));
    assert_eq!(trace.frames[0].line, Some(10));
}

#[test]
fn test_parse_go_panic() {
    let input = r#"panic: runtime error
goroutine 1 [running]:
main.doSomething()
	/home/user/project/main.go:15
main.main()
	/home/user/project/main.go:10"#;

    let trace = StackTrace::parse(input).expect("Failed to parse Go trace");
    assert_eq!(trace.format, TraceFormat::Go);
    assert!(!trace.frames.is_empty());
    assert_eq!(trace.frames[0].function, "main.doSomething");
}

#[test]
fn test_parse_nodejs_error() {
    let input = r#"Error: Something went wrong
    at Object.<anonymous> (/home/user/app.js:10:15)
    at Module._compile (node:internal/modules/cjs/loader:1159:14)"#;

    let trace = StackTrace::parse(input).expect("Failed to parse Node.js trace");
    assert_eq!(trace.format, TraceFormat::NodeJs);
    assert!(!trace.frames.is_empty());
    assert_eq!(trace.frames[0].function, "Object.<anonymous>");
    assert_eq!(trace.frames[0].line, Some(10));
}

#[test]
fn test_parse_multiple_traces() {
    let input = r#"Error: First error
    at test (/app.js:5:10)

thread 'main' panicked at 'second error', main.rs:20:5
   at main (main.rs:20:5)"#;

    let traces = StackTrace::parse_multiple(input);
    assert_eq!(traces.len(), 2);
    assert_eq!(traces[0].format, TraceFormat::NodeJs);
    assert_eq!(traces[1].format, TraceFormat::Rust);
}

#[test]
fn test_stdlib_detection() {
    let rust_input = r#"thread 'main' panicked
   at my_func (src/lib.rs:10:5)
   at core::ops::function::FnOnce::call_once (core/ops/function.rs:250:5)"#;

    let trace = StackTrace::parse(rust_input).unwrap();
    assert!(!trace.frames[0].is_stdlib);
    assert!(trace.frames[1].is_stdlib);
}
