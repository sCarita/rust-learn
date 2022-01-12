use std::io;
use std::io::Write;

mod core;
use crate::core::{lexer, parser, interpreter};

fn main() {

    loop {
        let mut input = String::new();

        let _ = io::stdout().write(b"spi> ");
        let _ = io::stdout().flush();

        io::stdin().read_line(&mut input).unwrap();

        let text = String::from(input.trim());
        let lexer = lexer::Lexer::new(text);
        let parser = parser::Parser::new(lexer);

        let mut interpreter = interpreter::Interpreter::new(parser);
        let result = interpreter.interpret();
        println!("{}", result);
    }

}
