use crate::lexer::Lexer;
use std::process::exit;
use std::{env, fs, io};

mod lexer;

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();
    match args.len() {
        2 => run_file(&args[1])?,
        _ => {
            eprintln!("Usage: rcc [source-file]");
            exit(1);
        }
    }
    Ok(())
}

fn run_file(path: &str) -> io::Result<()> {
    let source = fs::read_to_string(path)?;
    let lexer = Lexer::new(source);
    for token in lexer {
        println!("{:?}", &token);
    }
    Ok(())
}
