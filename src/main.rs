mod lexer;
mod parser;

fn main() {
    let code1: String = String::from("(print (+ 2 5))");

    match lexer::lex(code1) {
        Ok(tokens) => {
            println!("Tokens: {:?}", tokens);

            match parser::parse(&tokens) {
                Ok(sexpr) => {
                    println!("Parsed code: {:?}", sexpr);
                }
                Err(e) => eprintln!("Parsing error: {}", e),
            }
        }
        Err(e) => eprintln!("Lexing error: {}", e),
    }
}
