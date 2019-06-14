use std::collections::vec_deque::VecDeque;

#[derive(Debug)]
pub enum Token {
    OParen,
    CParen,
    Symbol(String),
    Float(f64),
    Int(i64),
    True,
    False,
}

pub fn tokenize(s: &str) -> VecDeque<Token> {
    s.replace('(', " ( ")
        .replace(')', " ) ")
        .split_whitespace()
        .map(|s| match s {
            "(" => Token::OParen,
            ")" => Token::CParen,
            sym => {
                if let Ok(i) = sym.parse() {
                    Token::Int(i)
                } else if let Ok(f) = sym.parse() {
                    Token::Float(f)
                } else {
                    Token::Symbol(sym.to_string())
                }
            }
        })
        .collect()
}
