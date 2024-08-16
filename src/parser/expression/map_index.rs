use super::Expression;
use crate::{parser::accept::Accept, token::Token, visitors::Visitor};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MapIndex {
    pub map: Expression,
    pub key: Token,
}

impl Accept for MapIndex {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_map_index(self)
    }
}
