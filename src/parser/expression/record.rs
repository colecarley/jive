use crate::{
    parser::{accept::Accept, expression::Expression},
    token::Token,
    visitors::Visitor,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Record {
    pub key_values: Vec<(Token, Expression)>,
}

impl Accept for Record {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_record(self)
    }
}
