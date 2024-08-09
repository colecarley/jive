use crate::{
    parser::{accept::Accept, expression::Expression},
    visitors::Visitor,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct IfExpression {
    pub condition: Expression,
    pub then_branch: Expression,
    pub else_branch: Expression,
}

impl Accept for IfExpression {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_if_expression(self)
    }
}
