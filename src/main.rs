use std::collections::vec_deque::VecDeque;
use std::collections::HashMap;
use std::fmt::Debug;
use std::{fmt, env};
use std::io::stdin;

#[derive(Debug)]
enum Token {
    OParen,
    CParen,
    Symbol(String),
    Float(f64),
    Int(i64),
    True,
    False,
}

type LispList = VecDeque<LispValue>;

#[derive(Clone)]
enum LispValue {
    Symbol(String),
    True,
    False,
    Int(i64),
    Float(f64),
    Lambda(fn(&LispList) -> LispValue),
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
            LispValue::Symbol(l) => {
                match other {
                    LispValue::Symbol(r) => l == r,
                    _ => false,
                }
            },
            LispValue::True => {
                match other {
                    LispValue::True => true,
                    _ => false,
                }
            },
            LispValue::False => {
                match other {
                    LispValue::False => true,
                    _ => false,
                }
            },
            LispValue::Int(l) => {
                match other {
                    LispValue::Int(r)=> l == r,
                    _ => false,
                }
            },
            LispValue::Float(l) => {
                match other {
                    LispValue::Float(r)=> l == r,
                    _ => false,
                }
            },
            LispValue::Lambda(_) => false,
            LispValue::List(l) => {
                match other {
                    LispValue::List(r)=> l == r,
                    _ => false,
                }
            }
        }
    }
}

type Variables = HashMap<String, LispValue>;

struct Env<'a> {
    variables: Variables,
    outer: Option<&'a Env<'a>>,
}

impl <'a> Env<'a> {
    fn global() -> Self {
        let mut variables = Variables::new();

        variables.insert("+".to_string(), LispValue::Lambda(|l| {
            match l.get(0).unwrap() {
                LispValue::Int(first) => {
                    let mut l: Vec<_> = l.into_iter().map(|v| match v {
                        LispValue::Int(i) => *i,
                        _ => panic!("Unexpected type")
                    }).collect();
                    let result = l.drain(1..).fold(*first, |acc, i| acc + i);
                    LispValue::Int(result)
                },
                LispValue::Float(first) => {
                    let mut l: Vec<_> = l.into_iter().map(|v| match v {
                        LispValue::Float(i) => *i,
                        _ => panic!("Unexpected type")
                    }).collect();
                    let result = l.drain(1..).fold(*first, |acc, i| acc + i);
                    LispValue::Float(result)
                },
                _ => panic!("Unexpected type")
            }
        }));

        variables.insert("*".to_string(), LispValue::Lambda(|l| {
            match l.get(0).unwrap() {
                LispValue::Int(first) => {
                    let mut l: Vec<_> = l.into_iter().map(|v| match v {
                        LispValue::Int(i) => *i,
                        _ => panic!("Unexpected type")
                    }).collect();
                    let result = l.drain(1..).fold(*first, |acc, i| acc * i);
                    LispValue::Int(result)
                },
                LispValue::Float(first) => {
                    let mut l: Vec<_> = l.into_iter().map(|v| match v {
                        LispValue::Float(i) => *i,
                        _ => panic!("Unexpected type")
                    }).collect();
                    let result = l.drain(1..).fold(*first, |acc, i| acc * i);
                    LispValue::Float(result)
                },
                _ => panic!("Unexpected type")
            }
        }));

        variables.insert("-".to_string(), LispValue::Lambda(|l| {
            match l.get(0).unwrap() {
                LispValue::Int(first) => {
                    let mut l: Vec<_> = l.into_iter().map(|v| match v {
                        LispValue::Int(i) => *i,
                        _ => panic!("Unexpected type")
                    }).collect();
                    let result = l.drain(1..).fold(*first, |acc, i| acc - i);
                    LispValue::Int(result)
                },
                LispValue::Float(first) => {
                    let mut l: Vec<_> = l.into_iter().map(|v| match v {
                        LispValue::Float(i) => *i,
                        _ => panic!("Unexpected type")
                    }).collect();
                    let result = l.drain(1..).fold(*first, |acc, i| acc - i);
                    LispValue::Float(result)
                },
                _ => panic!("Unexpected type")
            }
        }));

        variables.insert("/".to_string(), LispValue::Lambda(|l| {
            match l.get(0).unwrap() {
                LispValue::Int(first) => {
                    let mut l: Vec<_> = l.into_iter().map(|v| match v {
                        LispValue::Int(i) => *i,
                        _ => panic!("Unexpected type")
                    }).collect();
                    let result = l.drain(1..).fold(*first, |acc, i| acc / i);
                    LispValue::Int(result)
                },
                LispValue::Float(first) => {
                    let mut l: Vec<_> = l.into_iter().map(|v| match v {
                        LispValue::Float(i) => *i,
                        _ => panic!("Unexpected type")
                    }).collect();
                    let result = l.drain(1..).fold(*first, |acc, i| acc / i);
                    LispValue::Float(result)
                },
                _ => panic!("Unexpected type")
            }
        }));

        variables.insert("car".to_string(), LispValue::Lambda(|l| {
            match l.get(0).unwrap() {
                LispValue::List(l) => l.get(0).unwrap().clone(),
                _ => panic!("{:?} is not a list", l)
            }
        }));

        variables.insert("cdr".to_string(), LispValue::Lambda(|l| {
            match l.get(0).unwrap() {
                LispValue::List(l) => LispValue::List(l.clone().drain(1..).collect()),
                _ => panic!("{:?} is not a list", l)
            }
        }));

        Env {
            variables,
            outer: None,
        }
    }

    fn get(&self, key: &str) -> Option<&LispValue> {
        if let Some(v) = self.variables.get(key) {
            Some(v)
        } else if let Some(env) = self.outer {
            env.get(key)
        } else {
            None
        }
    }
}

fn resolve(ident: String, env: &Env) -> LispValue {
    if let Some(v) = env.get(&ident) {
        v.clone()
    } else {
        LispValue::Symbol(ident)
    }
}

fn call_function(func: String, args: &LispList, env: &Env) -> LispValue {
    if let Some(f) = env.get(&func) {
        match f {
            LispValue::Lambda(f) => {
                return f(args)
            },
            _ => {},
        }
    }
    let mut list = args.clone();
    list.push_front(LispValue::Symbol(func));
    LispValue::List(list)
}

fn eval_list(list: &mut LispList, env: &Env) -> LispValue {
    let mut evaluated: VecDeque<_> = list.into_iter().map(|e| eval(e, &env)).collect();
    match evaluated.pop_front() {
        Some(LispValue::Lambda(f)) => f(&evaluated),
        Some(e) => {
            evaluated.push_front(e);
            LispValue::List(evaluated)
        },
        _ => panic!("Unexpected"),
    }
}

fn eval(value: &mut LispValue, env: &Env) -> LispValue {
    match value {
        LispValue::Symbol(sym) => resolve(sym.to_string(), &env),
        LispValue::List(list) => eval_list(list, &env),
        v => v.clone()
    }
}

fn tokenize(s: &str) -> VecDeque<Token> {
    s.replace('(', " ( ")
        .replace(')', " ) ")
        .split_whitespace()
        .map(|s| {
            match s {
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
                },
            }
        }).collect()
}

fn parse(mut tokens: &mut VecDeque<Token>) -> LispValue {
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
        Token::CParen => {
            panic!("Unexpected )")
        }
        Token::Symbol(s) => LispValue::Symbol(s),
        Token::Int(i) => LispValue::Int(i),
        Token::Float(f) => LispValue::Float(f),
        Token::False => LispValue::False,
        Token::True => LispValue::True,
    }
}

fn main() {
    let env = Env::global();

    if let Some(ref program) = env::args().nth(1) {
        let mut tokens = tokenize(program);
        let mut list = parse(&mut tokens);
        let list = eval(&mut list, &env);
        println!("{:?}", list);
    } else {
        loop {
            let mut program = String::new();
            stdin().read_line(&mut program).unwrap();

            if program == "" {
                break
            }

            let mut tokens = tokenize(&program);
            let mut list = parse(&mut tokens);
            let list = eval(&mut list, &env);
            println!("{:?}", list);
        }
    }
}

mod test {
    use super::*;
    use std::collections::vec_deque::VecDeque;

    fn eval_test(s: &str) -> LispValue {
        let env = Env::global();
        let mut tokens = tokenize(s);
        let mut list = parse(&mut tokens);
        eval(&mut list, &env)
    }

    #[test]
    fn test_addition() {
        let actual = eval_test("(+ 1 2 3)");
        assert_eq!(actual, LispValue::Int(6));

        let actual = eval_test("(+ 1.0 2.0 3.0)");
        assert_eq!(actual, LispValue::Float(6.0));
    }

    #[test]
    fn test_subtraction() {
        let actual = eval_test("(- 1 2 3)");
        assert_eq!(actual, LispValue::Int(-4));

        let actual = eval_test("(- 1.0 2.0 3.0)");
        assert_eq!(actual, LispValue::Float(-4.0));
    }

    #[test]
    fn test_multiplication() {
        let actual = eval_test("(* 1 2 3)");
        assert_eq!(actual, LispValue::Int(6));

        let actual = eval_test("(* 1.0 2.0 3.0)");
        assert_eq!(actual, LispValue::Float(6.0));
    }

    #[test]
    fn test_division() {
        let actual = eval_test("(/ 1 2 3)");
        assert_eq!(actual, LispValue::Int(0));

        let actual = eval_test("(/ 1.0 2.0 3.0)");
        assert_eq!(actual, LispValue::Float(1.0 / 6.0));
    }

    #[test]
    fn test_car() {
        let actual = eval_test("(car (1 2 3))");
        assert_eq!(actual, LispValue::Int(1));
    }

    #[test]
    fn test_cdr() {
        let actual = eval_test("(cdr (1 2 3))");
        let mut expected = VecDeque::new();
        expected.push_back(LispValue::Int(2));
        expected.push_back(LispValue::Int(3));
        let expected = LispValue::List(expected);
        assert_eq!(actual, expected);
    }
}
