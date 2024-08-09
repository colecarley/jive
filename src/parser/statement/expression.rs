use crate::{
    parser::{accept::Accept, expression::Expression},
    visitors::Visitor,
};

pub struct ExpressionStatement {
    pub expression: Expression,
}

impl Accept for ExpressionStatement {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_expression_statement(self)
    }
}
