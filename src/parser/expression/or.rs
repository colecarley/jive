use crate::{
    parser::{accept::Accept, expression::Expression},
    visitors::Visitor,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Or {
    pub left: Expression,
    pub right: Expression,
}

impl Accept for Or {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_or(self)
    }
}
