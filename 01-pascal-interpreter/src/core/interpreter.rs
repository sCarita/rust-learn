use crate::core::parser::Parser;
use crate::core::token::Token;
use crate::core::ast::AST;

pub struct Interpreter {
	parser: Parser,
}

impl Interpreter {
	pub fn new(parser: Parser) -> Interpreter {
		Interpreter {
			parser: parser,
		}
	}

	fn visit_num(&self, node: &AST) -> i32 {
		match node.token {
			Token::INTEGER(i) => {
				return i;
			},
			_ => panic!("Error"),
		}
	}

	fn visit_binop(&self, node: &AST) -> i32 {
		let left_value = self.visit(&node.children[0]);
		let right_value = self.visit(&node.children[1]);

		match node.token {
			Token::PLUS => {
				return left_value + right_value;
			},
			Token::MINUS => {
				return left_value - right_value;
			},
			Token::DIV => {
				return left_value / right_value;
			},
			Token::MUL => {
				return left_value * right_value;
			},
			_ => panic!("Error"),
		}
	}

	fn visit(&self, node: &AST) -> i32 {
		match node.token {
			Token::INTEGER(_i) => {
				return self.visit_num(node);
			},
			Token::PLUS | Token::MINUS | Token::MUL | Token::DIV => {
				return self.visit_binop(node);
			},
			_ => panic!("Error"),
		}
	}

	pub fn interpret(&mut self) -> i32 {
		let tree = self.parser.parse();
		let result = self.visit(&tree);

		result
	}
}