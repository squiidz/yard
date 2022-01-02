//! Evaluate arithmetic operations of a string,
//! based on the shunting yard algorithm.
extern crate num;

pub mod evaluator;
pub mod parser;
pub mod token;

use evaluator::eval;
use num::Num;
use parser::parse;
use std::str::FromStr;

/// evaluate is a wrapper reducing the amount of code needed to process a string.
/// #Example
/// ```rust
///
/// fn example() {
///     let code = "3 + 4";
///     if let Ok(total) = yard::evaluate::<i32>(&code) {
///         println!("{}", total);
///     }
/// }
/// ```
pub fn evaluate<T>(code: &str) -> Result<T, String>
where
    T: Num + FromStr + Clone + Into<f64>,
{
    match parse::<T>(code) {
        Ok(tokens) => Ok(eval::<T>(&tokens)),
        Err(e) => Err(e),
    }
}
