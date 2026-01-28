#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rust_panic() {
        let input = r#"thread 'main' panicked at 'assertion failed'
   at my_function (src/main.rs:10:5)
   at another_func src/lib.rs:20:10"#;
        
        let trace = StackTrace::parse(input).unwrap();
        assert_eq!(trace.format, TraceFormat::Rust);
        assert_eq!(trace.frames.len(), 2);
        assert_eq!(trace.frames[0].function, "my_function");
        assert_eq!(trace.frames[0].file, Some("src/main.rs".to_string()));
        assert_eq!(trace.frames[0].line, Some(10));
        assert_eq!(trace.frames[0].column, Some(5));
    }

    #[test]
    fn test_parse_go_panic() {
        let input = r#"panic: runtime error
goroutine 1 [running]:
main.doSomething()
	/home/user/main.go:15 +0x50
main.main()
	/home/user/main.go:10 +0x20"#;
        
        let trace = StackTrace::parse(input).unwrap();
        assert_eq!(trace.format, TraceFormat::Go);
        assert!(trace.frames.len() >= 1);
        assert_eq!(trace.frames[0].file, Some("/home/user/main.go".to_string()));
    }

    #[test]
    fn test_parse_nodejs_error() {
        let input = r#"Error: Something went wrong
    at myFunc (/app/index.js:42:15)
    at Object.<anonymous> (/app/server.js:10:3)"#;
        
        let trace = StackTrace::parse(input).unwrap();
        assert_eq!(trace.format, TraceFormat::NodeJs);
        assert_eq!(trace.frames.len(), 2);
        assert_eq!(trace.frames[0].function, "myFunc");
        assert_eq!(trace.frames[0].line, Some(42));
    }

    #[test]
    fn test_parse_invalid_input() {
        let input = "This is not a stack trace at all";
        assert!(StackTrace::parse(input).is_none());
    }

    #[test]
    fn test_parse_multiple_traces() {
        let input = r#"thread 'main' panicked at 'error 1'
   at func1 (src/a.rs:5:1)

panic: error 2
goroutine 1 [running]:
main.func2()
	/home/b.go:10 +0x20"#;
        
        let traces = StackTrace::parse_multiple(input);
        assert_eq!(traces.len(), 2);
        assert_eq!(traces[0].format, TraceFormat::Rust);
        assert_eq!(traces[1].format, TraceFormat::Go);
    }

    #[test]
    fn test_stdlib_detection_rust() {
        let input = r#"thread 'main' panicked
   at std::panic::panic_any (library/std/src/panic.rs:10:5)
   at my_code src/main.rs:20:10"#;
        
        let trace = StackTrace::parse(input).unwrap();
        assert!(trace.frames[0].is_stdlib);
        assert!(!trace.frames[1].is_stdlib);
    }

    #[test]
    fn test_stdlib_detection_go() {
        let input = r#"panic: error
goroutine 1:
runtime.gopanic()
	/usr/local/go/src/runtime/panic.go:10
main.myFunc()
	/home/main.go:5"#;
        
        let trace = StackTrace::parse(input).unwrap();
        assert!(trace.frames[0].is_stdlib);
        assert!(!trace.frames[1].is_stdlib);
    }

    #[test]
    fn test_empty_input() {
        let traces = StackTrace::parse_multiple("");
        assert_eq!(traces.len(), 0);
    }
}
