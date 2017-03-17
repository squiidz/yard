use token::{Operator, RPNToken};

pub fn parse(code: &str) -> Result<Vec<RPNToken>, String> {
    let tokens = code.chars().filter(|c| !c.is_whitespace());
    let mut output: Vec<RPNToken> = Vec::new();
    let mut stack: Vec<Operator> = Vec::new();
    let mut num: String = String::new();
    let mut neg = true;

    for tok in tokens {
        if tok.is_numeric() {
            num.push(tok);
            neg = false;
        } else {
            if tok == '-' && neg {
                num.push('-');
                neg = false;
                continue;
            }
            if !num.is_empty() {
                let rpnt = RPNToken::Operand(num.parse().expect("Integer out of range"));
                output.push(rpnt);
                num.clear();
            }
            let tokop = Operator::from(tok);

            if tokop == Operator::LPAREN {
                stack.push(tokop);
                neg = true;
            } else if tokop == Operator::RPAREN {
                while let Some(v) = stack.pop() {
                    if v == Operator::LPAREN {
                        break
                    }
                    assert!(v != Operator::RPAREN);
                    output.push(RPNToken::Operator(v));
                }
            } else {
                while {
                    if let Some(&qe) = stack.last() {
                        tokop.value() <= qe.value()
                    } else {
                        false
                    }
                } {
                    output.push(RPNToken::Operator(stack.pop().unwrap()));
                }
                stack.push(tokop);
                neg = true;
            }
        }
    }

    if !num.is_empty() {
        let rpnt = RPNToken::Operand(num.parse().expect("Integer out of range"));
        output.push(rpnt);
        num.clear();
    }

    while let Some(v) = stack.pop() {
        output.push(RPNToken::Operator(v));
    }

    Ok(output)
}
