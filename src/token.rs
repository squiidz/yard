use num::Num;

/// RPNToken enum define a Operator and Operand variants.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RPNToken<T: Num + Clone> {
    Operator(Operator),
    Operand(T),
}

/// Operator enum define the allowed operations.
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
    /// return the associated value of an Operator variant.
    pub fn value(&self) -> u32 {
        match *self {
            Operator::LPAREN | Operator::RPAREN => 0,
            Operator::PLUS | Operator::MINUS => 1,
            Operator::MULTIPLY | Operator::DIVIDE => 2,
            Operator::POW => 3,
        }
    }

    /// try to convert a char to an Operator variant.
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
