use std::path::PathBuf;

use clap::Parser;
use sierra_parser::lexer::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CommandLineArguments {
    input_file: PathBuf,
}

fn main() {
    let args = CommandLineArguments::parse();

    let text = std::fs::read_to_string(args.input_file).expect("Could not read file");

    let mut lexer = Lexer::new(text);

    println!("{:?}", lexer.lex());
}
