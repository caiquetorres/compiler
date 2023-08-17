mod cli;
mod lang;

use cli::command_line_parser::CommandLineParser;

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

fn repl() {}

fn compile() {}
