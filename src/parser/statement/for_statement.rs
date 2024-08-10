use crate::{
    parser::{accept::Accept, expression::Expression},
    token::Token,
    visitors::Visitor,
};

use super::Statement;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct For {
    pub identifier: Token,
    pub iter: Expression,
    pub body: Statement,
}

impl Accept for For {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_for_statement(self)
    }
}
