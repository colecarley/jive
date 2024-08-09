use crate::{
    parser::{accept::Accept, expression::Expression},
    token::Token,
    visitors::Visitor,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VariableDeclaration {
    pub identifier: Token,
    pub expression: Option<Expression>,
}

impl Accept for VariableDeclaration {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_variable_declaration(self)
    }
}
