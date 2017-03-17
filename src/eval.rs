use token::{RPNToken, Operator};

pub fn eval(tokens: &[RPNToken]) -> i32 {
    let mut stack: Vec<i32> = Vec::new();
    for t in tokens {
        match *t {
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
                let n1 = stack.pop().expect("Unbalanced power");
                let n2 = stack.pop().expect("Unbalanced power");
                stack.push(n2.pow(n1 as u32));
            }
            RPNToken::Operator(Operator::LPAREN) => panic!("Stray ( in eval"),
            RPNToken::Operator(Operator::RPAREN) => panic!("Stray ) in eval"),
            RPNToken::Operator(Operator::INVALID(c)) => panic!("Invalid operator: {}", c),
            RPNToken::Operand(v) => stack.push(v),
        }
    }

    *stack.last().unwrap()
}