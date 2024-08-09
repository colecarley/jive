use crate::{
    parser::{accept::Accept, expression::Expression},
    visitors::Visitor,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Return {
    pub value: Option<Expression>,
}

impl Accept for Return {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_return(self)
    }
}
