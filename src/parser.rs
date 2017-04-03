use std::collections::HashMap;
use std::sync::Mutex;
use std::error::Error;

use token::{Operator, RPNToken};
use evaluator::eval;

lazy_static! {
    static ref VARS: Mutex<HashMap<String, String>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}

/// parse try to convert char into RPNToken
/// it strip whitespace and support negative operations.
pub fn parse(code: &str) -> Result<Vec<RPNToken>, String> {
    let tokens: Vec<char> = code.chars().filter(|c| !c.is_whitespace()).collect();
    let mut output: Vec<RPNToken> = Vec::new();
    let mut stack: Vec<Operator> = Vec::new();
    let mut num: String = String::new();
    let mut neg = true;
    let mut i = 0;

    while i < tokens.len() {
        let tok: char = tokens[i];
        if tok.is_numeric() {
            num.push(tok);
            neg = false;
        } else {
            if tok == '-' && neg {
                num.push('-');
                neg = false;
                i += 1;
                continue
            } else if tok.is_alphabetic() {
                let mut iden = String::new();
                let length = match read_string(&tokens[i..], &mut iden) {
                    Some(n) => n,
                    None => {
                        println!("Invalid Identifier");
                        break
                    },
                };
                let new_idx = i + length;
                if tokens[new_idx] == '=' {
                    let mut value = String::new();
                    let mut num_len = 1;
                    for (k, n) in tokens[(new_idx + 1)..].iter().enumerate() {
                        if n.is_numeric() {
                            value.push(*n);
                            num_len += 1;
                        } else if let Some(_) = Operator::try_from_char(*n) {
                            value.push(*n);
                            num_len += 1;
                        } else {
                            break
                        }
                    }
                    let mut variables = VARS.lock().unwrap();
                    variables.insert(iden, value);
                    i += length + num_len + 1;
                    //println!("{:?}", VARS);
                    continue
                } else if tok.is_alphabetic() {
                    let mut iden = String::new();
                    let length = match read_string(&tokens[i..], &mut iden) {
                        Some(n) => n,
                        None => {
                            println!("Invalid Identifier");
                            break
                        },
                    };
                    let variables = VARS.lock().unwrap();
                    match variables.get(&iden) {
                        Some(val) => {
                            match parse(val) {
                                Ok(tok) => {
                                    let rpnt = RPNToken::Operand(eval(&tok));
                                    output.push(rpnt);
                                    i += length;
                                    continue
                                },
                                Err(_) => break,
                            }
                        },
                        None => { println!("Invalid variable"); }, 
                    };
                }
            }
            if !num.is_empty() {
                let rpnt = RPNToken::Operand(num.parse::<i64>().map_err(|err| err.description().to_string())?);
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
                        assert_ne!(v, Operator::RPAREN);
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
        i += 1;
    }

    if !num.is_empty() {
        let rpnt = RPNToken::Operand(num.parse::<i64>().map_err(|err| err.description().to_string())?);
        output.push(rpnt);
    }

    while let Some(v) = stack.pop() {
        output.push(RPNToken::Operator(v));
    }

    Ok(output)
}

fn read_string(src: &[char], buffer: &mut String) -> Option<usize> {
    let mut read_counter: usize = 0;
    for c in src {
        if c.is_alphabetic() {
            buffer.push(*c);
            read_counter += 1;
        } else {
            break;
        }
    }
    Some(read_counter)
}
