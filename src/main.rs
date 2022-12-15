mod cutter;
mod keywords;
mod scanner;
mod token;
mod token_type;
mod value;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("usage: rjlox [script]");
        process::exit(1);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) {
    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(msg) => {
            eprintln!("{path}: {msg}");
            process::exit(1);
        }
    };

    run(&source);
}

fn run_prompt() {
    while let Some(line) = get_line() {
        run(&line);
    }
}

fn get_line() -> Option<String> {
    let mut line = String::new();

    print!("\n> ");
    io::stdout().flush().unwrap();

    match io::stdin().read_line(&mut line) {
        Ok(0) => None,
        Ok(_) => Some(line.trim().to_owned()),
        Err(msg) => {
            eprintln!("{msg}");
            process::exit(1);
        }
    }
}

fn run(source: &str) {
    let tokens = scanner::scan(source);

    // For now, just print the tokens.
    println!("{:#?}", tokens);
}
