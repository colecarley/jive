use crate::{
    parser::{accept::Accept, expression::Expression},
    token::Token,
    visitors::Visitor,
};

pub struct DeclarationStatement {
    pub identifier: Token,
    pub expression: Option<Expression>,
}

impl Accept for DeclarationStatement {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_declaration_statement(self)
    }
}
