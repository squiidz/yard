pub mod token;
pub mod parser;
pub mod evaluator;

use parser::parse;
use evaluator::eval;

pub fn evaluate(code: &str) -> Result<i32, String> {
    match parse(code) {
        Ok(tokens) => Ok(eval(&tokens)),
        Err(e) => Err(e),
    }
}