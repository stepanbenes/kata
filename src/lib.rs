mod all_balanced_parentheses;
mod differentiation;
mod fluid_volume;
mod molecule_parser;
mod sudoku;
//mod spiralize;
//mod last_digit_of_large_numbers;
//mod dijkstra;
mod infix_to_postfix;
mod eval;
mod char_count;

/// https://www.codewars.com/kata/5208f99aee097e6552000148
pub fn solution(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        if c.is_uppercase() {
            result.push(' ');
        }
        result.push(c);
    }
    result
}

/// https://www.codewars.com/kata/55908aad6620c066bc00002a
pub fn xo(string: &'static str) -> bool {
    // my solution:
    // let x_count = string
    //     .chars()
    //     .map(|c| c.to_ascii_lowercase())
    //     .filter(|c| *c == 'x')
    //     .count();
    // let o_count = string
    //     .chars()
    //     .map(|c| c.to_ascii_lowercase())
    //     .filter(|c| *c == 'o')
    //     .count();
    // x_count == o_count

    // better solution:
    // let s = string.to_lowercase();
    // s.matches('x').count() == s.matches('o').count()

    // best solution:
    string.chars().fold(0, |a, c| match c {
        'x' | 'X' => a + 1,
        'o' | 'O' => a - 1,
        _ => a,
    }) == 0
}

/// https://www.codewars.com/kata/56747fd5cb988479af000028
pub fn get_middle(s: &str) -> &str {
    let start = (s.len() - 1) / 2;
    let end = s.len() / 2;
    &s[start..=end]
}

/// https://www.codewars.com/kata/554b4ac871d6813a03000035
pub fn high_and_low(numbers: &str) -> String {
    let parsed_numbers: Vec<_> = numbers
        .split_whitespace()
        .into_iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    format!(
        "{} {}",
        parsed_numbers.iter().max().unwrap(),
        parsed_numbers.iter().min().unwrap()
    )
}

/// Sum of Minimums!
/// https://www.codewars.com/kata/5d5ee4c35162d9001af7d699
pub fn sum_of_minimums(numbers: [[u8; 4]; 4]) -> u8 {
    numbers.iter().map(|x| x.iter().min().unwrap()).sum()
    //numbers.iter().filter_map(|x| x.iter().min()).sum()
    //numbers.iter().flat_map(|x| x.iter().min()).sum()
}

/// Give me a Diamond
/// https://www.codewars.com/kata/5503013e34137eeeaa001648
pub fn print_diamond(n: i32) -> Option<String> {
    if n < 0 || n % 2 == 0 {
        return None;
    }
    let mut diamond = String::new();
    let origin = n / 2;
    for row in -origin..=origin {
        for column in -origin..=origin {
            let manhattan_distance_from_origin = row.abs() + column.abs();
            if manhattan_distance_from_origin <= origin {
                diamond.push('*');
            } else if column < 0 {
                diamond.push(' ');
            }
        }
        diamond.push('\n');
    }
    Some(diamond)
}

/// Fluid Volume of a Heightmap
/// https://www.codewars.com/kata/5b98dfa088d44a8b000001c1/train/rust
pub fn volume(heightmap: &Vec<Vec<i32>>) -> i32 {
    fluid_volume::calculate_fluid_volume(fluid_volume::HeightMap::new(heightmap))
}

/// Evaluate mathematical expression
/// https://www.codewars.com/kata/52a78825cdfc2cfc87000005/train/rust
pub fn calc(_expr: &str) -> f64 {
    todo!()
    // pouzit C# lexer ze semestralky na PJP
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution("camelCasing"), "camel Casing");
        assert_eq!(solution("camelCasingTest"), "camel Casing Test");
    }

    #[test]
    fn returns_expected() {
        assert_eq!(xo("xo"), true);
        assert_eq!(xo("Xo"), true);
        assert_eq!(xo("xxOo"), true);
        assert_eq!(xo("xxxm"), false);
        assert_eq!(xo("Oo"), false);
        assert_eq!(xo("ooom"), false);
    }

    #[test]
    fn example_tests() {
        assert_eq!(get_middle("test"), "es");
        assert_eq!(get_middle("testing"), "t");
        assert_eq!(get_middle("middle"), "dd");
        assert_eq!(get_middle("A"), "A");
        assert_eq!(get_middle("of"), "of");
    }

    #[test]
    fn test_differentiation() {
        use differentiation::diff;

        assert_eq!(diff("5"), "0");
        assert_eq!(diff("x"), "1");
        assert_eq!(diff("5"), "0");
        assert_eq!(diff("(+ x x)"), "2");
        assert_eq!(diff("(- x x)"), "0");
        assert_eq!(diff("(* x 2)"), "2");
        assert_eq!(diff("(/ x 2)"), "0.5");
        assert_eq!(diff("(^ x 2)"), "(* 2 x)");
        assert_eq!(diff("(cos x)"), "(* -1 (sin x))");
        assert_eq!(diff("(sin x)"), "(cos x)");
        assert_eq!(diff("(tan x)"), "(+ 1 (^ (tan x) 2))");
        assert_eq!(diff("(exp x)"), "(exp x)");
        assert_eq!(diff("(ln x)"), "(/ 1 x)");
        assert_eq!(diff("(+ x (+ x x))"), "3");
        assert_eq!(diff("(- (+ x x) x)"), "1");
        assert_eq!(diff("(* 2 (+ x 2))"), "2");
        assert_eq!(diff("(/ 2 (+ 1 x))"), "(/ -2 (^ (+ 1 x) 2))");
        assert_eq!(diff("(cos (+ x 1))"), "(* -1 (sin (+ x 1)))");

        let result = diff("(cos (* 2 x))");
        assert!(
            result == "(* 2 (* -1 (sin (* 2 x))))"
                || result == "(* -2 (sin (* 2 x)))"
                || result == "(* (* -1 (sin (* 2 x))) 2)"
        );

        assert_eq!(diff("(sin (+ x 1))"), "(cos (+ x 1))");
        assert_eq!(diff("(sin (* 2 x))"), "(* 2 (cos (* 2 x)))");
        assert_eq!(diff("(tan (* 2 x))"), "(* 2 (+ 1 (^ (tan (* 2 x)) 2)))");
        assert_eq!(diff("(exp (* 2 x))"), "(* 2 (exp (* 2 x)))");
        assert_eq!(diff(&diff("(sin x)")), "(* -1 (sin x))");
        assert_eq!(diff(&diff("(exp x)")), "(exp x)");

        let result = diff(&diff("(^ x 3)"));
        assert!(result == "(* 3 (* 2 x))" || result == "(* 6 x)");

        assert_eq!(diff("(^ (sin x) 3)"), "(* (cos x) (* 3 (^ (sin x) 2)))");
    }

    #[test]
    fn test_molecule_parser() {
        use molecule_parser::parse_molecule;

        assert_eq!(parse_molecule("pie").is_ok(), false);
        assert_eq!(parse_molecule("Mg(OH").is_ok(), false);
        assert_eq!(parse_molecule("Mg(OH}2").is_ok(), false);
        assert_eq!(parse_molecule("(C5H5)Fe(CO)2CH3").is_ok(), true);
        assert_eq!(parse_molecule("{[Co(NH3)4(OH)2]3Co}(SO4)3").is_ok(), true);
    }

    #[test]
    fn test_high_and_low() {
        assert_eq!("5 1", high_and_low("1 2 3 4 5"));
        assert_eq!("5 -3", high_and_low("1 2 -3 4 5"));
        assert_eq!("9 -5", high_and_low("1 9 3 4 -5"));
        assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
        assert_eq!("3 1", high_and_low("1 2 3"));
    }

    #[test]
    fn test_sum_of_minimums() {
        assert_eq!(
            sum_of_minimums([[7, 9, 8, 6], [6, 5, 4, 3], [5, 7, 4, 5], [7, 9, 4, 3]]),
            16
        );
        assert_eq!(
            sum_of_minimums([[7, 9, 8, 6], [6, 5, 4, 3], [5, 7, 4, 5], [7, 9, 4, 4]]),
            17
        );
        assert_eq!(
            sum_of_minimums([[7, 9, 8, 84], [6, 5, 4, 65], [5, 7, 4, 23], [7, 9, 4, 25]]),
            19
        );
    }

    #[test]
    fn test_diamond() {
        assert_eq!(print_diamond(3), Some(" *\n***\n *\n".to_string()));
        assert_eq!(
            print_diamond(5),
            Some("  *\n ***\n*****\n ***\n  *\n".to_string())
        );
        assert_eq!(print_diamond(-3), None);
        assert_eq!(print_diamond(2), None);
        assert_eq!(print_diamond(0), None);
        assert_eq!(print_diamond(1), Some("*\n".to_string()));
    }

    // #[test]
    // fn test_all_balanced_parenthesis() {
    //     assert_eq!(all_balanced_parentheses::balanced_parens(3), vec!["((()))", "(()())", "(())()", "()(())", "()()()"]);
    // }

    #[test]
    fn test_eval() {
        use eval::{ eval, to_postfix, Token };

        let input = "((2.33 / (2.9+3.5)*4) - -6)";
        let output: Vec<Token> = to_postfix(input);
        let output_string: String = output
            .iter()
            .map(|token| format!("{token}"))
            .collect::<Vec<_>>()
            .join("|");
        println!("input: {input}; output: {output_string};");
        let value = eval(output);
        assert_eq!(value, 7.45625);
    }
}

#[cfg(test)]
mod fluid_volume_tests {
    //use super::*;

    // this just helps with the test output on failure.
    // fn pretty_test(map: &Vec<Vec<i32>>, expected: i32) {
    //     let result = volume(&map);
    //     let mut printy = String::new();
    //     for row in map {
    //         printy.push_str(format!("{:?}\n", row).as_str());
    //     }
    //     assert_eq!(result, expected, "\nYour result (left) did not match expected result (right) for map:\n{}", printy);
    // }

    // #[test]
    // fn negative_heights_tests() {
    //     let tests = [
    //         (vec![vec![-1]], 0),

    //         (vec![vec![3, 3, 3, 3, 3],
    //               vec![3, 0, 0, 0, 3],
    //               vec![3, 3, 3, 0, 3],
    //               vec![3, 0, -2, 0, 3],
    //               vec![3, 0, 3, 3, 3],
    //               vec![3, 0, 0, 0, 3],
    //               vec![3, 3, 3, 1, -3]], 13),

    //         (vec![vec![8192, 8192, 8192, 8192],
    //               vec![8192,-8192,-8192, 8192],
    //               vec![8192,-8192,-8192, 8192],
    //               vec![8192, 8192, 8192, 8192]], 65536)
    //     ];

    //     for (map, expected) in tests.iter() {
    //         pretty_test(map, *expected);
    //     }
    // }

    // #[test]
    // fn large_map_test() {
    //     // 50x50 map without leaks; 100 around the border, 0 inside
    //     let mut map = vec![vec![100; 50]; 50];
    //     for y in 1..49 {
    //         for x in 1..49 {
    //             map[y][x] = 0;
    //         }
    //     }
    //     // volume = 100 * (48 * 48)
    //     pretty_test(&map, 230_400);
    // }
}

#[cfg(test)]
mod sudoku_tests {

    use super::sudoku::Sudoku;

    #[test]
    fn good_sudoku() {
        let good_sudoku_1 = Sudoku {
            data: vec![
                vec![7, 8, 4, 1, 5, 9, 3, 2, 6],
                vec![5, 3, 9, 6, 7, 2, 8, 4, 1],
                vec![6, 1, 2, 4, 3, 8, 7, 5, 9],
                vec![9, 2, 8, 7, 1, 5, 4, 6, 3],
                vec![3, 5, 7, 8, 4, 6, 1, 9, 2],
                vec![4, 6, 1, 9, 2, 3, 5, 8, 7],
                vec![8, 7, 6, 3, 9, 4, 2, 1, 5],
                vec![2, 4, 3, 5, 6, 1, 9, 7, 8],
                vec![1, 9, 5, 2, 8, 7, 6, 3, 4],
            ],
        };

        let good_sudoku_2 = Sudoku {
            data: vec![
                vec![1, 4, 2, 3],
                vec![3, 2, 4, 1],
                vec![4, 1, 3, 2],
                vec![2, 3, 1, 4],
            ],
        };
        assert!(good_sudoku_1.is_valid());
        assert!(good_sudoku_2.is_valid());
    }

    #[test]
    fn bad_sudoku() {
        let bad_sudoku_1 = Sudoku {
            data: vec![
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            ],
        };

        let bad_sudoku_2 = Sudoku {
            data: vec![
                vec![1, 2, 3, 4, 5],
                vec![1, 2, 3, 4],
                vec![1, 2, 3, 4],
                vec![1],
            ],
        };
        assert!(!bad_sudoku_1.is_valid());
        assert!(!bad_sudoku_2.is_valid());
    }
}

#[cfg(test)]
mod char_count_tests {
    use super::*;
    use std::collections::HashMap;
    use char_count::count;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    #[test]
    fn test_empty_string() {
        let test_input = "";
        let expected: HashMap<char, i32> = HashMap::new();
        
        assert_eq!(count(test_input), expected, "{ERR_MSG} with input: \"{test_input}\"");
    }
    
    #[test]
    fn test_string_with_two_equal_letters() {
        let test_input = "aa";
        let mut expected: HashMap<char, i32> = HashMap::new();
        expected.insert('a', 2);
        
        assert_eq!(count(test_input), expected, "{ERR_MSG} with input: \"{test_input}\"");
    }
        
    #[test]
    fn test_string_with_different_letters() {
        let test_input = "aabb";
        let mut expected: HashMap<char, i32> = HashMap::new();
        expected.insert('a', 2);
        expected.insert('b', 2);
        
        assert_eq!(count(test_input), expected, "{ERR_MSG} with input: \"{test_input}\"");
    }
}