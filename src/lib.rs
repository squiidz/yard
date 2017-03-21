//! Evaluate arithmetic operations of a string,
//! based on the shunting yard algorithm.
pub mod token;
pub mod parser;
pub mod evaluator;

use parser::parse;
use evaluator::eval;

/// evaluate is a wrapper reducing the amount of code needed to process a string.
/// #Example
/// ```rust
///
/// fn main() {
///     let code = "3 + 4";
///     if let Ok(total) = yard::evaluate(&code) {
///         println!("{}", total);
///     }
/// }
/// ```
pub fn evaluate(code: &str) -> Result<i32, String> {
    match parse(code) {
        Ok(tokens) => Ok(eval(&tokens)),
        Err(e) => Err(e),
    }
}
