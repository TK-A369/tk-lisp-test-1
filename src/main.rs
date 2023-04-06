mod evaluator;
mod lexer;
mod parser;

fn main() {
    // let code1: String = String::from("(print (+ 2 5))");
    // let code2: String = String::from("(print (+ (+ 8 2) 5))");
    // let code3: String = String::from(
    //     "
    //     (let
    //         (a 3)
    //         (b 9)
    //         (
    //             (print a)
    //             (print (+ (+ 8 2) 5))
    //             (print (+ a 4))
    //             (+ a b 7)
    //         )
    //     )",
    // );
    // let code4: String = String::from(
    //     "
    //     (let
    //         (a 3)
    //         (b 9)
    //         (
    //             (print a)
    //             (set a 4)
    //             (print a)
    //             (set a b)
    //             (print a)
    //             (set b 7)
    //             (print a)
    //             a
    //         )
    //     )",
    // );
    let code5: String = String::from(
        r#"
        (print "Hello world!\n" 3 " " 6 "\n")"#,
    );

    match lexer::lex(code5) {
        Ok(tokens) => {
            println!("Tokens: {:#?}", tokens);

            match parser::parse(&tokens) {
                Ok(sexpr) => {
                    println!("Parsed code: {:#?}", sexpr);

                    match evaluator::eval(&sexpr, &mut evaluator::EvalContext::new()) {
                        Ok(result) => {
                            println!("Result: {:#?}", result);
                        }
                        Err(e) => eprintln!("Evaluation error: {}", e),
                    }
                }
                Err(e) => eprintln!("Parsing error: {}", e),
            }
        }
        Err(e) => eprintln!("Lexing error: {}", e),
    }
}
