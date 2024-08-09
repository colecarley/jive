use crate::visitors::Visitor;

pub trait Accept {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output;
}
