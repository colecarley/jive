use crate::{parser::accept::Accept, visitors::Visitor};

use super::Statement;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Block {
    pub statements: Vec<Statement>,
}

impl Accept for Block {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_block(self)
    }
}
