use crate::lexer;

#[derive(Debug)]
pub enum Atom {
    Number(f64),
    Symbol(String),
}

#[derive(Debug)]
pub enum SExpr {
    Atom(Atom),
    List(Vec<SExpr>),
}

fn parse_expr(
    input: &Vec<lexer::Token>,
    curr_pos: &mut usize,
) -> Result<Option<SExpr>, Box<dyn std::error::Error>> {
    println!("curr_pos={}", *curr_pos);
    let org_pos = *curr_pos;
    if let lexer::Token::LeftParen = input[*curr_pos] {
        *curr_pos += 1;
        let mut list: Vec<SExpr> = Vec::new();
        loop {
            match parse_expr(input, curr_pos) {
                Ok(Some(sexpr)) => {
                    list.push(sexpr);
                }
                Ok(None) => {
                    break;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        if let lexer::Token::RightParen = input[*curr_pos] {
            *curr_pos += 1;
            Ok(Some(SExpr::List(list)))
        } else {
            *curr_pos = org_pos;
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "List not closed by right parenthesis",
            )))
        }
    } else if let lexer::Token::Number(num) = input[*curr_pos] {
        *curr_pos += 1;
        Ok(Some(SExpr::Atom(Atom::Number(num))))
    } else if let lexer::Token::Symbol(sym) = &input[*curr_pos] {
        *curr_pos += 1;
        Ok(Some(SExpr::Atom(Atom::Symbol(sym.clone()))))
    } else {
        Ok(None)
    }
}

pub fn parse(input: &Vec<lexer::Token>) -> Result<SExpr, Box<dyn std::error::Error>> {
    match parse_expr(input, &mut 0) {
        Ok(Some(sexpr)) => Ok(sexpr),
        Ok(None) => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Empty expression",
        ))),
        Err(e) => Err(e),
    }
}
