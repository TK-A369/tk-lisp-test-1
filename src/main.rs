mod lexer;

fn main() {
    let code1: String = String::from("(print (+ 2 5))");

    match lexer::lex(code1) {
        Ok(tokens) => {
            println!("Tokens: {:?}", tokens);
        }
        Err(e) => eprintln!("Lexing error: {}", e),
    }
}
