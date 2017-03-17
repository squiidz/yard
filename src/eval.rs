use token::RPNToken;

pub fn eval(tokens: &[RPNToken]) -> i32 {
    let mut stack: Vec<i32> = Vec::new();
    for t in tokens {
        match t.value {
            '+' => {
                let n1 = stack.pop().expect("Unbalanced addition");
                let n2 = stack.pop().expect("Unbalanced addition");
                stack.push(n1 + n2);
            },
            '-' => {
                let n1 = stack.pop().expect("Unbalanced substraction");
                let n2 = stack.pop().expect("Unbalanced substraction");
                stack.push(n2 - n1);
            },
            '*' => {
                let n1 = stack.pop().expect("Unbalanced multiplication");
                let n2 = stack.pop().expect("Unbalanced multiplication");
                stack.push(n1 * n2);
            },
            '/' => {
                let n1 = stack.pop().expect("Unbalanced division");
                let n2 = stack.pop().expect("Unbalanced division");
                stack.push(n2 / n1);
            },
            '^' => {
                let n1 = stack.pop().expect("Unbalanced power");
                let n2 = stack.pop().expect("Unbalanced power");
                stack.push(n1.pow(n2 as u32));
            }
            v @ _ => stack.push(v.to_digit(10).expect("Unable to convert") as i32),
        }
    }

    *stack.last().unwrap()
}