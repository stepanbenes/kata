// https://www.codewars.com/kata/52a78825cdfc2cfc87000005

#[derive(Debug, Copy, Clone)]
pub enum Token {
    Number(f64),
    UnaryMinus,
    BinaryOperator(char),
    OpenParen,
    CloseParen,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Number(n) => write!(f, "{}", n),
            Token::UnaryMinus => write!(f, "un-"),
            Token::BinaryOperator(op) => write!(f, "{}", op),
            Token::OpenParen => write!(f, "("),
            Token::CloseParen => write!(f, ")"),
        }
    }
}

impl Token {
    fn precedence(&self) -> u8 {
        match self {
            Token::BinaryOperator('+') => 1,
            Token::BinaryOperator('-') => 1,
            Token::BinaryOperator('*') => 2,
            Token::BinaryOperator('/') => 2,
            Token::UnaryMinus => 3,
            _ => 0,
        }
    }
}

fn lexer(input: &str) -> Vec<Token> {
    let mut result = Vec::<Token>::new();
    let mut previous_token: Option<Token> = None;
    let mut chars = input.chars().peekable();
    while let Some(&c) = chars.peek() {
        let token = match c {
            '0'..='9' | '.' => {
                let mut num_string = String::new();
                while let Some(&c) = chars.peek() {
                    if let '0'..='9' | '.' = c {
                        num_string.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if let Ok(number) = num_string.parse::<f64>() {
                    Token::Number(number)
                } else {
                    panic!("Error parsing number: {}", num_string);
                }
            }
            '+' | '*' | '/' => {
                chars.next();
                Token::BinaryOperator(c)
            }
            '-' => {
                chars.next();
                if let Some(Token::Number(_)) | Some(Token::CloseParen) = previous_token {
                    Token::BinaryOperator(c)
                } else {
                    Token::UnaryMinus
                }
            }
            '(' => {
                chars.next();
                Token::OpenParen
            }
            ')' => {
                chars.next();
                Token::CloseParen
            }
            ' ' => {
                chars.next();
                continue;
            }
            _ => panic!("unrecognized token"),
        };
        result.push(token);
        previous_token = Some(token);
    }
    result
}

pub fn to_postfix(infix: &str) -> Vec<Token> {
    let mut stack = Vec::<Token>::new();
    let mut postfix = Vec::<Token>::new();

    for current in lexer(infix) {
        match current {
            Token::Number(_) => postfix.push(current),
            Token::BinaryOperator(_) | Token::UnaryMinus => {
                while let Some(&top_stack_element) = stack.last() {
                    if current.precedence() <= top_stack_element.precedence() {
                        postfix.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                stack.push(current);
            }
            Token::OpenParen => stack.push(current),
            Token::CloseParen => {
                while let Some(&top_stack_element) = stack.last() {
                    if let Token::OpenParen = top_stack_element {
                        break;
                    }
                    postfix.push(stack.pop().unwrap());
                }
                stack.pop();
            } // ignore
        };
    }

    while let Some(x) = stack.pop() {
        postfix.push(x);
    }

    postfix
}

pub fn eval(mut tokens: Vec<Token>) -> f64 {
    fn index_of_first_operator(tokens: &[Token]) -> Option<usize> {
        for (index, item) in tokens.iter().enumerate() {
            match item {
                Token::UnaryMinus | Token::BinaryOperator(_) => return Some(index),
                _ => continue,
            }
        }
        None
    }

    while let Some(index) = index_of_first_operator(&tokens) {
        match tokens.remove(index) {
            Token::UnaryMinus => {
                if index < 1 {
                    panic!("missing operand for unary operator");
                }
                match tokens.remove(index - 1) {
                    Token::Number(value) => tokens.insert(index - 1, Token::Number(-value)),
                    _ => panic!("unary operator can be applied only to number"),
                }
            }
            Token::BinaryOperator(bin_op) => {
                if index < 2 {
                    panic!("missing operands for binary operator");
                }
                match (tokens.remove(index - 1), tokens.remove(index - 2)) {
                    (Token::Number(b), Token::Number(a)) => tokens.insert(
                        index - 2,
                        Token::Number(match bin_op {
                            '+' => a + b,
                            '-' => a - b,
                            '*' => a * b,
                            '/' => a / b,
                            _ => panic!("unknown op code"),
                        }),
                    ),
                    _ => panic!("binary operator can be applied only to number"),
                }
            }
            _ => unreachable!(),
        }
    }
    if let [Token::Number(value)] = tokens[..] {
        value
    } else {
        panic!("expected number at the bottom of the stack")
    }
}

fn main() {
    let input = "((2.33 / (2.9+3.5)*4) - -6)";
    let output: Vec<Token> = to_postfix(input);
    let output_string: String = output
        .iter()
        .map(|token| format!("{token}"))
        .collect::<Vec<_>>()
        .join("|");
    println!("input: {input}; output: {output_string};");
    let value = eval(output);

    println!("value: {value}");
}
