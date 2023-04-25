// Fluid Volume of a Heightmap

// heightmap:
//   8 8 8 8 6 6 6 6
//   8 0 0 8 6 0 0 6
//   8 0 0 8 6 0 0 6
//   8 8 8 8 6 6 6 0

// filled:
//   8 8 8 8 6 6 6 6
//   8 8 8 8 6 6 6 6
//   8 8 8 8 6 6 6 6
//   8 8 8 8 6 6 6 0

// result: 4*8 + 4*6 = 56
// For this heightmap, you would return 56: were you to pour water over it until it couldn't contain any more, it would look like the second heightmap, taking on 56 units of water in the process.

// Water pours off the edges of the heightmap, even when they are negative. It doesn't flow through diagonal cracks (note the lower-right corner of the example).
// Heightmaps in the test cases will come in many different sizes, and some will be quite large, but they will always be rectangular. Heights may be negative.

// Performances requirements:
// Think about the efficiency of your solution:

// 75 large random tests, where 80 <= width|height <= 100 and `-50 <= depth <= 150
use std::fmt::{self, Display};

#[derive(Debug)]
struct Square { height: i32, volume: i32, visited: bool }

#[derive(Debug, Copy, Clone)]
struct Index(isize, isize);

pub struct HeightMap { heightmap: Vec<Vec<Square>> }

impl HeightMap {
    pub fn new(heightmap: &Vec<Vec<i32>>) -> HeightMap {
        HeightMap { heightmap: heightmap.iter().map(|row| row.iter().map(|item| Square { height: *item, volume: 0, visited: false }).collect()).collect() }
    }

    fn at(&self, index: Index) -> Option<&Square> {
        let Index(x, y) = index;
        if x < 0 || y < 0 {
            return None;
        }
        self.heightmap.get(x as usize)?.get(y as usize)
    }

    fn at_mut(&mut self, index: Index) -> Option<&mut Square> {
        let Index(x, y) = index;
        if x < 0 || y < 0 {
            return None;
        }
        self.heightmap.get_mut(x as usize)?.get_mut(y as usize)
    }

    fn size(&self) -> (usize, usize) {
        (self.heightmap.len(), self.heightmap[0].len())
    }

    pub fn print(&self) -> String {
        let mut result = String::new();
        for row in self.heightmap.iter() {
            for square in row {
                result.push_str(&format!("{} ", square.height));
            }
            result.push('\n');
        }
        result
    }

    pub fn print_volume(&self) -> String {
        let mut result = String::new();
        for row in self.heightmap.iter() {
            for square in row {
                result.push_str(&format!("{} ", square.volume));
            }
            result.push('\n');
        }
        result
    }

    pub fn sum_volume(&self) -> i32 {
        let mut sum = 0;
        for row in self.heightmap.iter() {
            for square in row {
                sum += square.volume;
            }
        }
        sum
    }

    pub fn reset(&mut self) {
        for row in self.heightmap.iter_mut() {
            for square in row.iter_mut() {
                square.visited = false;
            }
        }
    }
}

impl Display for HeightMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

pub fn calculate_fluid_volume(mut heightmap: HeightMap) -> i32 {
    //println!("{heightmap}");
    let (rows, columns) = heightmap.size();
    for row in 0..rows {
        for column in 0..columns {
            let index = Index(row as isize, column as isize);
            if let Some(lowest_edge_square_index) = find_lowest_edge_square(index, &mut heightmap) {
                let lowest_edge_square = heightmap.at(lowest_edge_square_index).unwrap();
                flood_fill(index, lowest_edge_square.height, &mut heightmap);
            }
            heightmap.reset(); // TODO: avoid this
        }
    }
    //println!("{}", heightmap.print_volume());
    heightmap.sum_volume()
}

fn find_lowest_edge_square(index: Index, heightmap: &mut HeightMap) -> Option<Index> {
    let Index(center_x, center_y) = index;
    let center_square = heightmap.at_mut(index)?;
    let center_square_height = center_square.height;
    center_square.visited = true;

    let neighbor_indexes = [Index(center_x - 1, center_y), Index(center_x + 1, center_y), Index(center_x, center_y - 1), Index(center_x, center_y + 1)];
    let mut neighbors = Vec::<_>::new();
    for index in neighbor_indexes {
        let neighbor = heightmap.at_mut(index)?;
        if !neighbor.visited {
            neighbor.visited = true;
            neighbors.push((heightmap.at(index)?.height, index));
        }
    }

    let &(min_neighbor_height, min_neighbor_index) = neighbors.iter().min_by_key(|&(square_height, _)| square_height)?;
    if let Some(&(min_neighbor_edge_height, min_neighbor_edge_index)) = neighbors.iter().filter(|&(square_height, _)| *square_height > center_square_height).min_by_key(|&(square_height, _)| square_height) { // found edge
        if min_neighbor_edge_height == min_neighbor_height { // edge is lowest
            Some(min_neighbor_edge_index)
        }
        else {
            let min_global_edge_square_index = find_lowest_edge_square(min_neighbor_index, heightmap)?;
            let min_global_edge_square_height = heightmap.at(min_global_edge_square_index)?.height;
            if min_global_edge_square_height < min_neighbor_edge_height {
                Some(min_global_edge_square_index)
            }
            else {
                Some(min_neighbor_edge_index)
            }
        }
    }
    else {
        find_lowest_edge_square(min_neighbor_index, heightmap)
    }
}

fn flood_fill(index: Index, fill_height: i32, heightmap: &mut HeightMap) {
    let Index(center_x, center_y) = index;
    let center_square = heightmap.at_mut(index).unwrap();
    if try_fill_square(center_square, fill_height) {
        let neighbor_indexes = [Index(center_x - 1, center_y), Index(center_x + 1, center_y), Index(center_x, center_y - 1), Index(center_x, center_y + 1)];
        for neighbor_index in neighbor_indexes {
            if let Some(neighbor) = heightmap.at_mut(neighbor_index) {
                if try_fill_square(neighbor, fill_height) {
                    flood_fill(neighbor_index, fill_height, heightmap);
                }
            }
        }
    }

    fn try_fill_square(square: &mut Square, fill_height: i32) -> bool {
        if square.height + square.volume < fill_height {
            square.volume = fill_height - square.height;
            true
        }
        else {
            false
        }
    }
}