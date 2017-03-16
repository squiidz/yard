#[derive(Debug, Clone)]
pub enum RPNTokenType {
    Operand,
    Operator,
}

#[derive(Debug, Clone)]
pub struct RPNToken {
    token_type: RPNTokenType,
    pub value: char,
}

impl RPNToken {
    pub fn new(tt: RPNTokenType, val: char) -> RPNToken {
        RPNToken{
            token_type: tt,
            value: val,
        }
    }
}

pub enum Operator {
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    POW,
    LPAREN,
    RPAREN,
    INVALID,
}

impl Operator {
    pub fn value(&self) -> u32 {
        match *self {
            Operator::PLUS => { 0 },
            Operator::MINUS => { 0 },
            Operator::MULTIPLY => { 1 },
            Operator::DIVIDE => { 1 },
            Operator::POW => { 2 },
            Operator::LPAREN => { 0 },
            Operator::RPAREN => { 0 },
            _ => { 0 },
        }
    }
}

impl From<char> for Operator {
    fn from(c: char) -> Self {
        match c {
            '+' => Operator::PLUS,
            '-' => Operator::MINUS,
            '*' => Operator::MULTIPLY,
            '/' => Operator::DIVIDE,
            '^' => Operator::POW,
            '(' => Operator::LPAREN,
            ')' => Operator::RPAREN,
            _ => Operator::INVALID,
        }
    }
}
