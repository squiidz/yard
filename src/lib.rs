pub mod token;
pub mod parse;
pub mod eval;

use parse::parse;
use eval::eval;

pub fn evaluate(code: &str) -> Option<i32> {
    match parse(code) {
        Ok(tokens) => Some(eval(tokens)),
        Err(_) => None,
    }
}