use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!(
            "usage: {} <file>",
            args.first().map_or("my_wc", String::as_str)
        );
        process::exit(1);
    }

    let path = &args[1];
    let bytes = match fs::read(path) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("my_wc: {}: {}", path, err);
            process::exit(1);
        }
    };

    let lines = bytes.iter().filter(|&&b| b == b'\n').count();
    let text = String::from_utf8_lossy(&bytes);
    let words = text.split_whitespace().count();
    let byte_count = bytes.len();

    println!("{:>8} {:>8} {:>8} {}", lines, words, byte_count, path);
}
