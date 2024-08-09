use super::Expression;
use crate::{parser::accept::Accept, token::Token, visitors::Visitor};
pub struct Assignment {
    pub identifier: Token,
    pub value: Expression,
}

impl Accept for Assignment {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_assignment(self)
    }
}
