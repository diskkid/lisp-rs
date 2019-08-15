use std::collections::vec_deque::VecDeque;
use std::fmt;
use std::fmt::Debug;

use super::env::Env;
use super::token::Token;

pub type LispList = VecDeque<LispValue>;

#[derive(Clone)]
pub enum LispValue {
    Symbol(String),
    True,
    False,
    Int(i64),
    Float(f64),
    Lambda(fn(&Env, &LispList) -> LispValue),
    List(LispList),
}

impl Debug for LispValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LispValue::Symbol(s) => write!(f, "{:?}", s),
            LispValue::True => write!(f, "true"),
            LispValue::False => write!(f, "false"),
            LispValue::Int(i) => write!(f, "{:?}", i),
            LispValue::Float(n) => write!(f, "{:?}", n),
            LispValue::Lambda(_) => write!(f, "lambda"),
            LispValue::List(v) => write!(f, "{:?}", v),
        }
    }
}

impl PartialEq for LispValue {
    fn eq(&self, other: &Self) -> bool {
        match self {
            LispValue::Symbol(l) => match other {
                LispValue::Symbol(r) => l == r,
                _ => false,
            },
            LispValue::True => match other {
                LispValue::True => true,
                _ => false,
            },
            LispValue::False => match other {
                LispValue::False => true,
                _ => false,
            },
            LispValue::Int(l) => match other {
                LispValue::Int(r) => l == r,
                _ => false,
            },
            LispValue::Float(l) => match other {
                LispValue::Float(r) => l == r,
                _ => false,
            },
            LispValue::Lambda(_) => false,
            LispValue::List(l) => match other {
                LispValue::List(r) => l == r,
                _ => false,
            },
        }
    }
}

pub fn parse(mut tokens: &mut VecDeque<Token>) -> LispValue {
    let token = tokens.pop_front().unwrap();
    match token {
        Token::OParen => {
            let mut l = VecDeque::new();
            while let Some(t) = tokens.front() {
                match t {
                    Token::CParen => break,
                    _ => l.push_back(parse(&mut tokens)),
                }
            }
            LispValue::List(l)
        }
        Token::CParen => panic!("Unexpected )"),
        Token::Symbol(s) => LispValue::Symbol(s),
        Token::Int(i) => LispValue::Int(i),
        Token::Float(f) => LispValue::Float(f),
        Token::False => LispValue::False,
        Token::True => LispValue::True,
    }
}
