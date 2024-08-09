use crate::parser::{
    Accept, Assignment, Comparison, DeclarationStatement, Equality, ExpressionStatement, Factor,
    Primary, PrintStatement, Statement, Term, Unary,
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
}
