use std::io::{BufRead, IsTerminal};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(about, version, long_about=None)]
struct Args {
    string_to_copy: Option<String>,
}

#[derive(Parser, Debug)]
#[command(about, version, long_about=None)]
struct RequiredArgs {
    string_to_copy: String,
}

fn main() {
    // parse the command line arguments
    let args = Args::parse();

    let mut clipboard = clippers::Clipboard::get();

    // get the string to copy
    let string_to_copy = if let Some(string) = args.string_to_copy {
        string
    } else {
        // if ran straight from command line, show help message
        if is_stdin_tty() {
            RequiredArgs::parse();
            return;
        }

        // otherwise we got piped into, so read from stdin
        read_from_stdin()
    };

    // we don't want to copy an empty string
    if string_to_copy.is_empty() {
        return;
    }

    clipboard.write_text(string_to_copy).unwrap();
}

fn read_from_stdin() -> String {
    let mut input = String::new();
    let stdin = std::io::stdin();

    for line in stdin.lock().lines() {
        input.push_str(&line.unwrap());
        input.push_str("\n");
    }
    input.pop();
    input
}

fn is_stdin_tty() -> bool {
    std::io::stdin().is_terminal()
}

