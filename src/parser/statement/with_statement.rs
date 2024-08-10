use crate::{
    parser::{accept::Accept, expression::Expression},
    token::Token,
    visitors::Visitor,
};

use super::Statement;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct With {
    pub value: Expression,
    pub identifier: Token,
    pub body: Statement,
}

impl Accept for With {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_with_statement(self)
    }
}
