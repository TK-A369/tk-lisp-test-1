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
    // let code5: String = String::from(
    //     r#"
    //     (print "Hello world!\n" 3 " " 6 "\n")"#,
    // );
    // let code5: String = String::from(
    //     r#"
    //     (let
    //         (a 4)
    //         (
    //             (print
    //                 (if (>= a 5)
    //                     ((print "yes\n") (+ a 3))
    //                     ((print "no\n") (+ a 8))
    //                 )
    //                 "\n"
    //             )
    //         )
    //     )"#,
    // );
    // let code6: String = String::from(
    //     r#"
    //     (let
    //         (selected 72)
    //         (guess 0)
    //         (counter 0)
    //         (
    //             (while (
    //                 (set guess (readnum))
    //                 (set counter (+ counter 1))
    //                 (if (> guess selected)
    //                 (
    //                     (print "You entered too big number.\n")
    //                     1
    //                 )
    //                 (
    //                     (if (< guess selected)
    //                     (
    //                         (print "You entered too small number.\n")
    //                         1
    //                     )
    //                     (
    //                         (print "Congratulations! You guessed right number.\n")
    //                         0
    //                     ))
    //                 ))
    //             ) ())
    //             (print "You had " counter " guesses.\n")
    //             counter
    //         )
    //     )"#,
    // );
    let code7: String = String::from(
        r#"
        (let
            (myfun1 (lambda () () 
                (
                    (print "Hello, lambda!\n")
                    72
                )))
            (
                (call myfun1)
                (call myfun1)
            )
        )
        "#,
    );

    match lexer::lex(code7) {
        Ok(tokens) => {
            println!("Tokens:\n{:#?}", tokens);

            match parser::parse(&tokens) {
                Ok(sexpr) => {
                    println!("Parsed code:\n{:#?}", sexpr);

                    match evaluator::eval(&sexpr, &mut evaluator::EvalContext::new()) {
                        Ok(result) => {
                            println!("Result:\n{:#?}", result);
                        }
                        Err(e) => eprintln!("Evaluation error:\n{}", e),
                    }
                }
                Err(e) => eprintln!("Parsing error:\n{}", e),
            }
        }
        Err(e) => eprintln!("Lexing error:\n{}", e),
    }
}
