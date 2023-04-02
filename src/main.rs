mod evaluator;
mod lexer;
mod parser;

fn main() {
    // let code1: String = String::from("(print (+ 2 5))");
    let code1: String = String::from("(print (+ (+ 8 2) 5))");

    match lexer::lex(code1) {
        Ok(tokens) => {
            println!("Tokens: {:?}", tokens);

            match parser::parse(&tokens) {
                Ok(sexpr) => {
                    println!("Parsed code: {:?}", sexpr);

                    match evaluator::eval(sexpr, evaluator::EvalContext::new()) {
                        Ok(result) => {
                            println!("Result: {:?}", result);
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
