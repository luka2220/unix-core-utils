use clap::{ArgAction, Parser};
use std::{path, process};

/// wc cli command
#[derive(Parser, Debug)]
struct Args {
    /// file path
    #[arg(value_name = "FILE")]
    file_paths: Vec<path::PathBuf>,

    /// print the bytes count
    #[arg(short = 'c', long = "bytes", action = ArgAction::SetTrue, conflicts_with = "chars")]
    bytes: bool,

    /// print the chars count 
    #[arg(short = 'm', long = "chars", action = ArgAction::SetTrue, conflicts_with = "bytes")]
    chars: bool,
}

/// Flag types from the command option
enum FlagKind {
    Bytes,
    Chars,
    Default
}

fn main() {
    let args = Args::parse();

    if args.file_paths.len() < 1 {
        println!("No file provided");
        process::exit(0)
    }

    let count_mode: FlagKind;

    if args.bytes {
        count_mode = FlagKind::Bytes;
    } else if args.chars {
        count_mode = FlagKind::Chars;
    } else {
        count_mode = FlagKind::Default
    }

    match count_mode {
        FlagKind::Bytes => process_bytes(&args.file_paths),
        FlagKind::Chars => println!("process wc chars"),
        FlagKind::Default => println!("no flags provided"),
    }
}

/// Gets the byte count of all passed in files
fn process_bytes(files: &Vec<path::PathBuf>) {
    for item in files.iter() {
        println!("{:?}", item.to_string_lossy())
    }
}