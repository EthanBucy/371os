use std::env;
use std::fs;
use std::io::{self, Read};
use std::process;

#[derive(Clone, Copy, Debug, Default)]
struct Counts {
    lines: usize,
    words: usize,
    chars: usize,
    bytes: usize,
    max_line_length: usize,
}

#[derive(Debug, Default)]
struct Options {
    show_lines: bool,
    show_words: bool,
    show_chars: bool,
    show_bytes: bool,
    show_max_line_length: bool,
    files: Vec<String>,
    files0_from: Option<String>,
}

fn print_help(program: &str) {
    println!("Usage: wc [OPTION]... [FILE]...");
    println!("Count lines, words, characters, bytes, and max line length.");
    println!();
    println!("  -c, --bytes            print the byte counts");
    println!("  -m, --chars            print the character counts");
    println!("  -l, --lines            print the newline counts");
    println!("  -w, --words            print the word counts");
    println!("  -L, --max-line-length  print the maximum line length");
    println!("      --files0-from=FILE read input file names from FILE (NUL-separated)");
    println!("      --help             display this help and exit");
    println!("      --version          output version information and exit");
    println!();
    println!("Example: {} -cl src/main.rs", program);
}

fn print_version() {
    println!("my_wc 0.1.0");
}

fn parse_options() -> Result<Options, String> {
    let mut options = Options::default();
    let mut saw_count_flag = false;
    let mut stop_parsing_flags = false;
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        if !stop_parsing_flags {
            if arg == "--" {
                stop_parsing_flags = true;
                continue;
            }

            if arg == "--help" {
                let program = env::args().next().unwrap_or_else(|| "my_wc".to_string());
                print_help(&program);
                process::exit(0);
            }

            if arg == "--version" {
                print_version();
                process::exit(0);
            }

            if let Some(value) = arg.strip_prefix("--files0-from=") {
                options.files0_from = Some(value.to_string());
                continue;
            }

            if arg == "--files0-from" {
                let value = args.next().ok_or_else(|| {
                    "my_wc: option '--files0-from' requires an argument".to_string()
                })?;
                options.files0_from = Some(value);
                continue;
            }

            if arg == "--bytes" {
                options.show_bytes = true;
                saw_count_flag = true;
                continue;
            }

            if arg == "--chars" {
                options.show_chars = true;
                saw_count_flag = true;
                continue;
            }

            if arg == "--lines" {
                options.show_lines = true;
                saw_count_flag = true;
                continue;
            }

            if arg == "--words" {
                options.show_words = true;
                saw_count_flag = true;
                continue;
            }

            if arg == "--max-line-length" {
                options.show_max_line_length = true;
                saw_count_flag = true;
                continue;
            }

            if arg.starts_with("--") {
                return Err(format!("my_wc: unrecognized option '{}'", arg));
            }

            if arg.starts_with('-') && arg.len() > 1 {
                for flag in arg[1..].chars() {
                    match flag {
                        'c' => {
                            options.show_bytes = true;
                            saw_count_flag = true;
                        }
                        'm' => {
                            options.show_chars = true;
                            saw_count_flag = true;
                        }
                        'l' => {
                            options.show_lines = true;
                            saw_count_flag = true;
                        }
                        'w' => {
                            options.show_words = true;
                            saw_count_flag = true;
                        }
                        'L' => {
                            options.show_max_line_length = true;
                            saw_count_flag = true;
                        }
                        _ => return Err(format!("my_wc: invalid option -- '{}'", flag)),
                    }
                }
                continue;
            }
        }

        options.files.push(arg);
    }

    if !saw_count_flag {
        options.show_lines = true;
        options.show_words = true;
        options.show_bytes = true;
    }

    Ok(options)
}

fn read_stdin_all() -> Result<Vec<u8>, String> {
    let mut buffer = Vec::new();
    io::stdin()
        .read_to_end(&mut buffer)
        .map_err(|err| format!("my_wc: failed to read stdin: {}", err))?;
    Ok(buffer)
}

fn parse_files0_entries(raw: &[u8]) -> Vec<String> {
    raw.split(|&b| b == b'\0')
        .filter(|entry| !entry.is_empty())
        .map(|entry| String::from_utf8_lossy(entry).to_string())
        .collect()
}

fn compute_counts(bytes: &[u8]) -> Counts {
    let lines = bytes.iter().filter(|&&b| b == b'\n').count();
    let text = String::from_utf8_lossy(bytes);
    let words = text.split_whitespace().count();
    let chars = text.chars().count();
    let max_line_length = text
        .split('\n')
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);

    Counts {
        lines,
        words,
        chars,
        bytes: bytes.len(),
        max_line_length,
    }
}

fn add_counts(total: &mut Counts, counts: Counts) {
    total.lines += counts.lines;
    total.words += counts.words;
    total.chars += counts.chars;
    total.bytes += counts.bytes;
    total.max_line_length = total.max_line_length.max(counts.max_line_length);
}

fn print_counts(counts: Counts, options: &Options, label: Option<&str>) {
    if options.show_lines {
        print!("{:>8}", counts.lines);
    }
    if options.show_words {
        print!("{:>8}", counts.words);
    }
    if options.show_chars {
        print!("{:>8}", counts.chars);
    }
    if options.show_bytes {
        print!("{:>8}", counts.bytes);
    }
    if options.show_max_line_length {
        print!("{:>8}", counts.max_line_length);
    }

    if let Some(name) = label {
        print!(" {}", name);
    }

    println!();
}

fn main() {
    let mut options = match parse_options() {
        Ok(opts) => opts,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    let mut stdin_from_files0: Option<Vec<u8>> = None;
    let mut stdin_exhausted_for_data = false;

    if let Some(source) = options.files0_from.clone() {
        let entries_bytes = if source == "-" {
            stdin_exhausted_for_data = true;
            let data = match read_stdin_all() {
                Ok(data) => data,
                Err(err) => {
                    eprintln!("{}", err);
                    process::exit(1);
                }
            };
            stdin_from_files0 = Some(data.clone());
            data
        } else {
            match fs::read(&source) {
                Ok(data) => data,
                Err(err) => {
                    eprintln!("my_wc: {}: {}", source, err);
                    process::exit(1);
                }
            }
        };

        let mut entries = parse_files0_entries(&entries_bytes);
        options.files.append(&mut entries);
    }

    let mut total = Counts::default();
    let mut processed_count = 0usize;

    if options.files.is_empty() {
        let stdin_bytes = match stdin_from_files0.take() {
            Some(data) => data,
            None => match read_stdin_all() {
                Ok(data) => data,
                Err(err) => {
                    eprintln!("{}", err);
                    process::exit(1);
                }
            },
        };
        let counts = compute_counts(&stdin_bytes);
        print_counts(counts, &options, None);
        return;
    }

    let mut stdin_cache: Option<Vec<u8>> = None;

    for path in &options.files {
        let bytes = if path == "-" {
            if stdin_exhausted_for_data {
                Vec::new()
            } else {
                if stdin_cache.is_none() {
                    let read = match read_stdin_all() {
                        Ok(data) => data,
                        Err(err) => {
                            eprintln!("{}", err);
                            process::exit(1);
                        }
                    };
                    stdin_cache = Some(read);
                }
                stdin_exhausted_for_data = true;
                stdin_cache.take().unwrap_or_default()
            }
        } else {
            match fs::read(path) {
                Ok(data) => data,
                Err(err) => {
                    eprintln!("my_wc: {}: {}", path, err);
                    process::exit(1);
                }
            }
        };

        let counts = compute_counts(&bytes);
        print_counts(counts, &options, Some(path));
        add_counts(&mut total, counts);
        processed_count += 1;
    }

    if processed_count > 1 {
        print_counts(total, &options, Some("total"));
    }
}
