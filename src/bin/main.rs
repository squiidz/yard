extern crate yard;
use std::env;

fn main() {
    let inp = env::args().collect::<Vec<String>>();
    let code = inp.last().unwrap();
    println!("{}", yard::evaluate(code).unwrap());
}
