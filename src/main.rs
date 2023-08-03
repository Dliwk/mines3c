use anyhow::Error;

mod ast;
mod compile;
mod grammar;
mod tok;

fn main() -> Result<(), Error> {
    let text = include_str!("../prog_simple.mines3");
    let ast = grammar::ProgramParser::new().parse(text, tok::Tokenizer::new(text, 0))?;
    let mut compiler = compile::Compiler::new();
    let code = compiler.compile(ast)?;
    println!("{}", code);

    Ok(())
}
