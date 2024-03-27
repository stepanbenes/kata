// https://www.codewars.com/kata/54a91a4883a7de5d7800009c

pub fn increment_string(s: &str) -> String {
    let mut result = String::new();

    if s.is_empty() {
        result.push('1');
        return result;
    }

    let mut last_was_digit = true;
    let mut carry_one = true;
    for c in s.chars().rev() {
        match c.to_digit(10) {
            Some(d) if last_was_digit => {
                let new_d = d + if carry_one { 1 } else { 0 };
                result.insert(0, std::char::from_digit(new_d % 10, 10).unwrap());

                last_was_digit = true;
                carry_one = new_d > 9;
            }
            _ if carry_one => {
                result.insert(0, '1');
                result.insert(0, c);

                last_was_digit = false;
                carry_one = false;
            }
            _ => {
                result.insert(0, c);

                last_was_digit = false;
                carry_one = false;
            }
        }
    }

    result
}
