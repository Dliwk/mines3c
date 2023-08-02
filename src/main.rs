use anyhow::Error;

mod ast;
mod grammar;
mod tok;

fn main() -> Result<(), Error> {
    let text = include_str!("../prog.mines3");
    let ast = grammar::ProgramParser::new().parse(text, tok::Tokenizer::new(text, 0))?;

    println!("{:#?}", ast);

    Ok(())
}
