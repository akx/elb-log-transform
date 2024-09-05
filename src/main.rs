use clap::Parser;
use parse::AlbLogEntry;
use std::io::{BufRead, Write};

mod parse;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filter_request_substring: Option<String>,
}

fn main() {
    let args = Args::parse();
    let stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();
    for line in stdin.lines() {
        match line {
            Ok(line) => match AlbLogEntry::parse(&line) {
                Ok(entry) => {
                    if let Some(filter) = &args.filter_request_substring {
                        if !entry.request.contains(filter) {
                            continue;
                        }
                    }
                    serde_json::to_writer(&mut stdout, &entry).unwrap();
                    stdout.write_all(b"\n").unwrap();
                }
                Err(e) => {
                    eprintln!("Error parsing log entry: {}", e);
                }
            },
            Err(e) => {
                eprintln!("Error reading line: {}", e);
            }
        }
    }
}
