mod lexer;
mod parser;
mod token;
mod visitors;

use lexer::Lexer;
use parser::Parser;

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
        make bar;
        print bar;
        {
            print \"Inside block\";
            make x = 6;
            make y = 8;
            print x + y;
            print \"Inside block\";
        }
        print \"Outside block\";
        print x + y;

        if true 
            print \"Inside if\";
        else 
            print \"Inside else\";
        

        if false
            print \"Inside if\";
        else 
            print \"Inside else\";
        

        if 1 < 2
            print \"Inside if\";
        else 
            print \"Inside else\";

        if 1 > 2 
            print \"Inside if\";
        else 
            print \"Inside else\";

        if true print \"Inside if\";
        

        print \"Outside if\";

        if true {
            print \"Inside if x is\";
            print x;

            if true {
                print \"Inside nested if, making x = 68\";
                make x = 68;
                if true {
                    print \"Inside nested nested if, making x = 69\";
                    make x = 69;
                }
                print \"Outside nested nested if x is\";
                print x;
            }

            print \"Outside nested if x is\";
            print x;
        }

        print foo;
        print bar;


        make foo = true;
        make foobar = x if foo else y;
        print foobar;

        make foobar = 1 if false else 2 if true else 3;
        print \"printing foobar\";
        print foobar;
        make x = y = z = 1;

        print \"printing x, y, z\";

        print x;
        print y;
        print z;

        print true and true;
        print true and false;
        print true or false;
        print false or false;
        print !true;

        {
        make x = 0;
            while x < 10 {
                print x;
                x = x + 1;
            }
        }
        "
    .to_string();

    let mut lexer = Lexer::new(code);
    lexer.lex();
    // for token in lexer.tokens.clone() {
    //     println!("{:?}", token);
    // }

    let mut parser = Parser::new(lexer.tokens);
    let statements = parser.parse();

    let mut ast_printer = visitors::ast_printer::AstPrinter::new();
    ast_printer.print(&statements);

    let mut type_checker = visitors::type_checker::TypeChecker::new();
    type_checker.check(&statements);

    let mut interpreter = visitors::interpreter::Interpreter::new();
    interpreter.evaluate(&statements);
}
