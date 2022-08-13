pub fn last_digit(str1: &str, str2: &str) -> u32 {
    let last_digit = str1.chars().last().unwrap().to_digit(10).unwrap();
    let x = digit_modulo_count(last_digit);
    let y = modulo(str2, x);
    let result = digit_modulo_result(last_digit, y);
    result
}

pub fn digit_modulo_count(digit: u32) -> u32 {
    let mut count = 0;
    let mut n = digit;
    loop {
        n *= digit;
        count += 1;
        if n % 10 == digit {
            return count;
        }
    }
}

pub fn digit_modulo_result(digit: u32, count: u32) -> u32 {
    let mut n = digit;
    for _ in 0..count {
        n *= digit;
    }
    n % 10
}

pub fn modulo(num: &str, a: u32) -> u32 {
    let mut result = 0;
    for c in num.chars() {
        let digit = c.to_digit(10).unwrap();
        result = (result * 10 + digit) % a;
    }
    result
}
