#![allow(dead_code)]

pub fn spiralize(size: usize) -> Vec<Vec<i8>> {
    let mut left = 0;
    let mut right = size;
    let mut top = 0;
    let mut bottom = size;
    let mut matrix = vec![vec![0; size]; size];
    while left < right && top < bottom {
        for x in left..right {
            matrix[top][x] = 1;
        }
        if top > 0 {
            left += 2;
        }
        for y in top..bottom {
            matrix[y][right - 1] = 1;
        }
        top += 2;
        if top < bottom {
            for x in (left..right).rev() {
                matrix[bottom - 1][x] = 1;
            }
        }
        right -= 2;
        for y in (top..bottom).rev() {
            matrix[y][left] = 1;
        }
        bottom -= 2;
    }
    matrix
}
