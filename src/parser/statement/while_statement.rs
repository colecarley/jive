use crate::{
    parser::{accept::Accept, expression::Expression},
    visitors::Visitor,
};

use super::Statement;

pub struct WhileStatement {
    pub condition: Expression,
    pub body: Statement,
}

impl Accept for WhileStatement {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_while_statement(self)
    }
}
