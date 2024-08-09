use crate::visitors::Visitor;

use super::accept::Accept;

pub mod block;
pub mod declaration;
pub mod expression_statement;
pub mod print;

pub use block::Block;
pub use declaration::DeclarationStatement;
pub use expression_statement::ExpressionStatement;
pub use print::PrintStatement;

pub enum Statement {
    ExpressionStatement(Box<ExpressionStatement>),
    PrintStatement(Box<PrintStatement>),
    DeclarationStatement(Box<DeclarationStatement>),
    Block(Box<Block>),
}

impl Accept for Statement {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        match self {
            Statement::ExpressionStatement(expression_statement) => {
                visitor.visit_expression_statement(expression_statement)
            }
            Statement::PrintStatement(print_statement) => {
                visitor.visit_print_statement(print_statement)
            }
            Statement::DeclarationStatement(declaration_statement) => {
                visitor.visit_declaration_statement(declaration_statement)
            }
            Statement::Block(block) => visitor.visit_block(block),
        }
    }
}
