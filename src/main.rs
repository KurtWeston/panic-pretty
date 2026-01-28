use anyhow::Result;
use clap::Parser;
use std::fs;
use std::io::{self, Read};

mod formatter;
mod parser;

use formatter::Formatter;
use parser::StackTrace;

#[derive(Parser)]
#[command(name = "panic-pretty")]
#[command(about = "Transform ugly panic stack traces into beautiful, readable output")]
struct Cli {
    #[arg(help = "Input file path (reads from stdin if not provided)")]
    file: Option<String>,

    #[arg(short, long, help = "Filter out standard library and vendor frames")]
    filter: bool,

    #[arg(short, long, default_value = "0", help = "Number of source context lines to show")]
    context: usize,

    #[arg(long, help = "Disable colored output")]
    no_color: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let input = match &cli.file {
        Some(path) => fs::read_to_string(path)?,
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            buffer
        }
    };

    let traces = StackTrace::parse_multiple(&input);
    
    if traces.is_empty() {
        println!("No stack traces detected. Original output:");
        println!("{}", input);
        return Ok(());
    }

    let formatter = Formatter::new(!cli.no_color, cli.filter, cli.context);
    
    for (i, trace) in traces.iter().enumerate() {
        if i > 0 {
            println!("\n{}", "=".repeat(80));
        }
        formatter.format(trace);
    }

    Ok(())
}
