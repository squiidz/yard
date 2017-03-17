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
            Operator::PLUS => 1,
            Operator::MINUS => 1,
            Operator::MULTIPLY => 2,
            Operator::DIVIDE => 2,
            Operator::POW => 3,
            Operator::LPAREN => 0,
            Operator::RPAREN => 0,
            Operator::INVALID(_) => 0,
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
