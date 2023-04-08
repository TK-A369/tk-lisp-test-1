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

fn value_is_true(value: &parser::SExpr) -> bool {
    if let parser::SExpr::Atom(parser::Atom::Number(num)) = value {
        if *num == 0.0 {
            false
        } else {
            true
        }
    } else if let parser::SExpr::List(cond_list) = value {
        if cond_list.len() == 0 {
            false
        } else {
            true
        }
    } else {
        false
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
                                                "Statement list `set` must have exactly 3 elements: `set`, var_name, var_value.",
                                            )))
                                    }
                                } else {
                                    Err(Box::new(std::io::Error::new(
										std::io::ErrorKind::InvalidInput,
										"Statement list `set` must have exactly 3 elements: `set`, var_name, var_value.",
									)))
                                }
                            }
                            "if" => {
                                let cond_evaluated: parser::SExpr = eval(&list[1], ctx)?;
                                let cond: bool = value_is_true(&cond_evaluated);

                                if list.len() == 3 {
                                    if cond {
                                        Ok(eval(&list[2], ctx)?)
                                    } else {
                                        Ok(parser::SExpr::List(vec![]))
                                    }
                                } else if list.len() == 4 {
                                    if cond {
                                        Ok(eval(&list[2], ctx)?)
                                    } else {
                                        Ok(eval(&list[3], ctx)?)
                                    }
                                } else {
                                    Err(Box::new(std::io::Error::new(
                                        std::io::ErrorKind::InvalidInput,
                                        "Statement list `if` must have 3 or 4 elements: `if`, cond, block1, block2?."
                                    )))
                                }
                            }
                            "while" => {
                                if list.len() == 3 {
                                    let mut result: parser::SExpr = parser::SExpr::List(vec![]);
                                    while {
                                        let cond_evaluated: parser::SExpr = eval(&list[1], ctx)?;
                                        value_is_true(&cond_evaluated)
                                    } {
                                        result = eval(&list[2], ctx)?;
                                    }
                                    Ok(result)
                                } else {
                                    Err(Box::new(std::io::Error::new(
                                        std::io::ErrorKind::InvalidInput,
                                        "Statement list `while` must have 3 elements: `while`, cond, block."
                                    )))
                                }
                            }
                            op @ (">" | "<" | ">=" | "<=" | "=") => {
                                if list.len() == 3 {
                                    let val1 = eval(&list[1], ctx)?;
                                    let val2 = eval(&list[2], ctx)?;

                                    if let parser::SExpr::Atom(parser::Atom::Number(val1_num)) =
                                        val1
                                    {
                                        if let parser::SExpr::Atom(parser::Atom::Number(val2_num)) =
                                            val2
                                        {
                                            let result = match op {
                                                ">" => val1_num > val2_num,
                                                "<" => val1_num < val2_num,
                                                ">=" => val1_num >= val2_num,
                                                "<=" => val1_num <= val2_num,
                                                "=" => val1_num == val2_num,
                                                _ => unreachable!(),
                                            };
                                            if result {
                                                Ok(parser::SExpr::Atom(parser::Atom::Number(1.0)))
                                            } else {
                                                Ok(parser::SExpr::Atom(parser::Atom::Number(0.0)))
                                            }
                                        } else {
                                            Err(Box::new(std::io::Error::new(
                                                std::io::ErrorKind::InvalidInput,
                                                "Comparison statement list must have exactly 3 elements: operator, val1, val2.",
                                            )))
                                        }
                                    } else {
                                        Err(Box::new(std::io::Error::new(
                                            std::io::ErrorKind::InvalidInput,
                                            "Comparison statement list must have exactly 3 elements: operator, val1, val2.",
                                        )))
                                    }
                                } else {
                                    Err(Box::new(std::io::Error::new(
                                        std::io::ErrorKind::InvalidInput,
                                        "Statement list `if` must have 3 or 4 elements: `if`, cond, block1, block2?.",
                                    )))
                                }
                            }
                            "quote" => {
                                if list.len() == 2 {
                                    Ok(list[1].clone())
                                } else {
                                    Err(Box::new(std::io::Error::new(
										std::io::ErrorKind::InvalidInput,
										"Statement list `quote` must have exactly 2 elements: `quote`, value.",
									)))
                                }
                            }
                            "list" => {
                                let mut result: Vec<parser::SExpr> = Vec::new();
                                for i in 1..list.len() {
                                    result.push(eval(&list[i], ctx)?);
                                }
                                Ok(parser::SExpr::List(result))
                            }
                            "print" => {
                                if list.len() >= 2 {
                                    for i in 1..list.len() {
                                        let result = eval(&list[i], ctx)?;
                                        if let parser::SExpr::Atom(atom) = &result {
                                            if let parser::Atom::Number(num) = atom {
                                                print!("{}", num);
                                            } else {
                                                return Err(Box::new(std::io::Error::new(
												    std::io::ErrorKind::InvalidInput,
												    "2+nd argument to statement list `print` must evaluate to a number atom or list of number atoms (char codes).",
											    )));
                                            }
                                        } else if let parser::SExpr::List(print_list) = &result {
                                            for elem in print_list {
                                                if let parser::SExpr::Atom(parser::Atom::Number(
                                                    ch,
                                                )) = elem
                                                {
                                                    print!("{}", (*ch) as u8 as char);
                                                } else {
                                                    return Err(Box::new(std::io::Error::new(
                                                        std::io::ErrorKind::InvalidInput,
                                                        "2+nd argument to statement list `print` must evaluate to a number atom or list of number atoms (char codes).",
                                                    )));
                                                }
                                            }
                                        } else {
                                            return Err(Box::new(std::io::Error::new(
											std::io::ErrorKind::InvalidInput,
											"+2nd argument to statement list `print` must evaluate to a number atom or list of number atoms (char codes).",
										    )));
                                        }
                                    }
                                    Ok(parser::SExpr::List(vec![]))
                                } else {
                                    Err(Box::new(std::io::Error::new(
										std::io::ErrorKind::InvalidInput,
										"Statement list `print` must have at least 2 elements: `print`, value+.",
									)))
                                }
                            }
                            "readnum" => {
                                let mut buf: String = String::new();
                                std::io::stdin().read_line(&mut buf)?;
                                let num = buf.trim().parse::<f64>()?;
                                Ok(parser::SExpr::Atom(parser::Atom::Number(num)))
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
                Ok(parser::SExpr::List(vec![]))
            }
        }
    }
}
