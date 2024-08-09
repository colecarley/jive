use crate::{
    parser::{accept::Accept, expression::Expression},
    visitors::Visitor,
};

pub struct Cond {
    pub condition: Expression,
    pub then_branch: Expression,
    pub else_branch: Expression,
}

impl Accept for Cond {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_cond(self)
    }
}
