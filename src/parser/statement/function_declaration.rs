use crate::{parser::accept::Accept, token::Token, visitors::Visitor};

use super::Statement;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FunctionDeclaration {
    pub identifier: Token,
    pub parameters: Vec<Token>,
    pub body: Statement,
}

impl Accept for FunctionDeclaration {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_function_declaration(self)
    }
}
