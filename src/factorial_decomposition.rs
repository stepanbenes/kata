// https://www.codewars.com/kata/5a045fee46d843effa000070/train/rust

// n = 12; decomp(12) -> "2^10 * 3^5 * 5^2 * 7 * 11"; since 12! is divisible by 2 ten times, by 3 five times, by 5 two times and by 7 and 11 only once.

// n = 22; decomp(22) -> "2^19 * 3^9 * 5^4 * 7^3 * 11^2 * 13 * 17 * 19"

// n = 25; decomp(25) -> 2^22 * 3^10 * 5^6 * 7^3 * 11^2 * 13 * 17 * 19 * 23

use std::collections::BTreeMap;

pub fn decomp(n: i32) -> String {
    let mut factors_map: BTreeMap<i32, i32> = BTreeMap::new();

    for x in 2..=n {
        for factor in factorize(x) {
            *factors_map.entry(factor).or_default() += 1;
        }
    }

    let mut parts: Vec<String> = Vec::new();

    for (factor, count) in factors_map {
        parts.push(if count == 1 {
            factor.to_string()
        } else {
            format!("{factor}^{count}")
        });
    }

    parts.join(" * ")
}

fn factorize(mut n: i32) -> Vec<i32> {
    let mut factors = Vec::new();
    let mut divisor = 2;

    while n > 1 {
        while n % divisor == 0 {
            factors.push(divisor);
            n /= divisor;
        }

        divisor += 1;
    }

    factors
}
