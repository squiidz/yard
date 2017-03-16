extern crate yard;

use yard::{parse, eval};
use std::env;


fn main() {
    let inp = env::args().collect::<Vec<String>>();
    match parse(inp.last().expect("[ERROR] need a 1 argument")) {
        Ok(tokens) => {
            println!("{}", eval(tokens));
        },
        Err(_) => return,
    }
}
