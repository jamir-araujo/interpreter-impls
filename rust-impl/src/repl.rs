use anyhow::Result;
use std::io::{Stdin, Stdout, Write};

use crate::lexer::{Lexer, Token};

pub fn repl_start(input: &mut Stdin, output: &mut Stdout) -> Result<()>{
    let pronpt = ">>".as_bytes();

    let mut handler = output.lock();

    writeln!(handler, ">>")?;

    output.write(pronpt.clone())?;

    let mut line: String = Default::default();
    let _ = input.read_line(&mut line)?;

    let mut lexer = Lexer::new(line);

    loop {
        let token = lexer.next_token();
        if token == Token::Eof {
            break;
        }

        writeln!(handler, "{}", token.to_string())?;
    }
    
    return Ok(());
}
