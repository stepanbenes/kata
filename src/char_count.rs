// https://www.codewars.com/kata/52efefcbcdf57161d4000091

use std::collections::HashMap;

pub fn count(input: &str) -> HashMap<char, i32> {
    // let mut result = HashMap::<char, i32>::new();
    // for c in input.chars() {
    //     let value_ref = result.entry(c).or_insert(0);
    //     *value_ref += 1;
    // }
    //result
    input
        .chars()
        .fold(HashMap::<char, i32>::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        })
}
