use std::collections::HashSet;

pub struct Sudoku {
    pub data: Vec<Vec<u32>>,
}

// https://www.codewars.com/kata/540afbe2dc9f615d5e000425/solutions/rust
impl Sudoku {
    pub fn is_valid(&self) -> bool {
        println!("huhu:");
        for row in self.data.iter() {
            for element in row {
                print!("{element}");
            }
            println!();
        }

        let size = self.data.len();
        if !self.data.iter().all(|row| row.len() == size) {
            return false;
        }

        // check rows
        for row in self.data.iter() {
            let mut map = HashSet::new();
            for element in row {
                if map.contains(element) {
                    return false;
                } else {
                    map.insert(element);
                }
            }

            if !map.iter().all(|&x| *x > 0 && *x <= size as u32) {
                return false;
            }
        }

        // check columns
        for index in 0..self.data.len() {
            let mut map = HashSet::new();
            for row in self.data.iter() {
                let element = &row[index];
                if map.contains(element) {
                    return false;
                } else {
                    map.insert(element);
                }
            }
        }

        // check mini-squares
        // let mini_square_size = (self.data.len() as f64).sqrt() as usize;

        // for row_square_index in 0..mini_square_size {
        //     let start_row_index = row_square_index * mini_square_size;
        //     for column_square_index in 0..mini_square_size {
        //         let start_column_index = column_square_index * mini_square_size;
        //         let mut map = HashSet::new();
        //         for row_index in start_row_index..(start_row_index + mini_square_size) {
        //             for column_index in start_column_index..(start_column_index + mini_square_size) {
        //                 let element = &self.data[row_index][column_index];
        //                 if map.contains(element) {
        //                     return false;
        //                 }
        //                 else {
        //                     map.insert(element);
        //                 }
        //             }
        //         }
        //     }
        // }

        true
    }
}
