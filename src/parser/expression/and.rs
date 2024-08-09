use crate::{
    parser::{accept::Accept, expression::Expression},
    visitors::Visitor,
};

pub struct And {
    pub left: Expression,
    pub right: Expression,
}

impl Accept for And {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_and(self)
    }
}
