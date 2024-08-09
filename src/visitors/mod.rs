use crate::parser::{
    expression::{Assignment, Comparison, Cond, Equality, Factor, Primary, Term, Unary},
    statement::{Block, DeclarationStatement, ExpressionStatement, IfStatement, PrintStatement},
};

pub mod ast_printer;
pub mod environment;
pub mod interpreter;
pub mod type_checker;

pub trait Visitor {
    type Output;

    fn visit_assignment(&mut self, assignment: &Assignment) -> Self::Output;

    fn visit_equality(&mut self, equality: &Equality) -> Self::Output;

    fn visit_comparison(&mut self, comparison: &Comparison) -> Self::Output;

    fn visit_term(&mut self, term: &Term) -> Self::Output;

    fn visit_factor(&mut self, factor: &Factor) -> Self::Output;

    fn visit_unary(&mut self, unary: &Unary) -> Self::Output;

    fn visit_primary(&mut self, primary: &Primary) -> Self::Output;

    fn visit_expression_statement(
        &mut self,
        expression_statement: &ExpressionStatement,
    ) -> Self::Output;

    fn visit_print_statement(&mut self, print_statement: &PrintStatement) -> Self::Output;

    fn visit_declaration_statement(
        &mut self,
        declarion_statement: &DeclarationStatement,
    ) -> Self::Output;

    fn visit_block(&mut self, block: &Block) -> Self::Output;

    fn visit_if_statement(&mut self, if_statement: &IfStatement) -> Self::Output;

    fn visit_cond(&mut self, cond: &Cond) -> Self::Output;
}
