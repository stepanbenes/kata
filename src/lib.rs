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
    string.chars().fold(0, |a, c|{
        match c {
          'x' | 'X' => a + 1,
          'o' | 'O' => a - 1,
          _ => a
        }
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
  assert_eq!(get_middle("test"),"es");
  assert_eq!(get_middle("testing"),"t");
  assert_eq!(get_middle("middle"),"dd");
  assert_eq!(get_middle("A"),"A");
  assert_eq!(get_middle("of"),"of");
}
}
