mod cli;
mod lang;

use cli::command_line_parser::CommandLineParser;
use cli::parsed_options::ParsedOptions;
use lang::compiler::Compiler;

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

    if options.has("--compile") {
        let res = compile(&options);

        match res {
            Ok(_) => {}
            Err(error) => eprintln!("{}", error),
        }
    }
}

fn compile(options: &ParsedOptions) -> Result<(), String> {
    let file_path = options.get("--compile").unwrap();

    let compiler = Compiler::from_file(file_path).unwrap();
    compiler.compile()?;

    Ok(())
}
