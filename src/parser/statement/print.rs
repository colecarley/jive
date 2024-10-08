use crate::{
    parser::{accept::Accept, expression::Expression},
    visitors::Visitor,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PrintStatement {
    pub expression: Expression,
}

impl Accept for PrintStatement {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_print_statement(self)
    }
}
