// https://www.codewars.com/kata/56a5d994ac971f1ac500003e

pub fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    if k > 0 && strarr.len() >= k {
        let mut max_length = usize::MIN;
        let mut max_index: Option<usize> = None;
        for index in 0..strarr.len() - (k - 1) {
            let mut length_sum = 0;
            for i in index..index + k {
                length_sum += strarr[i].len();
            }
            if length_sum > max_length {
                max_index = Some(index);
                max_length = length_sum;
            }
        }
        if let Some(index) = max_index {
            return strarr.into_iter().skip(index).take(k).collect();
        }
    }
    String::new()
}
