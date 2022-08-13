#![allow(dead_code)]

#[derive(Debug)]
pub struct ParseError(String);

#[derive(Debug, Clone)]
enum Token {
    AtomName(String),
    OpeningBrace(char),
    ClosingBrace(char),
    Number(usize),
}

use Token::*;

pub fn parse_molecule(s: &str) -> Result<Vec<(String, usize)>, ParseError> {
    let tokens = lexical_analysis(s)?;
    let atom_tokens = flatten_atoms(&tokens)?;
    let atoms = atom_tokens
        .iter()
        .map(|t| match t {
            AtomName(a) => Ok(a),
            _ => panic!("Unexpected token, only AtomName is expected"),
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut molecule_sum = Vec::<(String, usize)>::new();

    for atom_name in atoms {
        let sum = molecule_sum.iter_mut().find(|(name, _)| name == atom_name);
        if let Some((_, sum)) = sum {
            *sum += 1;
        } else {
            molecule_sum.push((atom_name.to_owned(), 1));
        }
    }

    Ok(molecule_sum)
}

fn lexical_analysis(s: &str) -> Result<Vec<Token>, ParseError> {
    let mut result = Vec::<Token>::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => {
                result.push(OpeningBrace(c));
            }
            ')' => {
                result.push(ClosingBrace('('));
            }
            ']' => {
                result.push(ClosingBrace('['));
            }
            '}' => {
                result.push(ClosingBrace('{'));
            }
            _ if c.is_numeric() => {
                if let Some(Number(n)) = result.last_mut() {
                    *n = *n * 10 + c.to_digit(10).unwrap() as usize;
                } else {
                    result.push(Number(c.to_digit(10).unwrap() as usize));
                }
            }
            _ if c.is_alphabetic() => {
                if let Some(AtomName(atom_name)) = result.last_mut() {
                    if c.is_uppercase() {
                        result.push(AtomName(c.to_string())); // push new atom
                    } else {
                        atom_name.push(c); // append letter to previous atom
                    }
                } else if c.is_uppercase() {
                    result.push(AtomName(c.to_string())); // push new atom
                } else {
                    return Err(ParseError(
                        "Atom name cannot start with lowercase letter".to_owned(),
                    ));
                }
            }
            _ => return Err(ParseError("Unknown symbol".to_owned())),
        }
    }
    Ok(result)
}

fn flatten_atoms(tokens: &[Token]) -> Result<Vec<Token>, ParseError> {
    match tokens {
        [] => Ok(Vec::new()),
        [Number(_), ..] => Err(ParseError("Number at the beginning".to_owned())),
        [.., OpeningBrace(_)] => Err(ParseError("Opening brace at the end".to_owned())),
        [ClosingBrace(_), ..] => Err(ParseError("Closing brace at the beginning".to_owned())),
        [.., ClosingBrace(t)] => flatten_braces(&tokens[..tokens.len() - 1], *t, 1),
        [.., ClosingBrace(t), Number(n)] => flatten_braces(&tokens[..tokens.len() - 2], *t, *n),
        [.., AtomName(a)] => flatten_trailing_atoms(&tokens[..tokens.len() - 1], a, 1),
        [.., AtomName(a), Number(n)] => flatten_trailing_atoms(&tokens[..tokens.len() - 2], a, *n),
        _ => Err(ParseError(format!("Wrong token order: {:?}", tokens))),
    }
}

fn flatten_braces(
    tokens: &[Token],
    brace_type: char,
    number: usize,
) -> Result<Vec<Token>, ParseError> {
    let index = find_matching_opening_brace(&tokens[..tokens.len()], brace_type)?;
    let mut v = flatten_atoms(&tokens[..index])?;
    let inside_braces = flatten_atoms(&tokens[index + 1..tokens.len()])?;
    for _ in 0..number {
        v.extend_from_slice(&inside_braces);
    }
    Ok(v)
}

fn flatten_trailing_atoms(
    tokens: &[Token],
    atom_name: &String,
    number: usize,
) -> Result<Vec<Token>, ParseError> {
    let mut v = flatten_atoms(&tokens[..tokens.len()])?;
    v.extend_from_slice(&vec![AtomName(atom_name.to_owned()); number]);
    Ok(v)
}

fn find_matching_opening_brace(tokens: &[Token], brace_type: char) -> Result<usize, ParseError> {
    let mut depth = 0;
    for i in (0..tokens.len()).rev() {
        match tokens[i] {
            OpeningBrace(t) if t == brace_type => {
                if depth == 0 {
                    return Ok(i);
                } else {
                    depth -= 1;
                }
            }
            ClosingBrace(t) if t == brace_type => {
                depth += 1;
            }
            _ => {}
        }
    }
    Err(ParseError(format!(
        "Opening brace of type '{}' not found",
        brace_type
    )))
}
