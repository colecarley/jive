use crate::visitors::Visitor;

use super::accept::Accept;

pub mod block;
pub mod declaration;
pub mod expression;
pub mod if_statement;
pub mod print;
pub mod while_statement;

pub use block::Block;
pub use declaration::DeclarationStatement;
pub use expression::ExpressionStatement;
pub use if_statement::IfStatement;
pub use print::PrintStatement;
pub use while_statement::WhileStatement;

pub enum Statement {
    ExpressionStatement(Box<ExpressionStatement>),
    PrintStatement(Box<PrintStatement>),
    DeclarationStatement(Box<DeclarationStatement>),
    Block(Box<Block>),
    IfStatement(Box<IfStatement>),
    WhileStatement(Box<WhileStatement>),
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
            Statement::IfStatement(if_statement) => visitor.visit_if_statement(if_statement),
            Statement::WhileStatement(while_statement) => {
                visitor.visit_while_statement(while_statement)
            }
        }
    }
}
