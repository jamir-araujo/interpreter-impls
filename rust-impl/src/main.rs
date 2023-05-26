use crate::lexer::Lexer;


mod lexer;

fn main() {
    let input = "let five = 5;
    let ten = 10;
    let add = fn(x, y) {
    x + y;
    };
    let result = add(five, ten);";

    let mut lexer = Lexer::new(input.into());

    let token = lexer.next_token();

    println!("Hello, world!");
}
