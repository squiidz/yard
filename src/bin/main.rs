extern crate yard;
use std::env;
use std::io;
use std::io::prelude::*;

fn main() {
    let inp = env::args().collect::<Vec<String>>();
    if inp.len() >= 2 {
        match yard::evaluate::<f64>(inp.last().unwrap()) {
            Ok(v) => println!("{}", v),
            Err(e) => println!("{}", e),
        }
    } else {
        repl();
        return
    };
}

fn repl() {
    let inp = io::stdin();
    let mut inp = inp.lock();

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();
        let mut buff = String::new();
        match inp.read_line(&mut buff) {
            Ok(_) => {
                match yard::evaluate::<f64>(&buff) {
                    Ok(v) => println!("{}", v),
                    Err(e) => println!("{}", e),
                }
            },
            Err(e) => println!("{}", e),
        };
        buff.clear();
    }
}
