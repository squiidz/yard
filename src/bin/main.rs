extern crate shunting_yard;

use shunting_yard::{parse, eval};
use std::env;


fn main() {
    let inp = env::args().collect::<Vec<String>>();
    match parse::parse(inp.last().unwrap()) {
        Ok(tokens) => {
            for t in tokens.clone() {
                print!("{} ", t.value);
            }
            println!("\n{}", eval::eval(tokens));
        },
        Err(_) => return,
    }
}
