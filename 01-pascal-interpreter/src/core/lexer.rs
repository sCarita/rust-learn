use crate::core::token::Token;

pub struct Lexer {
    text: String,
    pos: i32,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(text: String) -> Lexer {
        let mut lexer = Lexer {
            text: text,
            pos: 0,
            current_char: None,
        };
        if lexer.text.len() > 0 {
            lexer.current_char = Some(lexer.text.as_bytes()[0] as char);
        }

        lexer
    }

    fn advance(&mut self) {
        self.pos += 1;
        if self.pos > self.text.len() as i32 - 1 {
            self.current_char = None; // Indicates end of input
        } else {
            self.current_char = Some(self.text.as_bytes()[self.pos as usize] as char);
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn integer(&mut self) -> i32 {
        let mut result = String::new();
        while let Some(ch) = self.current_char {
            if ch.is_digit(10) {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        result.parse::<i32>().unwrap()
    }

    pub fn get_next_token(&mut self) -> Token {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.skip_whitespace();
                continue;
            }

            if ch.is_digit(10) {
                return Token::INTEGER(self.integer());
            }

            match ch {
                '+' => {
                    self.advance();
                    return Token::PLUS;
                },
                '-' => {
                    self.advance();
                    return Token::MINUS;
                },
                '*' => {
                    self.advance();
                    return Token::MUL;
                },
                '/' => {
                    self.advance();
                    return Token::DIV;
                },
                '(' => {
                    self.advance();
                    return Token::LPAREN;
                },
                ')' => {
                    self.advance();
                    return Token::RPAREN;
                },
                _ => {}
            }

            panic!("Invalid character");
        }

        Token::EOF
    }

}
