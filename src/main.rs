pub mod lexer;
pub mod parser;
pub mod token;
pub mod visitors;

use lexer::lexer::Lexer;
use parser::parser::{Accept, Parser};

fn main() {
    let mut lexer = Lexer::new("(1 + 2 + 3 + 3.234)".to_string());
    lexer.lex();
    for token in lexer.tokens.clone() {
        println!("{:?}", token);
    }

    let mut parser = Parser::new(lexer.tokens);

    let expression = parser.parse();

    let ast_printer = visitors::visitors::AstPrinter::new();

    let ast_string = expression.accept(ast_printer);

    println!("{}", ast_string);

    let type_checker = visitors::visitors::TypeChecker::new();

    expression.accept(type_checker);

    let interpreter = visitors::visitors::Interpreter::new();

    let result = expression.accept(interpreter);

    println!("{:?}", result);
}
