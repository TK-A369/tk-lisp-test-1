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
    sexpr: &parser::SExpr,
    ctx: &mut EvalContext,
) -> Result<parser::SExpr, Box<dyn std::error::Error>> {
    match sexpr {
        parser::SExpr::Atom(parser::Atom::Number(num)) => {
            Ok(parser::SExpr::Atom(parser::Atom::Number(*num)))
        }
        parser::SExpr::Atom(parser::Atom::Symbol(sym)) => {
            if ctx.vars.len() > 0 {
                let mut curr_var: usize = ctx.vars.len() - 1;
                loop {
                    if &ctx.vars[curr_var].name == sym {
                        break Ok(ctx.vars[curr_var].value.clone());
                    }

                    if curr_var == 0 {
                        break Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::NotFound,
                            format!("Variable {} not defined.", sym),
                        )));
                    } else {
                        curr_var -= 1;
                    }
                }
            } else {
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Variable {} not defined.", sym),
                )))
            }
        }
        parser::SExpr::List(list) => {
            if list.len() > 0 {
                if let parser::SExpr::List(_) = list[0] {
                    // This is a list of lists
                    // Evaluate all elements and return the last one
                    let mut ret_val: Option<parser::SExpr> = None;
                    for elem in list {
                        ret_val = Some(eval(elem, ctx)?);
                    }
                    Ok(ret_val.unwrap())
                } else {
                    // The first element of list is an atom
                    match &list[0] {
                        parser::SExpr::Atom(parser::Atom::Symbol(sym)) => match sym.as_str() {
                            "let" => {
                                if list.len() >= 3 {
                                    let mut ctx_new = ctx.clone();
                                    for i in 0..(list.len() - 2) {
                                        if let parser::SExpr::List(var_def_list) = &list[i + 1] {
                                            if let parser::SExpr::Atom(parser::Atom::Symbol(
                                                var_name,
                                            )) = var_def_list[0].clone()
                                            {
                                                let value_evaluated: parser::SExpr =
                                                    eval(&var_def_list[1], &mut ctx_new)?;
                                                ctx_new.vars.push(Variable {
                                                    name: var_name,
                                                    value: value_evaluated,
                                                });
                                            } else {
                                                return Err(Box::new(std::io::Error::new(
                                                    std::io::ErrorKind::InvalidInput,
                                                    "1st element of `let` variable definition must be symbol - variable name.",
										        )));
                                            }
                                        } else {
                                            return Err(Box::new(std::io::Error::new(
                                                std::io::ErrorKind::InvalidInput,
                                                "Arguments (besides first and last) of statement list `let` must be list of variable name and value.",
                                            )));
                                        }
                                    }
                                    eval(&list[list.len() - 1], &mut ctx_new)
                                } else {
                                    Err(Box::new(std::io::Error::new(
										std::io::ErrorKind::InvalidInput,
										"Statement list `let` must have at least 3 elements: `let`, (var_name, var_value)+, block.",
									)))
                                }
                            }
                            "set" => {
                                if list.len() == 3 {
                                    if let parser::SExpr::Atom(parser::Atom::Symbol(var_name)) =
                                        &list[1]
                                    {
                                        let value_evaluated: parser::SExpr = eval(&list[2], ctx)?;
                                        let mut curr_var: usize = ctx.vars.len() - 1;
                                        if let Err(e) = loop {
                                            if &ctx.vars[curr_var].name == var_name {
                                                ctx.vars[curr_var].value = value_evaluated.clone();
                                                break Ok(());
                                            }

                                            if curr_var == 0 {
                                                break Err(Box::new(std::io::Error::new(
                                                    std::io::ErrorKind::NotFound,
                                                    format!("Variable {} not defined.", var_name),
                                                )));
                                            } else {
                                                curr_var -= 1;
                                            }
                                        } {
                                            return Err(e);
                                        }

                                        Ok(value_evaluated)
                                    } else {
                                        Err(Box::new(std::io::Error::new(
                                                std::io::ErrorKind::InvalidInput,
                                                "Arguments (besides first and last) of statement list `let` must be list of variable name and value.",
                                            )))
                                    }
                                } else {
                                    Err(Box::new(std::io::Error::new(
										std::io::ErrorKind::InvalidInput,
										"Statement list `let` must have at least 3 elements: `let`, (var_name, var_value)+, block.",
									)))
                                }
                            }
                            "print" => {
                                if list.len() == 2 {
                                    let result = eval(&list[1], ctx)?;
                                    if let parser::SExpr::Atom(atom) = &result {
                                        if let parser::Atom::Number(num) = atom {
                                            println!("{}", num);
                                            Ok(result)
                                        } else {
                                            Err(Box::new(std::io::Error::new(
												std::io::ErrorKind::InvalidInput,
												"2nd argument to statement list `print` must evaluate to a number atom.",
											)))
                                        }
                                    } else {
                                        Err(Box::new(std::io::Error::new(
											std::io::ErrorKind::InvalidInput,
											"2nd argument to statement list `print` must evaluate to a number atom.",
										)))
                                    }
                                } else {
                                    Err(Box::new(std::io::Error::new(
										std::io::ErrorKind::InvalidInput,
										"Statement list `print` must have exactly 2 elements: `print`, value.",
									)))
                                }
                            }
                            "+" => {
                                if list.len() > 2 {
                                    let mut result: f64 = 0.0;
                                    for i in 1..list.len() {
                                        let elem_result = eval(&list[i], ctx)?;
                                        if let parser::SExpr::Atom(parser::Atom::Number(num)) =
                                            elem_result
                                        {
                                            result += num;
                                        } else {
                                            return Err(Box::new(std::io::Error::new(
                                                std::io::ErrorKind::InvalidInput,
                                                "2+nd argument of statement list `+` must evaluate to a number atom.",
                                            )));
                                        }
                                    }
                                    Ok(parser::SExpr::Atom(parser::Atom::Number(result)))
                                } else {
                                    Err(Box::new(std::io::Error::new(
										std::io::ErrorKind::InvalidInput,
										"Statement list `+` must have more than 2 elements: `+`, values... .",
									)))
                                }
                            }
                            statement => Err(Box::new(std::io::Error::new(
                                std::io::ErrorKind::InvalidInput,
                                format!("Bad statement `{}`.", statement),
                            ))),
                        },
                        parser::SExpr::Atom(parser::Atom::Number(_)) => {
                            Err(Box::new(std::io::Error::new(
                                std::io::ErrorKind::InvalidInput,
                                "Cannot evaluate list whose first element is a number.",
                            )))
                        }
                        parser::SExpr::List(_) => unreachable!(),
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
