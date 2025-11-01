use std::{env, process};

fn main() {
    let proc: Vec<String> = env::args().collect();
    validate_args(&proc);

    println!("{:?}", proc)
}

fn validate_args(args: &Vec<String>) {
    if args.len() < 2 {
        println!("No arguments provided");
        process::exit(0)
    }
}

// fn display(args: &Vec<String>) {
//     let command = args.get(1);
// }
