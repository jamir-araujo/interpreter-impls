#[derive(Debug, PartialEq)]
pub enum Token {
    Int(String),
    Ident(String),
    Illegal,
    Eof,
    Assign,
    Plus,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Function,
    Let,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    If,
    Else,
    Return,
    True,
    False,
    Eq,
    NotEq,
}

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    char: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            char: 0,
        };

        lexer.read_next();

        return lexer;
    }

    fn read_next(&mut self) {
        if self.read_position >= self.input.len() {
            self.char = 0;
        } else {
            self.char = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        } else {
            return self.input[self.read_position];
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.char {
            b'=' => {
                let mut token = Token::Assign;
                if self.peek_char() == b'=' {
                    token = Token::Eq;

                    self.read_next();
                }

                token
            }
            b';' => Token::Semicolon,
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b',' => Token::Comma,
            b'+' => Token::Plus,
            b'{' => Token::Lbrace,
            b'}' => Token::Rbrace,
            b'-' => Token::Minus,
            b'!' => {
                let mut token = Token::Bang;
                if self.peek_char() == b'=' {
                    token = Token::NotEq;

                    self.read_next();
                }

                token
            },
            b'*' => Token::Asterisk,
            b'/' => Token::Slash,
            b'<' => Token::Lt,
            b'>' => Token::Gt,
            b'0'..=b'9' => return Token::Int(self.read_int()),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident();
                return match ident.as_str() {
                    "let" => Token::Let,
                    "fn" => Token::Function,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "return" => Token::Return,
                    "true" => Token::True,
                    "false" => Token::False,
                    _ => Token::Ident(ident),
                };
            }
            0 => Token::Eof,
            _ => Token::Illegal,
        };

        self.read_next();

        return token;
    }

    fn read_ident(&mut self) -> String {
        let start_pos = self.position;
        while self.is_ident_char() {
            self.read_next();
        }

        return String::from_utf8_lossy(&self.input[start_pos..self.position]).to_string();
    }

    fn is_ident_char(&mut self) -> bool {
        self.char.is_ascii_alphabetic() || self.char == b'_'
    }

    fn skip_whitespace(&mut self) {
        while self.char.is_ascii_whitespace() {
            self.read_next();
        }
    }

    fn read_int(&mut self) -> String {
        let position = self.position;
        while self.char.is_ascii_digit() {
            self.read_next();
        }

        return String::from_utf8_lossy(&self.input[position..self.position]).to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, Token};

    #[test]
    fn test_next_token_1() {
        let input = "let five = 5;
        let ten = 10;
        let add = fn(x, y) {
        x + y;
        };
        let result = add(five, ten);";

        let tokens = vec![
            Token::Let,
            Token::Ident("five".into()),
            Token::Assign,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".into()),
            Token::Assign,
            Token::Int("10".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".into()),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Ident("x".into()),
            Token::Comma,
            Token::Ident("y".into()),
            Token::Rparen,
            Token::Lbrace,
            Token::Ident("x".into()),
            Token::Plus,
            Token::Ident("y".into()),
            Token::Semicolon,
            Token::Rbrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".into()),
            Token::Assign,
            Token::Ident("add".into()),
            Token::Lparen,
            Token::Ident("five".into()),
            Token::Comma,
            Token::Ident("ten".into()),
            Token::Rparen,
            Token::Semicolon,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input.into());

        for exp_token in tokens {
            let token = lexer.next_token();
            println!("expected: {:?}, received {:?}", exp_token, token);
            assert_eq!(exp_token, token)
        }
    }

    #[test]
    fn test_next_token_2() {
        let input = r#"let five = 5;
        let ten = 10;
        let add = fn(x, y) {
        x + y;
        };
        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;"#;

        let tokens = vec![
            Token::Let,
            Token::Ident("five".into()),
            Token::Assign,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".into()),
            Token::Assign,
            Token::Int("10".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".into()),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Ident("x".into()),
            Token::Comma,
            Token::Ident("y".into()),
            Token::Rparen,
            Token::Lbrace,
            Token::Ident("x".into()),
            Token::Plus,
            Token::Ident("y".into()),
            Token::Semicolon,
            Token::Rbrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".into()),
            Token::Assign,
            Token::Ident("add".into()),
            Token::Lparen,
            Token::Ident("five".into()),
            Token::Comma,
            Token::Ident("ten".into()),
            Token::Rparen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Int("5".into()),
            Token::Lt,
            Token::Int("10".into()).into(),
            Token::Gt,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input.into());

        for exp_token in tokens {
            let token = lexer.next_token();
            println!("expected: {:?}, received {:?}", exp_token, token);
            assert_eq!(exp_token, token)
        }
    }

    #[test]
    fn test_next_token_3() {
        let input = r#"let five = 5;
        let ten = 10;
        let add = fn(x, y) {
        x + y;
        };
        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;
        
        if (5 < 10) {
        return true;
        } else {
        return false;
        }"#;

        let tokens = vec![
            Token::Let,
            Token::Ident("five".into()),
            Token::Assign,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".into()),
            Token::Assign,
            Token::Int("10".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".into()),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Ident("x".into()),
            Token::Comma,
            Token::Ident("y".into()),
            Token::Rparen,
            Token::Lbrace,
            Token::Ident("x".into()),
            Token::Plus,
            Token::Ident("y".into()),
            Token::Semicolon,
            Token::Rbrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".into()),
            Token::Assign,
            Token::Ident("add".into()),
            Token::Lparen,
            Token::Ident("five".into()),
            Token::Comma,
            Token::Ident("ten".into()),
            Token::Rparen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Int("5".into()),
            Token::Lt,
            Token::Int("10".into()).into(),
            Token::Gt,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::If,
            Token::Lparen,
            Token::Int("5".into()),
            Token::Lt,
            Token::Int("10".into()),
            Token::Rparen,
            Token::Lbrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::Rbrace,
            Token::Else,
            Token::Lbrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::Rbrace,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input.into());

        for exp_token in tokens {
            let token = lexer.next_token();
            println!("expected: {:?}, received {:?}", exp_token, token);
            assert_eq!(exp_token, token)
        }
    }

    #[test]
    fn test_next_token_4() {
        let input = r#"let five = 5;
        let ten = 10;
        let add = fn(x, y) {
        x + y;
        };
        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;
        
        if (5 < 10) {
        return true;
        } else {
        return false;
        }
        
        10 == 10;
        10 != 9;
        "#;

        let tokens = vec![
            Token::Let,
            Token::Ident("five".into()),
            Token::Assign,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".into()),
            Token::Assign,
            Token::Int("10".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".into()),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Ident("x".into()),
            Token::Comma,
            Token::Ident("y".into()),
            Token::Rparen,
            Token::Lbrace,
            Token::Ident("x".into()),
            Token::Plus,
            Token::Ident("y".into()),
            Token::Semicolon,
            Token::Rbrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".into()),
            Token::Assign,
            Token::Ident("add".into()),
            Token::Lparen,
            Token::Ident("five".into()),
            Token::Comma,
            Token::Ident("ten".into()),
            Token::Rparen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Int("5".into()),
            Token::Lt,
            Token::Int("10".into()).into(),
            Token::Gt,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::If,
            Token::Lparen,
            Token::Int("5".into()),
            Token::Lt,
            Token::Int("10".into()),
            Token::Rparen,
            Token::Lbrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::Rbrace,
            Token::Else,
            Token::Lbrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::Rbrace,
            Token::Int("10".into()),
            Token::Eq,
            Token::Int("10".into()),
            Token::Semicolon,
            Token::Int("10".into()),
            Token::NotEq,
            Token::Int("9".into()),
            Token::Semicolon,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input.into());

        for exp_token in tokens {
            let token = lexer.next_token();
            println!("expected: {:?}, received {:?}", exp_token, token);
            assert_eq!(exp_token, token)
        }
    }
}
