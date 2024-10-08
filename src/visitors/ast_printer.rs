use crate::parser::{
    accept::Accept,
    expression::{
        Assignment, Call, Comparison, Equality, Factor, IfExpression, Index, IndexAssignment, List,
        MapIndex, MapIndexAssignment, Primary, Record, Term, Unary,
    },
    statement::{
        Block, ExpressionStatement, For, IfStatement, PrintStatement, Return, Statement,
        VariableDeclaration, WhileStatement, With,
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
            let statement = statement.accept(self);
            result.push_str(format!("{}\n", statement).as_str());
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

    fn visit_variable_declaration(
        &mut self,
        variable_declaration: &VariableDeclaration,
    ) -> Self::Output {
        let identifier = variable_declaration.identifier.lexeme.clone();
        if let Some(expression) = &variable_declaration.expression {
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

    fn visit_while_statement(&mut self, while_statement: &WhileStatement) -> Self::Output {
        format!(
            "while ({}) do {}",
            while_statement.condition.accept(self),
            while_statement.body.accept(self)
        )
    }

    fn visit_call(&mut self, call: &Call) -> Self::Output {
        format!(
            "{}({})",
            call.identifier.accept(self),
            call.arguments
                .iter()
                .map(|arg| arg.accept(self))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }

    fn visit_function_declaration(
        &mut self,
        function_declaration: &crate::parser::statement::FunctionDeclaration,
    ) -> Self::Output {
        format!(
            "funk {} ({}) {}",
            function_declaration.identifier.lexeme,
            function_declaration
                .parameters
                .iter()
                .map(|param| param.lexeme.clone())
                .collect::<Vec<String>>()
                .join(","),
            function_declaration.body.accept(self),
        )
    }

    fn visit_return(&mut self, return_statement: &Return) -> Self::Output {
        format!(
            "return ({})",
            match &return_statement.value {
                Some(value) => value.accept(self),
                None => String::from(""),
            }
        )
    }

    fn visit_with_statement(&mut self, with_statement: &With) -> Self::Output {
        format!(
            "with {} as ({}) {}",
            with_statement.value.accept(self),
            with_statement.identifier.lexeme,
            with_statement.body.accept(self)
        )
    }

    fn visit_list(&mut self, list: &List) -> Self::Output {
        format!(
            "[{}]",
            list.values
                .iter()
                .map(|v| v.accept(self))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }

    fn visit_for_statement(&mut self, for_statement: &For) -> Self::Output {
        format!(
            "for {} in {} {}",
            for_statement.identifier.lexeme,
            for_statement.iter.accept(self),
            for_statement.body.accept(self)
        )
    }

    fn visit_index(&mut self, index: &Index) -> Self::Output {
        format!(
            "{}[{}]",
            index.list.accept(self),
            index.expression.accept(self)
        )
    }

    fn visit_record(&mut self, record: &Record) -> Self::Output {
        format!(
            "{{ {} }}",
            record
                .key_values
                .iter()
                .map(|kv| format!("{}:{},", kv.0.lexeme, kv.1.accept(self)))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }

    fn visit_map_index(&mut self, map_index: &MapIndex) -> Self::Output {
        format!("{}.{}", map_index.map.accept(self), map_index.key.lexeme)
    }

    fn visit_map_index_assignment(
        &mut self,
        map_index_assignment: &MapIndexAssignment,
    ) -> Self::Output {
        format!(
            "{}.{} = {}",
            map_index_assignment.map.accept(self),
            map_index_assignment.key.lexeme,
            map_index_assignment.value.accept(self)
        )
    }

    fn visit_index_assignment(&mut self, index_assignment: &IndexAssignment) -> Self::Output {
        format!(
            "{}[{}] = {}",
            index_assignment.list.accept(self),
            index_assignment.expression.accept(self),
            index_assignment.value.accept(self)
        )
    }
}
