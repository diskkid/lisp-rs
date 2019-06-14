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

fn eval_list(list: &mut LispList, env: &Env) -> LispValue {
    let mut evaluated: VecDeque<_> = list.into_iter().map(|e| eval(e, &env)).collect();
    match evaluated.pop_front() {
        Some(LispValue::Lambda(f)) => f(&evaluated),
        Some(e) => {
            evaluated.push_front(e);
            LispValue::List(evaluated)
        }
        _ => panic!("Unexpected"),
    }
}

fn eval(value: &mut LispValue, env: &Env) -> LispValue {
    match value {
        LispValue::Symbol(sym) => env.resolve(sym.to_string()),
        LispValue::List(list) => eval_list(list, &env),
        v => v.clone(),
    }
}

fn main() {
    let env = Env::global();

    if let Some(ref program) = env::args().nth(1) {
        let mut tokens = token::tokenize(program);
        let mut list = value::parse(&mut tokens);
        let list = eval(&mut list, &env);
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
            let list = eval(&mut list, &env);
            println!("{:?}", list);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eval_test(s: &str) -> LispValue {
        let env = Env::global();
        let mut tokens = token::tokenize(s);
        let mut list = value::parse(&mut tokens);
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
