use std::fmt;
use BinOp::*;
use Node::*;

pub enum BinOp {
    And,
    Or,
    Xor,
    Impl,
    Leq,
}

pub enum Node {
    Binary {
        op: BinOp,
        left: Box<Node>,
        right: Box<Node>,
    },
    Not(Box<Node>),
    Var(char),
}

impl From<&BinOp> for char {
    fn from(op: &BinOp) -> Self {
        match *op {
            And => '&',
            Or => '|',
            Xor => '^',
            Impl => '>',
            Leq => '=',
        }
    }
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", char::from(self))
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Binary { op, left, right } => write!(f, "{}{}{}", left, right, op),
            Not(operand) => write!(f, "{}!", operand),
            Var(c) => write!(f, "{}", c),
        }
    }
}
