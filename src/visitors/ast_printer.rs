use crate::parser::{
    accept::Accept,
    expression::{Assignment, Comparison, Equality, Factor, IfExpression, Primary, Term, Unary},
    statement::{
        Block, DeclarationStatement, ExpressionStatement, IfStatement, PrintStatement, Statement,
    },
};

pub struct AstPrinter {}

impl AstPrinter {
    pub fn new() -> Self {
        AstPrinter {}
    }

    pub fn print(&mut self, statements: &Vec<Statement>) {
        let mut result = String::new();
        for statement in statements {
            match statement {
                Statement::ExpressionStatement(expression_statement) => {
                    result.push_str(&expression_statement.accept(self));
                }
                Statement::PrintStatement(print_statement) => {
                    result.push_str(&print_statement.accept(self));
                }
                Statement::DeclarationStatement(declaration_statement) => {
                    result.push_str(&declaration_statement.accept(self));
                }
                Statement::Block(block) => {
                    result.push_str(&block.accept(self));
                }
                Statement::IfStatement(if_statement) => {
                    result.push_str(&if_statement.accept(self));
                }
                Statement::WhileStatement(while_statement) => {
                    result.push_str(&while_statement.accept(self));
                }
            }
            result.push('\n');
        }

        println!("{}", result)
    }
}

impl super::Visitor for AstPrinter {
    type Output = String;

    fn visit_assignment(&mut self, assignment: &Assignment) -> Self::Output {
        format!(
            "{} = {}",
            assignment.identifier.lexeme,
            assignment.value.accept(self)
        )
    }

    fn visit_equality(&mut self, equality: &Equality) -> Self::Output {
        format!(
            "({} == {})",
            equality.left.accept(self),
            equality.right.accept(self)
        )
    }

    fn visit_comparison(&mut self, comparison: &Comparison) -> Self::Output {
        format!(
            "({} {} {})",
            comparison.left.accept(self),
            comparison.operator.lexeme,
            comparison.right.accept(self)
        )
    }

    fn visit_term(&mut self, term: &Term) -> Self::Output {
        format!(
            "({} {} {})",
            term.left.accept(self),
            term.operator.lexeme,
            term.right.accept(self)
        )
    }

    fn visit_factor(&mut self, factor: &Factor) -> Self::Output {
        format!(
            "({} {} {})",
            factor.left.accept(self),
            factor.operator.lexeme,
            factor.right.accept(self)
        )
    }

    fn visit_unary(&mut self, unary: &Unary) -> Self::Output {
        format!("({}{})", unary.operator.lexeme, unary.right.accept(self))
    }

    fn visit_primary(&mut self, primary: &Primary) -> Self::Output {
        primary.value.lexeme.clone()
    }

    fn visit_expression_statement(
        &mut self,
        expression_statement: &ExpressionStatement,
    ) -> Self::Output {
        expression_statement.expression.accept(self)
    }

    fn visit_print_statement(&mut self, print_statement: &PrintStatement) -> Self::Output {
        format!("print ({})", print_statement.expression.accept(self))
    }

    fn visit_declaration_statement(
        &mut self,
        declarion_statement: &DeclarationStatement,
    ) -> Self::Output {
        let identifier = declarion_statement.identifier.lexeme.clone();
        if let Some(expression) = &declarion_statement.expression {
            format!("make {} = {}", identifier, expression.accept(self))
        } else {
            format!("make {}", identifier)
        }
    }

    fn visit_block(&mut self, block: &Block) -> Self::Output {
        let mut result = String::new();
        for statement in &block.statements {
            result.push_str(format!("\t{}", &statement.accept(self)).as_str());
            result.push('\n');
        }

        format!("(\n{})", result)
    }

    fn visit_if_statement(&mut self, if_statement: &IfStatement) -> Self::Output {
        let mut result = String::new();

        let condition = if_statement.condition.accept(self);

        let then_branch = if_statement.then_branch.accept(self);

        if let Some(else_branch) = &if_statement.else_branch {
            let else_branch = else_branch.accept(self);
            result.push_str(
                format!("if {} then {} else {}", condition, then_branch, else_branch).as_str(),
            );
        } else {
            result.push_str(format!("if {} then {}", condition, then_branch).as_str());
        }

        result
    }

    fn visit_if_expression(&mut self, cond: &IfExpression) -> Self::Output {
        format!(
            "if {} then {} else {}",
            cond.condition.accept(self),
            cond.then_branch.accept(self),
            cond.else_branch.accept(self)
        )
    }

    fn visit_and(&mut self, and: &crate::parser::expression::And) -> Self::Output {
        format!("({} and {})", and.left.accept(self), and.right.accept(self))
    }

    fn visit_or(&mut self, or: &crate::parser::expression::Or) -> Self::Output {
        format!("({} or {})", or.left.accept(self), or.right.accept(self))
    }

    fn visit_while_statement(
        &mut self,
        while_statement: &crate::parser::statement::WhileStatement,
    ) -> Self::Output {
        format!(
            "while ({}) do {}",
            while_statement.condition.accept(self),
            while_statement.body.accept(self)
        )
    }
}
