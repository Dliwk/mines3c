use anyhow::Result;
use clap::Parser;
use std::fs;
use std::process::exit;

mod ast;
mod compile;
mod grammar;
mod tok;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Путь к файлу с исходным кодом.
    #[arg(short, long)]
    path: String,

    /// Не использовать стандартную библиотеку
    #[arg(short, long)]
    nostdlib: bool,
}

fn compile(source: &str, shift: usize) -> Result<String> {
    let tokenizer = tok::Tokenizer::new(source, shift);
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

    let mut text = fs::read_to_string(args.path)?;
    let stdlib = include_str!("std.mines3");

    let shift = if !args.nostdlib {
        text = stdlib.to_owned() + "\n" + text.as_str();

        stdlib.len() + 1
    } else {
        0
    };

    let prog = compile(text.as_str(), shift)?;

    println!("{}", prog);

    Ok(())
}
