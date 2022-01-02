use num::Num;
use std::str::FromStr;
use token::{Operator, RPNToken};

/// eval process RPNToken provided by the parser and returned the value of the operation.
/// #Example
/// ```rust
/// extern crate yard;
/// use yard::{parser, evaluator};
///
/// fn example() {
///     let code = "3 + 4";
///     if let Ok(tokens) = parser::parse(&code) {
///         let result = evaluator::eval::<i32>(&tokens);
///         println!("{}", result);
///     }
/// }
/// ```
/// for normal usage, evaluate should be use instead.
pub fn eval<T>(tokens: &[RPNToken<T>]) -> T
where
    T: Num + FromStr + Clone + Into<f64>,
{
    let mut stack: Vec<T> = Vec::new();
    for t in tokens {
        match t {
            RPNToken::Operator(Operator::PLUS) => {
                let n = pop_stack(&mut stack);
                stack.push(n.1 + n.0);
            }
            RPNToken::Operator(Operator::MINUS) => {
                let n = pop_stack(&mut stack);
                stack.push(n.1 - n.0);
            }
            RPNToken::Operator(Operator::MULTIPLY) => {
                let n = pop_stack(&mut stack);
                stack.push(n.1 * n.0);
            }
            RPNToken::Operator(Operator::DIVIDE) => {
                let n = pop_stack(&mut stack);
                stack.push(n.1 / n.0);
            }
            RPNToken::Operator(Operator::POW) => {
                let n = pop_stack(&mut stack);
                let res = num::pow(n.1, n.0.into() as usize);
                stack.push(res);
            }
            RPNToken::Operator(Operator::LPAREN) => panic!("Stray ( in eval"),
            RPNToken::Operator(Operator::RPAREN) => panic!("Stray ) in eval"),
            RPNToken::Operand(v) => stack.push(v.clone()),
        }
    }

    stack.last().unwrap().clone()
}

fn pop_stack<T: Num + FromStr + Clone + Into<f64>>(stack: &mut Vec<T>) -> (T, T) {
    let n1 = stack.pop().expect("Unbalanced operation");
    let n2 = stack.pop().expect("Unbalanced operation");
    (n1, n2)
}
