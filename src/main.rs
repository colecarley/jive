pub mod lexer;
pub mod parser;
pub mod token;
pub mod visitors;

use lexer::lexer::Lexer;
use parser::parser::Parser;

fn main() {
    let code = "
        print 7 * 2 + 3; 
        make x = 3;
        make y = 4;
        make z = nil;
        print y + x;
        make z;
        print z;
        z = 12;
        print z;
        make r;
        r = 4;
        print r;
        make foo = 4;
        print foo;
        "
    .to_string();

    let mut lexer = Lexer::new(code);
    lexer.lex();
    // for token in lexer.tokens.clone() {
    //     println!("{:?}", token);
    // }

    let mut parser = Parser::new(lexer.tokens);
    let statements = parser.parse();

    // let mut ast_printer = visitors::visitors::AstPrinter::new();
    // ast_printer.print(&statements);

    let mut type_checker = visitors::visitors::TypeChecker::new();
    type_checker.check(&statements);

    let mut interpreter = visitors::visitors::Interpreter::new();
    interpreter.evaluate(&statements);
}
