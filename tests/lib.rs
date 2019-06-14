use lisp_rs::env::Env;
use lisp_rs::value::LispValue;
use lisp_rs::*;
use std::collections::VecDeque;

fn eval_test(s: &str) -> LispValue {
    let env = Env::global();
    let mut tokens = token::tokenize(s);
    let mut list = value::parse(&mut tokens);
    env.eval(&mut list)
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
