use super::Expression;
use crate::{parser::accept::Accept, visitors::Visitor};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct IndexAssignment {
    pub list: Expression,
    pub expression: Expression,
    pub value: Expression,
}

impl Accept for IndexAssignment {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_index_assignment(self)
    }
}
