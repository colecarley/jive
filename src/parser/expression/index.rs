use super::Expression;
use crate::{parser::accept::Accept, visitors::Visitor};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Index {
    pub list: Expression,
    pub expression: Expression,
}

impl Accept for Index {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_index(self)
    }
}
