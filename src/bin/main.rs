extern crate yard;

use yard::{parse, eval};
use std::env;


fn main() {
    let inp = env::args().collect::<Vec<String>>();
    match parse(inp.last().unwrap()) {
        Ok(tokens) => {
            //for t in &tokens {
            //    print!("{} ", t.value);
            //}
            println!("{}", eval(tokens));
        },
        Err(_) => return,
    }
}
