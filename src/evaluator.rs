use crate::parser;

#[derive(Clone)]
struct Variable {
    name: String,
    value: parser::SExpr,
}

#[derive(Clone)]
pub struct EvalContext {
    vars: Vec<Variable>,
}

impl EvalContext {
    pub fn new() -> EvalContext {
        EvalContext { vars: Vec::new() }
    }
}

pub fn eval(
    sexpr: parser::SExpr,
    ctx: EvalContext,
) -> Result<parser::SExpr, Box<dyn std::error::Error>> {
    match sexpr {
        parser::SExpr::Atom(parser::Atom::Number(num)) => {
            Ok(parser::SExpr::Atom(parser::Atom::Number(num)))
        }
        parser::SExpr::Atom(parser::Atom::Symbol(sym)) => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Reading variables not implemented yet.",
        ))),
        parser::SExpr::List(list) => {
            if list.len() > 0 {
                if let parser::SExpr::List(_) = list[0] {
                    // This is a list of lists
                    // Evaluate all elements and return the last one
                    let mut ret_val: Option<parser::SExpr> = None;
                    for elem in list {
                        ret_val = Some(eval(elem, ctx.clone())?);
                    }
                    Ok(ret_val.unwrap())
                } else {
                    // The first element of list is an atom
                    match &list[0] {
                        parser::SExpr::Atom(parser::Atom::Symbol(sym)) => match sym.as_str() {
                            "let" => {
                                if list.len() == 4 {
                                    if let parser::SExpr::Atom(parser::Atom::Symbol(var_name)) =
                                        list[1].clone()
                                    {
                                        let mut ctx_new = ctx.clone();
                                        ctx_new.vars.push(Variable {
                                            name: var_name,
                                            value: eval(list[2].clone(), ctx.clone())?,
                                        });
                                        eval(list[3].clone(), ctx_new)
                                    } else {
                                        Err(Box::new(std::io::Error::new(
											std::io::ErrorKind::InvalidInput,
											"2nd argument of statement list `let` must be symbol - variable name.",
										)))
                                    }
                                } else {
                                    Err(Box::new(std::io::Error::new(
										std::io::ErrorKind::InvalidInput,
										"Statement list `let` must have exactly 4 elements: `let`, var_name, var_value, block.",
									)))
                                }
                            }
                            statement => Err(Box::new(std::io::Error::new(
                                std::io::ErrorKind::InvalidInput,
                                format!("Bad statement `{}`.", statement),
                            ))),
                        },
                        parser::SExpr::Atom(parser::Atom::Number(num)) => {
                            Err(Box::new(std::io::Error::new(
                                std::io::ErrorKind::InvalidInput,
                                "Cannot evaluate list whose first element is a number.",
                            )))
                        }
                        parser::SExpr::List(list) => unreachable!(),
                    }
                }
            } else {
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Cannot evaluate empty list.",
                )))
            }
        }
    }
}
