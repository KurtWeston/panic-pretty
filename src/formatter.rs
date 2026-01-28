use crate::parser::{StackTrace, TraceFormat};
use colored::*;
use std::collections::HashSet;
use std::fs;

pub struct Formatter {
    use_color: bool,
    filter_stdlib: bool,
    context_lines: usize,
}

impl Formatter {
    pub fn new(use_color: bool, filter_stdlib: bool, context_lines: usize) -> Self {
        if !use_color {
            colored::control::set_override(false);
        }
        Self { use_color, filter_stdlib, context_lines }
    }

    pub fn format(&self, trace: &StackTrace) {
        let format_name = match trace.format {
            TraceFormat::Rust => "Rust",
            TraceFormat::Go => "Go",
            TraceFormat::NodeJs => "Node.js",
        };

        println!("\n{} {}", "Stack Trace Format:".bold(), format_name.cyan().bold());
        println!("{} {}", "Message:".bold(), trace.message.red());
        println!();

        let frames: Vec<_> = if self.filter_stdlib {
            trace.frames.iter().filter(|f| !f.is_stdlib).collect()
        } else {
            trace.frames.iter().collect()
        };

        let mut unique_files = HashSet::new();
        for (i, frame) in frames.iter().enumerate() {
            if let Some(ref file) = frame.file {
                unique_files.insert(file.clone());
            }

            let frame_num = format!("#{}", i).bright_black();
            let function = frame.function.yellow().bold();
            
            print!("  {} {}", frame_num, function);

            if let Some(ref file) = frame.file {
                let location = if let Some(line) = frame.line {
                    if let Some(col) = frame.column {
                        format!(" at {}:{}:{}", file.green(), line.to_string().blue(), col.to_string().blue())
                    } else {
                        format!(" at {}:{}", file.green(), line.to_string().blue())
                    }
                } else {
                    format!(" at {}", file.green())
                };
                println!("{}", location);

                if self.context_lines > 0 && frame.line.is_some() {
                    self.show_context(file, frame.line.unwrap());
                }
            } else {
                println!();
            }
        }

        println!("\n{}", "Summary:".bold());
        println!("  Total frames: {}", trace.frames.len().to_string().cyan());
        if self.filter_stdlib {
            let filtered = trace.frames.len() - frames.len();
            println!("  Filtered frames: {}", filtered.to_string().yellow());
        }
        println!("  Unique files: {}", unique_files.len().to_string().green());
    }

    fn show_context(&self, file: &str, line: usize) {
        if let Ok(content) = fs::read_to_string(file) {
            let lines: Vec<&str> = content.lines().collect();
            let start = line.saturating_sub(self.context_lines + 1);
            let end = (line + self.context_lines).min(lines.len());

            for (i, content) in lines.iter().enumerate().take(end).skip(start) {
                let line_num = i + 1;
                if line_num == line {
                    println!("    {} {}", format!("{:>4}", line_num).red().bold(), content.white().bold());
                } else {
                    println!("    {} {}", format!("{:>4}", line_num).bright_black(), content.bright_black());
                }
            }
        }
    }
}
