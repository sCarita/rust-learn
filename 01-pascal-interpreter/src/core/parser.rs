use crate::core::token::Token;
use crate::core::lexer::Lexer;
use crate::core::ast::AST;

pub struct Parser {
	lexer: Lexer,
	current_token: Option<Token>,
}

impl Parser {
	pub fn new(lexer: Lexer) -> Parser {
		let mut parser = Parser {
			lexer: lexer,
			current_token: None,
		};
		parser.current_token = Some(parser.lexer.get_next_token());

		parser
	}

	fn eat(&mut self, token: Token) {
		if self.current_token.clone().unwrap() == token {
			self.current_token = Some(self.lexer.get_next_token());
		} else {
			panic!("Invalid Syntax");
		}
	}
	
	// factor : INTEGER | LPAREN expr RPAREN
	fn factor(&mut self) -> AST {
		let token = self.current_token.clone().unwrap();

		match token {
			Token::INTEGER(i) => {
				self.eat(Token::INTEGER(i));
				return AST::new(token, vec![]);
			},
			Token::LPAREN => {
				self.eat(Token::LPAREN);
				let node = self.expr();
				self.eat(Token::RPAREN);
				return node;
			},
			_ => panic!("Invalid Syntax")
		};
	}

	// term : factor ((MUL | DIV) factor)*
	fn term(&mut self) -> AST {
		let mut node = self.factor();

		while self.current_token == Some(Token::MUL) ||
			self.current_token ==Some(Token::DIV) {
				// current_token is MUL or DIV?
				match self.current_token {
					Some(Token::MUL) => {
						self.eat(Token::MUL);
						let children: Vec<AST> = vec![node, self.factor()];
                		node = AST::new(Token::MUL, children);
					},
					Some(Token::DIV) => {
						self.eat(Token::DIV);
						let children: Vec<AST> = vec![node, self.factor()];
                		node = AST::new(Token::DIV, children);
					},
					_ => panic!("Invalid Syntax"),
				};
			}
		node
	}

	/*
		expr   : term ((PLUS | MINUS) term)*
        term   : factor ((MUL | DIV) factor)*
        factor : INTEGER | LPAREN expr RPAREN
	*/
	fn expr (&mut self) -> AST {
		let mut node = self.term();

		while self.current_token == Some(Token::PLUS) ||
			self.current_token == Some(Token::MINUS) {

			// current_token is PLUS or MINUS?
			match self.current_token {
				Some(Token::PLUS) => {
					self.eat(Token::PLUS);
					let children: Vec<AST> = vec![node, self.term()];
            		node = AST::new(Token::PLUS, children);
				},
				Some(Token::MINUS) => {
					self.eat(Token::MINUS);
					let children: Vec<AST> = vec![node, self.term()];
            		node = AST::new(Token::MINUS, children);
				},
				_ => panic!("Invalid Syntax"),
			}
        }
        node
	}

	pub fn parse(&mut self) -> AST {
		self.expr()
	}
}
