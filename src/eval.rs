use std::char;
use token::{RPNToken, RPNTokenType};

pub fn eval(tokens: Vec<RPNToken>) -> i32 {
    let mut stack: Vec<i32> = Vec::new();
    for t in tokens {
        match t.value {
            '+' => {
                //let n1 = stack.pop().expect("Not able to pop").value.to_digit(10).expect("Not able to convert to digit");
                //let n2 = stack.pop().expect("Not able to pop").value.to_digit(10).expect("Not able to convert to digit");
                //let res = n1 + n2;
                //stack.push(RPNToken::new(RPNTokenType::Operand, char::from_digit(res, 10).unwrap()));
                let n1 = stack.pop().expect("Unable to pop");
                let n2 = stack.pop().expect("Unable to pop");
                stack.push(n1 + n2);
            },
            '-' => {
                let n1 = stack.pop().expect("Unable to pop");
                let n2 = stack.pop().expect("Unable to pop");
                stack.push(n2 - n1);
            },
            '*' => {
                let n1 = stack.pop().expect("Unable to pop");
                let n2 = stack.pop().expect("Unable to pop");
                stack.push(n1 * n2);
            },
            '/' => {
                let n1 = stack.pop().expect("Unable to pop");
                let n2 = stack.pop().expect("Unable to pop");
                stack.push(n2 / n1);
            },
            '^' => {
                let n1 = stack.pop().expect("Unable to pop");
                let n2 = stack.pop().expect("Unable to pop");
                stack.push(n1.pow(n2 as u32));
            }
            v @ _ => stack.push(v.to_digit(10).expect("Unable to convert") as i32),
        }
    }

    *stack.last().unwrap()
}
