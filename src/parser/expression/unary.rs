use super::Expression;
use crate::{parser::accept::Accept, token::Token, visitors::Visitor};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Unary {
    pub operator: Token,
    pub right: Expression,
}

impl Unary {
    pub fn new(operator: Token, right: Expression) -> Unary {
        Unary { operator, right }
    }
}
impl Accept for Unary {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_unary(self)
    }
}
