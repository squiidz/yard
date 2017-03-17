#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RPNToken {
    Operator(Operator),
    Operand(i32),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Operator {
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    POW,
    LPAREN,
    RPAREN,
    INVALID(char),
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
            Operator::INVALID(_) => { 0 },
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
            c => Operator::INVALID(c),
        }
    }
}
