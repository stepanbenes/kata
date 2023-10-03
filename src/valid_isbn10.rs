// https://www.codewars.com/kata/51fc12de24a9d8cb0e000001/train/rust

pub fn valid_isbn10(isbn: &str) -> bool {
    if isbn.len() != 10 {
        return false;
    }
    let mut sum = 0;
    for (i, c) in isbn.chars().enumerate() {
        let value = if let Some(digit) = c.to_digit(10) {
            digit
        } else if c == 'X' && i == 9 {
            10
        } else {
            return false;
        };
        sum += value * (i + 1) as u32;
    }
    println!("{isbn}: {sum}");
    sum % 11 == 0
}
