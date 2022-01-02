extern crate yard;

use std::env;
use std::io::prelude::*;

fn main() {
    let inp = env::args().collect::<Vec<String>>();
    if inp.len() >= 2 {
        let v = yard::evaluate::<f64>(inp.last().unwrap()).expect("invalid input string");
        println!("{}", v);
    } else {
        repl();
    };
}

fn repl() {
    let input = std::io::stdin();
    let mut input = input.lock();

    loop {
        print!(">>> ");
        std::io::stdout().flush().expect("can't flush stdout");
        let mut buff = String::new();
        input
            .read_line(&mut buff)
            .expect("can't read line from stdin");

        let v = yard::evaluate::<f64>(&buff).expect("can't evaluate input string");
        println!("{}", v);
        buff.clear();
    }
}
