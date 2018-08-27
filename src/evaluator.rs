use std::str::FromStr;
use num::Num;
use token::{RPNToken, Operator};

/// eval process RPNToken provided by the parser and returned the value of the operation.
/// #Example
/// ```rust
/// extern crate yard;
/// use yard::{parser, evaluator};
///
/// fn main() {
///     let code = "3 + 4";
///     if let Ok(tokens) = parser::parse(&code) {
///         let result = evaluator::eval::<i32>(&tokens);
///         println!("{}", result);
///     }
/// }
/// ```
/// for normal usage, evaluate should be use instead.
pub fn eval<T: Num + FromStr + Clone + >(tokens: &[RPNToken<T>]) -> T {
    let mut stack: Vec<T> = Vec::new();
    for t in tokens {
        match t {
            RPNToken::Operator(Operator::PLUS) => {
                let n1 = stack.pop().expect("Unbalanced addition");
                let n2 = stack.pop().expect("Unbalanced addition");
                stack.push(n2 + n1);
            },
            RPNToken::Operator(Operator::MINUS) => {
                let n1 = stack.pop().expect("Unbalanced substraction");
                let n2 = stack.pop().expect("Unbalanced substraction");
                stack.push(n2 - n1);
            },
            RPNToken::Operator(Operator::MULTIPLY) => {
                let n1 = stack.pop().expect("Unbalanced multiplication");
                let n2 = stack.pop().expect("Unbalanced multiplication");
                stack.push(n2 * n1);
            },
            RPNToken::Operator(Operator::DIVIDE) => {
                let n1 = stack.pop().expect("Unbalanced division");
                let n2 = stack.pop().expect("Unbalanced division");
                stack.push(n2 / n1);
            },
            RPNToken::Operator(Operator::POW) => {
                let _n1 = stack.pop().expect("Unbalanced power");
                let _n2 = stack.pop().expect("Unbalanced power");
                // TODO: add pow with T 
                // stack.push(pow::<T>(n2, n1));
            }
            RPNToken::Operator(Operator::LPAREN) => panic!("Stray ( in eval"),
            RPNToken::Operator(Operator::RPAREN) => panic!("Stray ) in eval"),
            RPNToken::Operand(v) => stack.push(v.clone()),
        }
    }

    stack.last().unwrap().clone()
}
