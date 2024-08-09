use crate::{
    parser::{accept::Accept, expression::Expression},
    visitors::Visitor,
};

pub struct Or {
    pub left: Expression,
    pub right: Expression,
}

impl Accept for Or {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_or(self)
    }
}
