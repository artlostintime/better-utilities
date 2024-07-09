use std::env; // For accessing command-line arguments
use std::fs; // For file system operations, including creating directories
use std::fs::File; // For creating files
use std::io::ErrorKind; // For handling specific kinds of I/O errors

fn main() {
    // Retrieve command-line arguments
    let args: Vec<String> = env::args().collect();

    // Define a custom name for the executable in the usage instructions
    let program_name = "make"; // Custom name for the executable

    // Check if enough arguments are provided
    if args.len() < 2 {
        println!("Usage: {} [-d] <name>", program_name);
        return;
    }

    // Check for the '-d' flag and act accordingly
    if args.len() > 2 && args[1] == "-d" {
        // Directory creation mode
        let dir_name = &args[2];
        match fs::create_dir_all(dir_name) {
            Ok(_) => println!("Directory '{}' created successfully", dir_name),
            Err(e) => println!("Error creating directory '{}': {}", dir_name, e),
        }
    } else {
        // File creation mode (similar to 'touch')
        let file_name = &args[1];
        match File::create(file_name) {
            Ok(_) => println!("File '{}' created successfully", file_name),
            Err(e) => match e.kind() {
                ErrorKind::AlreadyExists => println!("File '{}' already exists", file_name),
                _ => println!("Error creating file '{}': {}", file_name, e),
            },
        }
    }
}
