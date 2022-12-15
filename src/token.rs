use crate::token_type::TokenType;
use crate::value::Value;

#[derive(Debug)]
pub struct Token {
    tag: TokenType,
    lexeme: String,
    value: Option<Value>,
    line: usize,
}

impl Token {
    pub fn new(tag: TokenType, lexeme: String, value: Option<Value>, line: usize) -> Self {
        Self { tag, lexeme, value, line }
    }
}
