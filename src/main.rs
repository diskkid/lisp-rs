use std::collections::vec_deque::VecDeque;

use lisp_rs::env::Env;
use lisp_rs::token;
use lisp_rs::value;
use lisp_rs::value::{LispList, LispValue};
use std::env;
use std::io::stdin;

fn call_function(func: String, args: &LispList, env: &Env) -> LispValue {
    if let Some(f) = env.get(&func) {
        match f {
            LispValue::Lambda(f) => return f(args),
            _ => {}
        }
    }
    let mut list = args.clone();
    list.push_front(LispValue::Symbol(func));
    LispValue::List(list)
}

fn main() {
    let env = Env::global();

    if let Some(ref program) = env::args().nth(1) {
        let mut tokens = token::tokenize(program);
        let mut list = value::parse(&mut tokens);
        let list = env.eval(&mut list);
        println!("{:?}", list);
    } else {
        loop {
            let mut program = String::new();
            stdin().read_line(&mut program).unwrap();

            if program == "" {
                break;
            }

            let mut tokens = token::tokenize(&program);
            let mut list = value::parse(&mut tokens);
            let list = env.eval(&mut list);
            println!("{:?}", list);
        }
    }
}
