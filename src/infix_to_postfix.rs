// https://www.codewars.com/kata/52e864d1ffb6ac25db00017f

// https://www.geeksforgeeks.org/convert-infix-expression-to-postfix-expression/

trait OperatorProperties {
    fn precedence(&self) -> u8;
    fn asociativity(&self) -> Asociativity;
}

impl OperatorProperties for char {
    fn precedence(&self) -> u8 {
        match self {
            '+' => 1,
            '-' => 1,
            '*' => 2,
            '/' => 2,
            '^' => 3,
            _ => 0,
        }
    }

    fn asociativity(&self) -> Asociativity {
        match self {
            '^' => Asociativity::Right,
            _ => Asociativity::Left,
        }
    }
}

enum Asociativity {
    Left,
    Right,
}

fn check_precedence_and_asociativity(top_stack_element: char, current: char) -> bool {
    if current.precedence() == top_stack_element.precedence() {
        match current.asociativity() {
            Asociativity::Right => true,
            Asociativity::Left => false,
        }
    } else {
        current.precedence() < top_stack_element.precedence()
    }
}

fn to_postfix(infix: &str) -> String {
    let mut stack = Vec::<char>::new();
    let mut postfix = Vec::<char>::new();

    for current in infix.chars() {
        match current {
            '0'..='9' => postfix.push(current),
            '+' | '-' | '*' | '/' | '^' => {
                while let Some(&top_stack_element) = stack.last() {
                    if check_precedence_and_asociativity(top_stack_element, current) {
                        postfix.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                stack.push(current);
            }
            '(' => stack.push(current),
            ')' => {
                while let Some(&top_stack_element) = stack.last() {
                    if top_stack_element == '(' {
                        break;
                    }
                    postfix.push(stack.pop().unwrap());
                }
                stack.pop();
            }
            _ => continue, // ignore
        };
    }

    while let Some(x) = stack.pop() {
        postfix.push(x);
    }

    postfix.into_iter().collect()
}

fn main() {
    let input = "3+4*(2-1)";
    let output = to_postfix(input);
    println!("input: {input}; output: {output}");
}
