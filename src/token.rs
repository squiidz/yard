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
}

impl Operator {
    pub fn value(&self) -> u32 {
        match *self {
            Operator::LPAREN | Operator::RPAREN => 0,
            Operator::PLUS | Operator::MINUS => 1,
            Operator::MULTIPLY | Operator::DIVIDE => 2,
            Operator::POW => 3,
        }
    }

    // Until std::convert::TryFrom stabilizes
    pub fn try_from_char(c: char) -> Option<Operator> {
        Some(match c {
            '+' => Operator::PLUS,
            '-' => Operator::MINUS,
            '*' => Operator::MULTIPLY,
            '/' => Operator::DIVIDE,
            '^' => Operator::POW,
            '(' => Operator::LPAREN,
            ')' => Operator::RPAREN,
            _ => return None,
        })
    }
}
