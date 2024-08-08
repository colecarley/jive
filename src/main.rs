pub mod lexer;
pub mod parser;
pub mod token;
pub mod visitors;

use lexer::lexer::Lexer;
use parser::parser::Parser;

fn main() {
    let code = "print 7 * 2 + 3; 
        "
    .to_string();

    let mut lexer = Lexer::new(code);

    lexer.lex();
    for token in lexer.tokens.clone() {
        println!("{:?}", token);
    }

    let mut parser = Parser::new(lexer.tokens);

    let statements = parser.parse();

    let ast_printer = visitors::visitors::AstPrinter::new();

    ast_printer.print(&statements);

    let type_checker = visitors::visitors::TypeChecker::new();

    type_checker.check(&statements);

    let interpreter = visitors::visitors::Interpreter::new();

    interpreter.evaluate(&statements);
}
