// To address the warnings, apply the following fixes:
// 1. Remove unused imports: `Write` and `std::path::Path`.
// 2. Prefix the unused variable `file_type` in `apply_syntax_highlighting` with an underscore.
// 3. Either use `convert_encoding` function or remove it if not needed.

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::process::Command;
// Updated function with unused variable prefixed with an underscore
fn apply_syntax_highlighting(input: &str, _file_type: &str) -> String {
    input.to_string() // Return the input as is for now
}

fn prepend_line_numbers(input: &str) -> String {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| format!("{:4} {}", i + 1, line))
        .collect::<Vec<_>>()
        .join("\n")
}

fn search_and_highlight(input: &str, pattern: &str) -> String {
    input.replace(pattern, &format!("[{}]", pattern)) // Simple highlight by brackets
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut highlight = false;
    let mut line_numbers = false;
    let mut interactive_mode = false;
    let mut search_pattern = String::new();
    let mut encoding_conversion = false;
    let mut file_info = false;
    let mut separator = String::new();

    // Parse command-line arguments
    let mut files = Vec::new();
    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--highlight" | "-h" => highlight = true,
            "--line-numbers" | "-ln" => line_numbers = true,
            "--interactive" | "-i" => interactive_mode = true,
            "--encoding-conversion" | "-enc" => encoding_conversion = true,
            "--file-info" | "-info" => file_info = true,
            _ if arg.starts_with("--search=") => {
                search_pattern = arg["--search=".len()..].to_string();
            }
            _ if arg.starts_with("--separator=") => {
                separator = arg["--separator=".len()..].to_string();
            }
            _ => files.push(arg.clone()),
        }
    }
    // Launch the default code editor in interactive mode
    if interactive_mode {
        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string()); // Default to nano if EDITOR is not set
        match Command::new(editor).spawn() {
            Ok(mut child) => {
                child.wait().expect("Failed to wait on child");
            }
            Err(e) => eprintln!("Failed to launch the editor: {}", e),
        }
    }
    // Handle reading from stdin or files
    if files.is_empty() || files.contains(&"-".to_string()) {
        let stdin = io::stdin();
        let input = stdin.lock();
        process_input(
            input,
            &highlight,
            &line_numbers,
            &interactive_mode,
            &search_pattern,
            &encoding_conversion,
            &file_info,
        )?;
    } else {
        for (i, file) in files.iter().enumerate() {
            if i > 0 && !separator.is_empty() {
                println!("{}", separator);
            }
            let file = File::open(file)?;
            let reader = BufReader::new(file);
            process_input(
                reader,
                &highlight,
                &line_numbers,
                &interactive_mode,
                &search_pattern,
                &encoding_conversion,
                &file_info,
            )?;
        }
    }

    Ok(())
}

fn process_input<R: Read>(
    reader: R,
    highlight: &bool,
    line_numbers: &bool,
    _interactive_mode: &bool,
    search_pattern: &str,
    _encoding_conversion: &bool,
    _file_info: &bool,
) -> io::Result<()> {
    let reader = BufReader::new(reader);
    let mut content = String::new();

    for line in reader.lines() {
        let mut line = line?;
        if *highlight {
            line = apply_syntax_highlighting(&line, "txt"); // Assuming text files for simplicity
        }
        if !search_pattern.is_empty() {
            line = search_and_highlight(&line, search_pattern);
        }
        content.push_str(&line);
        content.push('\n');
    }

    if *line_numbers {
        content = prepend_line_numbers(&content);
    }

    print!("{}", content);

    Ok(())
}
