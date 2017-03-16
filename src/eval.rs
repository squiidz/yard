use token::RPNToken;

pub fn eval(tokens: Vec<RPNToken>) -> i32 {
    let mut stack: Vec<i32> = Vec::new();
    for t in tokens {
        match t.value {
            '+' => {
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
