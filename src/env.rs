use super::value::{LispList, LispValue};
use std::collections::{HashMap, VecDeque};
type Variables = HashMap<String, LispValue>;

pub struct Env<'a> {
    variables: Variables,
    outer: Option<&'a Env<'a>>,
}

impl<'a> Env<'a> {
    pub fn global() -> Self {
        let mut variables = Variables::new();

        variables.insert(
            "+".to_string(),
            LispValue::Lambda(|l| match l.get(0).unwrap() {
                LispValue::Int(first) => {
                    let mut l: Vec<_> = l
                        .into_iter()
                        .map(|v| match v {
                            LispValue::Int(i) => *i,
                            _ => panic!("Unexpected type"),
                        })
                        .collect();
                    let result = l.drain(1..).fold(*first, |acc, i| acc + i);
                    LispValue::Int(result)
                }
                LispValue::Float(first) => {
                    let mut l: Vec<_> = l
                        .into_iter()
                        .map(|v| match v {
                            LispValue::Float(i) => *i,
                            _ => panic!("Unexpected type"),
                        })
                        .collect();
                    let result = l.drain(1..).fold(*first, |acc, i| acc + i);
                    LispValue::Float(result)
                }
                _ => panic!("Unexpected type"),
            }),
        );

        variables.insert(
            "*".to_string(),
            LispValue::Lambda(|l| match l.get(0).unwrap() {
                LispValue::Int(first) => {
                    let mut l: Vec<_> = l
                        .into_iter()
                        .map(|v| match v {
                            LispValue::Int(i) => *i,
                            _ => panic!("Unexpected type"),
                        })
                        .collect();
                    let result = l.drain(1..).fold(*first, |acc, i| acc * i);
                    LispValue::Int(result)
                }
                LispValue::Float(first) => {
                    let mut l: Vec<_> = l
                        .into_iter()
                        .map(|v| match v {
                            LispValue::Float(i) => *i,
                            _ => panic!("Unexpected type"),
                        })
                        .collect();
                    let result = l.drain(1..).fold(*first, |acc, i| acc * i);
                    LispValue::Float(result)
                }
                _ => panic!("Unexpected type"),
            }),
        );

        variables.insert(
            "-".to_string(),
            LispValue::Lambda(|l| match l.get(0).unwrap() {
                LispValue::Int(first) => {
                    let mut l: Vec<_> = l
                        .into_iter()
                        .map(|v| match v {
                            LispValue::Int(i) => *i,
                            _ => panic!("Unexpected type"),
                        })
                        .collect();
                    let result = l.drain(1..).fold(*first, |acc, i| acc - i);
                    LispValue::Int(result)
                }
                LispValue::Float(first) => {
                    let mut l: Vec<_> = l
                        .into_iter()
                        .map(|v| match v {
                            LispValue::Float(i) => *i,
                            _ => panic!("Unexpected type"),
                        })
                        .collect();
                    let result = l.drain(1..).fold(*first, |acc, i| acc - i);
                    LispValue::Float(result)
                }
                _ => panic!("Unexpected type"),
            }),
        );

        variables.insert(
            "/".to_string(),
            LispValue::Lambda(|l| match l.get(0).unwrap() {
                LispValue::Int(first) => {
                    let mut l: Vec<_> = l
                        .into_iter()
                        .map(|v| match v {
                            LispValue::Int(i) => *i,
                            _ => panic!("Unexpected type"),
                        })
                        .collect();
                    let result = l.drain(1..).fold(*first, |acc, i| acc / i);
                    LispValue::Int(result)
                }
                LispValue::Float(first) => {
                    let mut l: Vec<_> = l
                        .into_iter()
                        .map(|v| match v {
                            LispValue::Float(i) => *i,
                            _ => panic!("Unexpected type"),
                        })
                        .collect();
                    let result = l.drain(1..).fold(*first, |acc, i| acc / i);
                    LispValue::Float(result)
                }
                _ => panic!("Unexpected type"),
            }),
        );

        variables.insert(
            "car".to_string(),
            LispValue::Lambda(|l| match l.get(0).unwrap() {
                LispValue::List(l) => l.get(0).unwrap().clone(),
                _ => panic!("{:?} is not a list", l),
            }),
        );

        variables.insert(
            "cdr".to_string(),
            LispValue::Lambda(|l| match l.get(0).unwrap() {
                LispValue::List(l) => LispValue::List(l.clone().drain(1..).collect()),
                _ => panic!("{:?} is not a list", l),
            }),
        );

        Env {
            variables,
            outer: None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&LispValue> {
        if let Some(v) = self.variables.get(key) {
            Some(v)
        } else if let Some(env) = self.outer {
            env.get(key)
        } else {
            None
        }
    }

    pub fn resolve(&self, ident: String) -> LispValue {
        if let Some(v) = self.get(&ident) {
            v.clone()
        } else {
            LispValue::Symbol(ident)
        }
    }

    pub fn eval(&self, value: &mut LispValue) -> LispValue {
        match value {
            LispValue::Symbol(sym) => self.resolve(sym.to_string()),
            LispValue::List(list) => self.eval_list(list),
            v => v.clone(),
        }
    }

    fn eval_list(&self, list: &mut LispList) -> LispValue {
        let mut evaluated: VecDeque<_> = list.into_iter().map(|e| self.eval(e)).collect();
        match evaluated.pop_front() {
            Some(LispValue::Lambda(f)) => f(&evaluated),
            Some(e) => {
                evaluated.push_front(e);
                LispValue::List(evaluated)
            }
            _ => panic!("Unexpected"),
        }
    }
}
