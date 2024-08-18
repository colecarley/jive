pub mod and;
pub mod assignment;
pub mod call;
pub mod comparison;
pub mod equality;
pub mod factor;
pub mod if_expression;
pub mod index;
pub mod index_assignment;
pub mod list;
pub mod map_index;
pub mod map_index_assignment;
pub mod or;
pub mod primary;
pub mod record;
pub mod term;
pub mod unary;

pub use and::And;
pub use assignment::Assignment;
pub use call::Call;
pub use comparison::Comparison;
pub use equality::Equality;
pub use factor::Factor;
pub use if_expression::IfExpression;
pub use index::Index;
pub use index_assignment::IndexAssignment;
pub use list::List;
pub use map_index::MapIndex;
pub use map_index_assignment::MapIndexAssignment;
pub use or::Or;
pub use primary::Primary;
pub use record::Record;
pub use term::Term;
pub use unary::Unary;

use crate::visitors::Visitor;

use super::accept::Accept;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expression {
    Equality(Box<Equality>),
    Assignment(Box<Assignment>),
    IfExpression(Box<IfExpression>),
    Comparison(Box<Comparison>),
    Term(Box<Term>),
    Factor(Box<Factor>),
    Unary(Box<Unary>),
    Primary(Box<Primary>),
    Or(Box<Or>),
    And(Box<And>),
    Call(Box<Call>),
    List(Box<List>),
    Index(Box<Index>),
    Record(Box<Record>),
    MapIndex(Box<MapIndex>),
    MapIndexAssignment(Box<MapIndexAssignment>),
    IndexAssignment(Box<IndexAssignment>),
}

impl Accept for Expression {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        match self {
            Expression::Assignment(assignment) => assignment.accept(visitor),
            Expression::Equality(equality) => equality.accept(visitor),
            Expression::Comparison(comparison) => comparison.accept(visitor),
            Expression::Term(term) => term.accept(visitor),
            Expression::Factor(factor) => factor.accept(visitor),
            Expression::Unary(unary) => unary.accept(visitor),
            Expression::Primary(primary) => primary.accept(visitor),
            Expression::IfExpression(cond) => cond.accept(visitor),
            Expression::Or(or) => or.accept(visitor),
            Expression::And(and) => and.accept(visitor),
            Expression::Call(call) => call.accept(visitor),
            Expression::List(list) => list.accept(visitor),
            Expression::Index(index) => index.accept(visitor),
            Expression::Record(record) => record.accept(visitor),
            Expression::MapIndex(map_index) => map_index.accept(visitor),
            Expression::MapIndexAssignment(map_index_assignment) => {
                map_index_assignment.accept(visitor)
            }
            Expression::IndexAssignment(index_assignment) => index_assignment.accept(visitor),
        }
    }
}
