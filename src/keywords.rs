use crate::token_type::TokenType::{self, *};
use std::collections::HashMap;

pub fn get() -> HashMap<String, TokenType> {
    let mut keywords = HashMap::new();

    keywords.insert(String::from("and"),    And);
    keywords.insert(String::from("class"),  Class);
    keywords.insert(String::from("else"),   Else);
    keywords.insert(String::from("false"),  False);
    keywords.insert(String::from("for"),    For);
    keywords.insert(String::from("fun"),    Fun);
    keywords.insert(String::from("if"),     If);
    keywords.insert(String::from("nil"),    Nil);
    keywords.insert(String::from("or"),     Or);
    keywords.insert(String::from("print"),  Print);
    keywords.insert(String::from("return"), Return);
    keywords.insert(String::from("super"),  Super);
    keywords.insert(String::from("this"),   This);
    keywords.insert(String::from("true"),   True);
    keywords.insert(String::from("var"),    Var);
    keywords.insert(String::from("while"),  While);

    keywords
}
