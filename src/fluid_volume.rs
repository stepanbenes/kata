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

struct Square { height: i32, volume: i32, visited: bool }
struct Index(usize, usize);

pub struct HeightMap { heightmap: Vec<Vec<Square>> }

impl HeightMap {
    pub fn new(heightmap: &Vec<Vec<i32>>) -> HeightMap {
        HeightMap { heightmap: heightmap.iter().map(|row| row.iter().map(|item| Square { height: *item, volume: 0, visited: false }).collect()).collect() }
    }

    pub fn at(&self, index: Index) -> Option<&Square> {
        let Index(x, y) = index;
        self.heightmap.get(x)?.get(y)
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
}

impl Display for HeightMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

pub fn calculate_fluid_volume(heightmap: HeightMap) -> i32 {
    //print!("{}", heightmap);
    0
}

// fn find_lowest_neighbor(index: Index, heightmap: &HeightMap) -> Index {
//     let Index(center_x, center_y) = index;
//     let center_square = heightmap.at(index).unwrap();
//     let left_neighbor = heightmap.at(Index(center_x - 1, center_y));



//     let right_neighbor = heightmap.at(Index(center_x + 1, center_y));
//     let top_neighbor = heightmap.at(Index(center_x, center_y - 1));
//     let bottom_neighbor = heightmap.at(Index(center_x, center_y + 1));
// }