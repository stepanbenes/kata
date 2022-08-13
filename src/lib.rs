mod differentiation;
mod molecule_parser;
//mod spiralize;
//mod last_digit_of_large_numbers;
//mod dijkstra;

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
}
