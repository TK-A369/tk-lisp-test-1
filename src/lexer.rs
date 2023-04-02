#[derive(Debug)]
pub enum Token {
    LeftParen,
    RightParen,
    Number(f64),
    Symbol(String),
}

pub fn lex(input: String) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
    let mut curr_pos = 0;

    let mut tokens: Vec<Token> = Vec::new();

    while curr_pos < input.len() {
        if input.chars().nth(curr_pos).unwrap() == '(' {
            tokens.push(Token::LeftParen);
            curr_pos += 1;
        } else if input.chars().nth(curr_pos).unwrap() == ')' {
            tokens.push(Token::RightParen);
            curr_pos += 1;
        } else if {
            let ch = input.chars().nth(curr_pos).unwrap();
            ch >= '0' && ch <= '9'
        } {
            let mut buf: String = String::new();
            let mut ch: char;
            while {
                ch = input.chars().nth(curr_pos).unwrap();
                ch >= '0' && ch <= '9'
            } {
                buf.push(ch);
                curr_pos += 1;
            }

            tokens.push(Token::Number(buf.parse::<f64>()?));
        } else if {
            let ch = input.chars().nth(curr_pos).unwrap();
            (ch >= 'a' && ch <= 'z')
                || (ch >= 'A' && ch <= 'Z')
                || ch == '_'
                || ch == '+'
                || ch == '-'
                || ch == '*'
                || ch == '/'
                || ch == '>'
                || ch == '<'
                || ch == '='
        } {
            let mut buf: String = String::new();
            let mut ch: char;
            while {
                ch = input.chars().nth(curr_pos).unwrap();
                (ch >= 'a' && ch <= 'z')
                    || (ch >= 'A' && ch <= 'Z')
                    || ch == '_'
                    || ch == '+'
                    || ch == '-'
                    || ch == '*'
                    || ch == '/'
                    || ch == '>'
                    || ch == '<'
                    || ch == '='
            } {
                buf.push(ch);
                curr_pos += 1;
            }

            tokens.push(Token::Symbol(buf));
        } else {
            curr_pos += 1;
        }
    }

    Ok(tokens)
}
