pub mod token;
pub mod parse;
pub mod eval;

use parse::parse;
use eval::eval;

pub fn evaluate(code: &str) -> Result<i32, String> {
    match parse(code) {
        Ok(tokens) => Ok(eval(tokens)),
        Err(e) => Err(e),
    }
}