use lisp_rs::env::Env;
use lisp_rs::token;
use lisp_rs::value;
use std::env;
use std::io::stdin;

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
