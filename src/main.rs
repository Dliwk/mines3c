use std::fs;
use std::process::exit;
use anyhow::Result;
use clap::Parser;

mod ast;
mod compile;
mod grammar;
mod tok;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to source file.
    #[arg(short, long)]
    path: String,
}

fn compile(source: &str) -> Result<String> {
    let tokenizer = tok::Tokenizer::new(source, 0);
    let parser = grammar::ProgramParser::new();
    let ast = match parser.parse(tokenizer) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("ParseError: {}", e);
            exit(1);
        }
    };
    let mut compiler = compile::Compiler::new();
    let code = compiler.compile(ast)?;
    Ok(code)
}

fn main() -> Result<()> {
    let args = Args::parse();

    let text = fs::read_to_string(args.path)?;
    let prog = compile(text.as_str()).unwrap();

    println!("{}", prog);

    Ok(())
}
