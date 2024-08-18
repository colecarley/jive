use super::Expression;
use crate::{parser::accept::Accept, token::Token, visitors::Visitor};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MapIndexAssignment {
    pub map: Expression,
    pub key: Token,
    pub value: Expression,
}

impl Accept for MapIndexAssignment {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_map_index_assignment(self)
    }
}
