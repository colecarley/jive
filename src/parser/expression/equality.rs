use super::Expression;
use crate::{parser::accept::Accept, token::Token, visitors::Visitor};

pub struct Equality {
    pub left: Expression,
    pub operator: Token,
    pub right: Expression,
}

impl Equality {
    pub fn new(left: Expression, operator: Token, right: Expression) -> Equality {
        Equality {
            left,
            operator,
            right,
        }
    }
}

impl Accept for Equality {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_equality(self)
    }
}
