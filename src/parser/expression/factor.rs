use super::Expression;
use crate::{parser::accept::Accept, token::Token, visitors::Visitor};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Factor {
    pub left: Expression,
    pub operator: Token,
    pub right: Expression,
}

impl Factor {
    pub fn new(left: Expression, operator: Token, right: Expression) -> Factor {
        Factor {
            left,
            operator,
            right,
        }
    }
}
impl Accept for Factor {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_factor(self)
    }
}
