extern crate yard;
use std::env;

fn main() {
    let inp = env::args().collect::<Vec<String>>();
    if inp.len() >= 2 {
        let code = inp.last().unwrap();
        match yard::evaluate(code) {
            Ok(v) => println!("{}", v),
            Err(e) => println!("{}", e),
        }
    } else {
        println!("Equation arguments required.");
    }
}
