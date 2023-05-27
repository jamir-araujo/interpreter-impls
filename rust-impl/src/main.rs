use crate::repl::repl_start;
use anyhow::Result;

mod lexer;
mod repl;

fn main() -> Result<()> {
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands");

    let mut input = std::io::stdin();
    let mut output = std::io::stdout();

    repl_start(&mut input, &mut output)?;

    return Ok(());
}
