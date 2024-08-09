use super::Expression;
use crate::{parser::accept::Accept, token::Token, visitors::Visitor};

pub struct Comparison {
    pub left: Expression,
    pub operator: Token,
    pub right: Expression,
}

impl Comparison {
    pub fn new(left: Expression, operator: Token, right: Expression) -> Comparison {
        Comparison {
            left,
            operator,
            right,
        }
    }
}

impl Accept for Comparison {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_comparison(self)
    }
}
