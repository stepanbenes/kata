#![allow(dead_code)]

pub fn diff(expr: &str) -> String {
    let mut input_tokens = Vec::<Token>::new();
    // braces are unnecessary, remove them
    for s in expr.split(&[' ', '(', ')'][..]).rev() {
        if s.is_empty() {
            continue;
        }
        input_tokens.push(if let Ok(n) = s.parse::<f64>() {
            Number(n)
        } else {
            Identifier(s)
        });
    }
    let expression = build_expression(&mut input_tokens);
    let mut diff_expr = diff_expression(expression);
    for _ in 0..get_depth_of(&diff_expr) {
        diff_expr = normalize(diff_expr);
    }
    format!("{}", diff_expr)
}

#[derive(Debug)]
enum Token<'a> {
    Number(f64),
    Identifier(&'a str),
}

#[derive(Clone)]
enum Expression {
    Constant(f64),
    Variable(&'static str),
    UnaryFunction(&'static str, Box<Expression>),
    BinaryFunction(&'static str, Box<Expression>, Box<Expression>),
}

use Expression::*;
use Token::*;

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let text = match self {
            Constant(c) => c.to_string(),
            Variable(name) => name.to_string(),
            UnaryFunction(name, arg) => format!("({} {})", name, arg),
            BinaryFunction(name, left, right) => format!("({} {} {})", name, left, right),
        };
        write!(f, "{}", text)
    }
}

fn build_expression(tokens: &mut Vec<Token>) -> Expression {
    if let Some(token) = tokens.pop() {
        match token {
            Number(n) => Constant(n),
            Identifier("sin") => UnaryFunction("sin", Box::new(build_expression(tokens))),
            Identifier("cos") => UnaryFunction("cos", Box::new(build_expression(tokens))),
            Identifier("tan") => UnaryFunction("tan", Box::new(build_expression(tokens))),
            Identifier("exp") => UnaryFunction("exp", Box::new(build_expression(tokens))),
            Identifier("ln") => UnaryFunction("ln", Box::new(build_expression(tokens))),
            Identifier("+") => BinaryFunction(
                "+",
                Box::new(build_expression(tokens)),
                Box::new(build_expression(tokens)),
            ),
            Identifier("-") => BinaryFunction(
                "-",
                Box::new(build_expression(tokens)),
                Box::new(build_expression(tokens)),
            ),
            Identifier("*") => BinaryFunction(
                "*",
                Box::new(build_expression(tokens)),
                Box::new(build_expression(tokens)),
            ),
            Identifier("/") => BinaryFunction(
                "/",
                Box::new(build_expression(tokens)),
                Box::new(build_expression(tokens)),
            ),
            Identifier("^") => BinaryFunction(
                "^",
                Box::new(build_expression(tokens)),
                Box::new(build_expression(tokens)),
            ),
            Identifier("x") => Variable("x"),
            _ => panic!("{}", format!("unknown token {:?}", token)),
        }
    } else {
        panic!("{}", format!("{:?}", tokens))
    }
}

fn diff_expression(expression: Expression) -> Expression {
    match expression {
        Constant(_) => Constant(0.0),
        Variable(_) => Constant(1.0),
        UnaryFunction("sin", arg) => {
            let arg_diff = diff_expression((*arg).clone());
            BinaryFunction("*", Box::new(arg_diff), Box::new(UnaryFunction("cos", arg)))
        }
        UnaryFunction("cos", arg) => {
            let arg_diff = diff_expression((*arg).clone());
            BinaryFunction(
                "*",
                Box::new(arg_diff),
                Box::new(BinaryFunction(
                    "*",
                    Box::new(Constant(-1.0)),
                    Box::new(UnaryFunction("sin", arg)),
                )),
            )
        }
        UnaryFunction("tan", arg) => {
            let arg_diff = diff_expression((*arg).clone());
            BinaryFunction(
                "*",
                Box::new(arg_diff),
                Box::new(BinaryFunction(
                    "+",
                    Box::new(Constant(1.0)),
                    Box::new(BinaryFunction(
                        "^",
                        Box::new(UnaryFunction("tan", arg)),
                        Box::new(Constant(2.0)),
                    )),
                )),
            )
        }
        UnaryFunction("exp", arg) => {
            let arg_diff = diff_expression((*arg).clone());
            BinaryFunction("*", Box::new(arg_diff), Box::new(UnaryFunction("exp", arg)))
        }
        UnaryFunction("ln", arg) => {
            let arg_diff = diff_expression((*arg).clone());
            BinaryFunction(
                "*",
                Box::new(arg_diff),
                Box::new(BinaryFunction("/", Box::new(Constant(1.0)), arg)),
            )
        }
        BinaryFunction("+", left, right) => BinaryFunction(
            "+",
            Box::new(diff_expression(*left)),
            Box::new(diff_expression(*right)),
        ),
        BinaryFunction("-", left, right) => BinaryFunction(
            "-",
            Box::new(diff_expression(*left)),
            Box::new(diff_expression(*right)),
        ),
        BinaryFunction("*", left, right) => {
            let left_diff = diff_expression((*left).clone());
            let right_diff = diff_expression((*right).clone());
            BinaryFunction(
                "+",
                Box::new(BinaryFunction("*", Box::new(left_diff), right)),
                Box::new(BinaryFunction("*", left, Box::new(right_diff))),
            )
        }
        BinaryFunction("/", left, right) => {
            let left_diff = diff_expression((*left).clone());
            let right_diff = diff_expression((*right).clone());
            BinaryFunction(
                "/",
                Box::new(BinaryFunction(
                    "-",
                    Box::new(BinaryFunction(
                        "*",
                        Box::new(left_diff),
                        Box::new((*right).clone()),
                    )),
                    Box::new(BinaryFunction("*", left, Box::new(right_diff))),
                )),
                Box::new(BinaryFunction(
                    "^",
                    Box::new((*right).clone()),
                    Box::new(Constant(2.0)),
                )),
            )
        }
        BinaryFunction("^", base_arg, exponent_arg) => {
            if let Constant(exponent) = *exponent_arg {
                let base_arg_diff = diff_expression((*base_arg).clone());
                BinaryFunction(
                    "*",
                    Box::new(base_arg_diff),
                    Box::new(BinaryFunction(
                        "*",
                        Box::new(Constant(exponent)),
                        Box::new(BinaryFunction(
                            "^",
                            base_arg,
                            Box::new(Constant(exponent - 1_f64)),
                        )),
                    )),
                )
            } else {
                panic!("Differentiation of exponenciation not supported")
            }
        }
        _ => panic!("pattern for {} not implemented", expression),
    }
}

fn normalize(expression: Expression) -> Expression {
    match expression.clone() {
        Constant(_) => expression,
        Variable(_) => expression,
        UnaryFunction(name, arg) => UnaryFunction(name, Box::new(normalize(*arg))),
        BinaryFunction(name, left, right) => {
            let (left, right) = (*left, *right);
            match (name, left.clone(), right.clone()) {
                ("+", Constant(c1), Constant(c2)) => Constant(c1 + c2),
                ("-", Constant(c1), Constant(c2)) => Constant(c1 - c2),
                ("*", Constant(c1), Constant(c2)) => Constant(c1 * c2),
                ("/", Constant(c1), Constant(c2)) => Constant(c1 / c2),
                ("^", Constant(c1), Constant(c2)) => Constant(c1.powf(c2)),
                ("+", Constant(c), other) | ("+", other, Constant(c)) if c == 0.0 => {
                    normalize(other)
                }
                ("*", Constant(c), _) | ("*", _, Constant(c)) if c == 0.0 => Constant(0.0),
                ("*", Constant(c), other) | ("*", other, Constant(c)) if c == 1.0 => {
                    normalize(other)
                }
                ("^", _, Constant(c)) if c == 0.0 => Constant(1.0),
                ("^", other, Constant(c)) if c == 1.0 => normalize(other),
                _ => BinaryFunction(name, Box::new(normalize(left)), Box::new(normalize(right))),
            }
        }
    }
}

fn get_depth_of(expression: &Expression) -> u32 {
    match expression {
        Constant(_) => 0,
        Variable(_) => 0,
        UnaryFunction(_, arg) => 1 + get_depth_of(arg),
        BinaryFunction(_, left, right) => {
            1 + std::cmp::max(get_depth_of(left), get_depth_of(right))
        }
    }
}
