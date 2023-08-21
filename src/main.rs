mod cli;
mod lang;

use cli::{command_line_parser::CommandLineParser, parsed_options::ParsedOptions};
use lang::parser::Parser;
use std::{
    fs,
    io::{self, Write},
};

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
        compile(&options);
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

fn compile(options: &ParsedOptions) {
    if !options.has("--compile") {
        ()
    } else {
        let path = options.get("--compile").unwrap();
        let result = fs::read_to_string(path);

        match result {
            Ok(text) => {
                let mut parser = Parser::new(&text[..]);
                match parser.parse() {
                    Ok(tree) => println!("{}", tree),
                    Err(err) => eprintln!("{}", err),
                };
            }
            Err(_) => (),
        }
    }
}
