use token::{Operator, RPNToken, RPNTokenType};

pub fn parse(code: &str) -> Result<Vec<RPNToken>, String> {
    let tokens: Vec<char> = code.chars().filter(|c| !c.is_whitespace()).collect();
    let mut output: Vec<RPNToken> = Vec::new();
    let mut queue: Vec<RPNToken> = Vec::new();
    let mut paren = false;

    for tok in tokens {
        if tok.is_numeric() {
            let rpnt = RPNToken::new(RPNTokenType::Operand, tok);
            output.push(rpnt)
        }else {
            let rpnt = RPNToken::new(RPNTokenType::Operator, tok);
            if tok == '(' {
                paren = true;
            }
            let qe = match queue.clone().last() {
                Some(v) => { v.value },
                None => {
                    queue.push(rpnt);
                    continue
                },
            };

            if Operator::from(tok).value() > Operator::from(qe).value() {
                queue.push(rpnt)
            } else if !paren {
                output.push(queue.pop().unwrap());
                queue.push(rpnt)
            } else {
                if tok == ')' {
                    for t in queue.clone() {
                        if t.value != '(' {
                            let v = queue.pop().unwrap();
                            if v.value != '(' || v.value != ')' {
                                output.push(v);
                            }
                        }
                    }
                    queue.pop();
                    paren = false;
                } else if rpnt.value != '(' {
                    queue.push(rpnt);
                }
            }
        }
    }

    for _ in 0..queue.len() {
        output.push(queue.pop().unwrap());
    }

    Ok(output)
}
