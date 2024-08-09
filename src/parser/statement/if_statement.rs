use crate::{
    parser::{accept::Accept, expression::Expression},
    visitors::Visitor,
};

use super::Block;

pub struct IfStatement {
    pub condition: Expression,
    pub then_branch: Box<Block>,
    pub else_branch: Option<Box<Block>>,
}

impl Accept for IfStatement {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_if_statement(self)
    }
}
