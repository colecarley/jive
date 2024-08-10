mod lexer;
mod parser;
mod token;
mod visitors;

use std::{env, fs};

use lexer::Lexer;
use parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let code = fs::read_to_string(filename).expect("Should have been able to read the file");

    let mut lexer = Lexer::new(code);
    lexer.lex();
    for token in lexer.tokens.iter() {
        println!("{:?}", token);
    }

    let mut parser = Parser::new(lexer.tokens);
    let statements = parser.parse();

    let mut ast_printer = visitors::ast_printer::AstPrinter::new();
    ast_printer.print(&statements);

    let mut type_checker = visitors::type_checker::TypeChecker::new();
    type_checker.check(&statements);

    let mut interpreter = visitors::interpreter::Interpreter::new();
    interpreter.evaluate(&statements);
}
