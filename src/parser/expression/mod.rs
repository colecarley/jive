pub mod assignment;
pub mod comparison;
pub mod equality;
pub mod factor;
pub mod if_expression;
pub mod primary;
pub mod term;
pub mod unary;

pub use assignment::Assignment;
pub use comparison::Comparison;
pub use equality::Equality;
pub use factor::Factor;
pub use if_expression::IfExpression;
pub use primary::Primary;
pub use term::Term;
pub use unary::Unary;

use crate::visitors::Visitor;

use super::accept::Accept;

pub enum Expression {
    Equality(Box<Equality>),
    Assignment(Box<Assignment>),
    IfExpression(Box<IfExpression>),
    Comparison(Box<Comparison>),
    Term(Box<Term>),
    Factor(Box<Factor>),
    Unary(Box<Unary>),
    Primary(Box<Primary>),
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
        }
    }
}
