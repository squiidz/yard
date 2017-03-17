use token::{Operator, RPNToken};

pub fn parse(code: &str) -> Result<Vec<RPNToken>, String> {
    let tokens = code.chars().filter(|c| !c.is_whitespace());
    let mut output: Vec<RPNToken> = Vec::new();
    let mut queue: Vec<RPNToken> = Vec::new();
    let mut num: String = String::new();
    let mut paren = false;

    for tok in tokens {
        if tok.is_numeric() {
            num.push(tok);
        } else {
            if !num.is_empty() {
                let rpnt = RPNToken::Operand(num.parse().expect("Integer out of range"));
                output.push(rpnt);
                num.clear();
            }
            let tokop = Operator::from(tok);
            let rpnt = RPNToken::Operator(tokop);
            if tok == '(' {
                paren = true;
            }
            let qe = match queue.last() {
                Some(&RPNToken::Operator(v)) => { v },
                Some(_) => continue,
                None => {
                    queue.push(rpnt);
                    continue
                },
            };

            if tokop.value() > qe.value() {
                queue.push(rpnt)
            } else if !paren {
                output.push(queue.pop().unwrap());
                queue.push(rpnt)
            } else {
                if tokop == Operator::RPAREN {
                    for t in queue.clone() {
                        if t != RPNToken::Operator(Operator::LPAREN) {
                            let v = queue.pop().unwrap();
                            if v != RPNToken::Operator(Operator::LPAREN) &&
                                v != RPNToken::Operator(Operator::RPAREN) {
                                output.push(v);
                            }
                        }
                    }
                    queue.pop();
                    paren = false;
                } else if tokop != Operator::LPAREN {
                    queue.push(rpnt);
                }
            }
        }
    }

    if !num.is_empty() {
        let rpnt = RPNToken::Operand(num.parse().expect("Integer out of range"));
        output.push(rpnt);
        num.clear();
    }

    while let Some(v) = queue.pop() {
        output.push(v);
    }

    Ok(output)
}
