use super::Expression;
use crate::{parser::accept::Accept, visitors::Visitor};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Call {
    pub identifier: Expression,
    pub arguments: Vec<Expression>,
}

impl Accept for Call {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_call(self)
    }
}
