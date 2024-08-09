use crate::visitors::Visitor;

use super::accept::Accept;

pub mod block;
pub mod expression;
pub mod function_declaration;
pub mod if_statement;
pub mod print;
pub mod return_statement;
pub mod variable_declaration;
pub mod while_statement;

pub use block::Block;
pub use expression::ExpressionStatement;
pub use function_declaration::FunctionDeclaration;
pub use if_statement::IfStatement;
pub use print::PrintStatement;
pub use return_statement::Return;
pub use variable_declaration::VariableDeclaration;
pub use while_statement::WhileStatement;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Statement {
    ExpressionStatement(Box<ExpressionStatement>),
    PrintStatement(Box<PrintStatement>),
    VariableDeclaration(Box<VariableDeclaration>),
    Block(Box<Block>),
    IfStatement(Box<IfStatement>),
    WhileStatement(Box<WhileStatement>),
    FunctionDeclaration(Box<FunctionDeclaration>),
    Return(Box<Return>),
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
            Statement::VariableDeclaration(variable_declaration) => {
                visitor.visit_variable_declaration(variable_declaration)
            }
            Statement::Block(block) => visitor.visit_block(block),
            Statement::IfStatement(if_statement) => visitor.visit_if_statement(if_statement),
            Statement::WhileStatement(while_statement) => {
                visitor.visit_while_statement(while_statement)
            }
            Statement::FunctionDeclaration(function_declaration) => {
                visitor.visit_function_declaration(function_declaration)
            }
            Statement::Return(return_statement) => visitor.visit_return(return_statement),
        }
    }
}
