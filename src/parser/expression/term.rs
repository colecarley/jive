use super::Expression;
use crate::{parser::accept::Accept, token::Token, visitors::Visitor};

pub struct Term {
    pub left: Expression,
    pub operator: Token,
    pub right: Expression,
}

impl Term {
    pub fn new(left: Expression, operator: Token, right: Expression) -> Term {
        Term {
            left,
            operator,
            right,
        }
    }
}

impl Accept for Term {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_term(self)
    }
}
