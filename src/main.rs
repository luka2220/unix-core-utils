use clap::{ArgAction, Parser};
use std::{
    fs::File,
    io::{self, Read},
    path, process,
};

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
    Default,
}

fn main() -> io::Result<()> {
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
        FlagKind::Bytes => process_bytes(&args.file_paths).expect("Error processing bytes"),
        FlagKind::Chars => process_chars(&args.file_paths).expect("Error processing chars"),
        FlagKind::Default => process_default(&args.file_paths).expect("Error processing default"),
    }

    Ok(())
}

/// Gets the byte count of all passed in files
fn process_bytes(files: &Vec<path::PathBuf>) -> io::Result<()> {
    let mut byte_count = 0;

    for item in files.iter() {
        let p = item.as_path();
        let mut f = File::open(p)
            .expect(format!("Unable to open file: {}", item.to_string_lossy()).as_str());

        // WARN: Not efficient reading a file into an empty vec
        // WARN: Stored on the heap so every time it grows it
        // WARN: reallocates memory. This can be expensive for
        // WARN: large files
        //
        let mut buf: Vec<u8> = Vec::new();

        // TODO: Use a preallocated buffer instead of an empty
        // TODO: initialized vector

        byte_count += f.read_to_end(&mut buf)?;
    }

    println!("Total bytes count: {}", byte_count);

    Ok(())
}

/// Gets the chars count for all files passed in
fn process_chars(_files: &Vec<path::PathBuf>) -> io::Result<()> {
    Ok(())
}

/// Gets the number of lines, words, and bytes for all files passed in
fn process_default(_files: &Vec<path::PathBuf>) -> io::Result<()> {
    Ok(())
}
