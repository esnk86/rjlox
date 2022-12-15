use crate::cutter::Cutter;
use crate::keywords;
use crate::token::Token;
use crate::token_type::TokenType::{self, *};
use crate::value::Value;
use std::collections::HashMap;

pub fn scan(input: &str) -> Option<Vec<Token>> {
    let mut output = Vec::new();
    let mut scanner = Scanner::new(input, &mut output);

    scanner.scan();

    if scanner.error {
        None
    } else {
        Some(output)
    }
}

struct Scanner<'a> {
    output: &'a mut Vec<Token>,
    error: bool,
    cutter: Cutter,
    keywords: HashMap<String, TokenType>,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &str, output: &'a mut Vec<Token>) -> Self {
        Self {
            output,
            error: false,
            cutter: Cutter::new(input),
            keywords: keywords::get(),
        }
    }

    pub fn scan(&mut self) {
        while !self.cutter.eof() {
            self.scan_token();
        }

        self.add_token(EOF, None);
    }

    fn scan_token(&mut self) {
        let c = self.cutter.next().unwrap();

        match c {
            '(' => self.add_token(LeftParen, None),
            ')' => self.add_token(RightParen, None),
            '{' => self.add_token(LeftBrace, None),
            '}' => self.add_token(RightBrace, None),
            ',' => self.add_token(Comma, None),
            '.' => self.add_token(Dot, None),
            '-' => self.add_token(Minus, None),
            '+' => self.add_token(Plus, None),
            ';' => self.add_token(Semicolon, None),
            '*' => self.add_token(Star, None),
            '!' => self.add_maybe_equal(Bang, BangEqual),
            '<' => self.add_maybe_equal(Less, LessEqual),
            '=' => self.add_maybe_equal(Equal, EqualEqual),
            '>' => self.add_maybe_equal(Greater, GreaterEqual),
            '"' => self.string(),
            '/' => self.slash(),
            _ => {
                if c.is_ascii_whitespace() {
                    // Ignore whitespace.
                    self.cutter.cut();
                } else if c.is_ascii_alphabetic() || c == '_' {
                    self.word();
                } else if c.is_ascii_digit() {
                    self.number();
                } else {
                    self.report(&format!("unexpected character: {:?}", c));
                }
            }
        }
    }

    fn add_maybe_equal(&mut self, operator: TokenType, operator_equal: TokenType) {
        let tag = if self.cutter.next_eq('=') {
            operator_equal
        } else {
            operator
        };

        self.add_token(tag, None);
    }

    fn add_token(&mut self, tag: TokenType, value: Option<Value>) {
        let lexeme = self.cutter.cut();
        let token = Token::new(tag, lexeme, value, self.cutter.line);

        self.output.push(token);
    }

    fn word(&mut self) {
        self.cutter.next_while(|c| c.is_ascii_alphanumeric() || c == '_');

        let lexeme = self.cutter.copy();

        let (tag, value) = match self.keywords.get(&lexeme) {
            Some(&keyword) => (keyword, None),
            None => (Word, Some(Value::Word(lexeme))),
        };

        self.add_token(tag, value);
    }

    fn number(&mut self) {
        // Integer part.
        self.cutter.next_while(|c| c.is_ascii_digit());

        // Fraction part.
        if self.cutter.next_eq('.') {
            self.cutter.next_while(|c| c.is_ascii_digit());
        }

        // Parse lexeme.
        let lexeme = self.cutter.copy();
        let number = lexeme.parse::<f64>().unwrap();
        let value = Some(Value::Number(number));

        self.add_token(Number, value);
    }

    fn string(&mut self) {
        self.cutter.next_while(|c| c != '"');

        if self.cutter.eof() {
            self.report("unterminated string");
            return;
        }

        // The closing double quote.
        self.cutter.next();

        let mut value = self.cutter.copy();
        value.remove(0);
        value.remove(value.len() - 1);
        let value = Some(Value::Str(value));

        self.add_token(Str, value);
    }

    fn slash(&mut self) {
        if self.cutter.next_eq('/') {
            self.cutter.next_while(|c| c != '\n');
            self.cutter.cut();
        } else {
            self.add_token(Slash, None);
        }
    }

    fn report(&mut self, msg: &str) {
        eprintln!("[line {}] error: {}", self.cutter.line, msg);
        self.error = true;
    }
}
