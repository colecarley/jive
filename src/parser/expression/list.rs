use crate::{
    parser::{accept::Accept, expression::Expression},
    visitors::Visitor,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct List {
    pub values: Vec<Expression>,
}

impl Accept for List {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_list(self)
    }
}
