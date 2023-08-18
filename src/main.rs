mod cli;
mod lang;

use cli::command_line_parser::CommandLineParser;
use lang::parser::Parser;
use std::io::{self, Write};

fn main() {
    let mut parser = CommandLineParser::new();

    parser.add_option("-f", "--file");
    parser.add_option("-v", "--verbose");
    parser.add_option("-r", "--repl");

    let args = std::env::args().collect::<Vec<String>>();
    let options = parser.parse(&args);

    if options.has("--verbose") {
        println!("Verbose mode enabled.");
    }

    if options.has("--repl") {
        repl();
    } else if options.has("--compile") {
        compile();
    }
}

fn repl() {
    loop {
        let mut input = String::new();

        print!("> ");

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let text = input.trim();

        let mut parser = Parser::new(text);
        match parser.parse() {
            Ok(tree) => println!("{}", tree),
            Err(err) => eprintln!("{}", err),
        };
    }
}

fn compile() {}
