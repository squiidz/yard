use std::error::Error;
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
                let rpnt = RPNToken::Operand(num.parse::<i32>().map_err(|err| err.description().to_string())?);
                output.push(rpnt);
                num.clear();
            }

            match Operator::try_from_char(tok) { 
                Some(Operator::LPAREN) => {
                    stack.push(Operator::LPAREN);
                    neg = true;
                },
                Some(Operator::RPAREN) => {
                    while let Some(v) = stack.pop() {
                        if v == Operator::LPAREN {
                            break
                        }
                        assert!(v != Operator::RPAREN);
                        output.push(RPNToken::Operator(v));
                    }
                },
                Some(tokop) => {
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
                },
                None => return Err(format!("Unexpected character: {}", tok)),
            }
        }
    }

    if !num.is_empty() {
        let rpnt = RPNToken::Operand(num.parse::<i32>().map_err(|err| err.description().to_string())?);
        output.push(rpnt);
    }

    while let Some(v) = stack.pop() {
        output.push(RPNToken::Operator(v));
    }

    Ok(output)
}
