use std::mem;

use anyhow::Ok;

use crate::{
    ast::{Expression, Let, Program, Statement},
    lexer::{Lexer, Token},
};

struct Parser {
    lexer: Lexer,
    pub current_token: Token,
    pub peek_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();

        return Self {
            lexer,
            current_token,
            peek_token,
        };
    }

    pub fn next_token(&mut self) {
        self.current_token = self.lexer.next_token();
        mem::swap(&mut self.current_token, &mut self.peek_token);
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program { statements: vec![] };

        while self.current_token != Token::Eof {
            if let Some(s) = self.parse_statement() {
                program.statements.push(s);
            }

            self.next_token();
        }

        return Some(program);
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        let stat = match self.current_token {
            Token::Let => self.parse_let_statement(),
            _ => None,
        };

        return stat;
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        if !self.expect_peek(Token::Ident("".into())) {
            return None;
        }

        let current_token = self.current_token.clone();
        if let Token::Ident(ident) = current_token {
            if !self.expect_peek(Token::Assign) {
                return None;
            }

            while !self.curr_token_is(Token::Semicolon) {
                self.next_token();
            }

            let let_sta = Let {
                name: ident.clone(),
                value: Expression::Value("".into()),
            };

            return Some(Statement::Let(let_sta));
        }

        return None;
    }

    fn curr_token_is(&mut self, token: Token) -> bool {
        return matches!(&self.current_token, token);
    }

    fn peek_token_is(&mut self, token: Token) -> bool {
        return matches!(&self.peek_token, token);
    }

    fn expect_peek(&mut self, token: Token) -> bool {
        if self.peek_token_is(token) {
            self.next_token();
            return true;
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::Statement, lexer::Lexer};

    use super::Parser;

    #[test]
    fn test() {
        let input = "let x = 5;
        let y = 10;
        let foobar = 838383;";

        let lexer = Lexer::new(input.into());
        let mut parser = Parser::new(lexer);

        let program = match parser.parse_program() {
            Some(p) => p,
            None => panic!("parse_program return Node"),
        };

        let statement_count = program.statements.len();
        assert_eq!(
            3, statement_count,
            "program.statements does not contain 3 statements. got={}",
            statement_count
        );

        let names: Vec<String> = vec!["x".into(), "y".into(), "foobar".into()];

        for (i, name) in names.iter().enumerate() {
            unsafe {
                let c = program.statements.get_unchecked(i);

                assert_let_statement(&name, &c);
            }
        }
    }

    fn assert_let_statement(expected_name: &String, statement: &Statement) {
        match statement {
            Statement::Let(l) => assert_eq!(expected_name, &l.name),
            Statement::Expression(_) => panic!(),
        };
    }
}
