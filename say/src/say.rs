use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }

    let mut newline = true;
    for arg in &args[1..] {
        if arg == "-n" {
            newline = false;
            continue;
        }
        print!("{} ", arg);
    }

    if newline {
        println!();
    } else {
        // Flush stdout to ensure everything is printed since we're not automatically adding a newline
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }
}
