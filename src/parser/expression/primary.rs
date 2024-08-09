use crate::{parser::accept::Accept, token::Token, visitors::Visitor};

pub struct Primary {
    pub value: Token,
}

impl Primary {
    pub fn new(value: Token) -> Primary {
        Primary { value }
    }
}

impl Accept for Primary {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_primary(self)
    }
}
