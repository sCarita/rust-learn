use crate::core::token::Token;

pub struct AST {
	pub token: Token,
	pub children: Vec<AST>,
}

impl AST {
    pub fn new(token: Token, children: Vec<AST>) -> AST {
        AST {
            token: token,
            children: children,
        }
    }
}