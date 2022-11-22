use std::collections::HashSet;

use bitvec::{array::BitArray, BitArr};

const GRID_DIM: usize = 1000;

struct Segment {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Segment {
    fn is_straight(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }

    fn is_left_diagonal(&self) -> bool {
        self.x1 < self.x2 && self.y1 < self.y2 || self.x1 > self.x2 && self.y1 > self.y2
    }

    fn is_right_diagonal(&self) -> bool {
        self.x1 > self.x2 && self.y1 < self.y2 || self.x1 < self.x2 && self.y1 > self.y2
    }
}

fn get_segments() -> impl Iterator<Item = Segment> {
    let file = include_str!("../../inputs/day5.txt");

    file.lines().map(|s| {
        let points = s
            .split(" -> ")
            .flat_map(|s| s.split(','))
            .flat_map(str::parse)
            .collect::<Vec<usize>>();

        Segment {
            x1: points[0],
            y1: points[1],
            x2: points[2],
            y2: points[3],
        }
    })
}

// /// Part 1
// fn main() {
//     let mut intersect_set = HashSet::new();
//     let mut grid: [BitArr!(for GRID_DIM); GRID_DIM] = [BitArray::ZERO; GRID_DIM];

//     for segment in get_segments().filter(Segment::is_straight) {
//         if segment.y1 == segment.y2 {
//             let (lower, upper) = if segment.x1 < segment.x2 {
//                 (segment.x1, segment.x2)
//             } else {
//                 (segment.x2, segment.x1)
//             };
//             let horizontal_slice = &mut grid[segment.y1][lower..=upper];
//             let ones = horizontal_slice
//                 .iter_ones()
//                 .map(|idx| segment.y1 * GRID_DIM + lower + idx);

//             intersect_set.extend(ones);
//             horizontal_slice.fill(true);
//         } else {
//             let (lower, upper) = if segment.y1 < segment.y2 {
//                 (segment.y1, segment.y2)
//             } else {
//                 (segment.y2, segment.y1)
//             };

//             for (idx, row) in (&mut grid[lower..=upper]).into_iter().enumerate() {
//                 if let Some(mut bit) = row.get_mut(segment.x1) {
//                     if *bit {
//                         intersect_set.insert((lower + idx) * GRID_DIM + segment.x1);
//                     } else {
//                         *bit = true;
//                     }
//                 }
//             }
//         }
//     }

//     println!("{}", intersect_set.len());
// }

/// Part 2
fn main() {
    let mut intersect_set = HashSet::new();
    let mut grid: [BitArr!(for GRID_DIM); GRID_DIM] = [BitArray::ZERO; GRID_DIM];

    for segment in get_segments() {
        let (lower_x, upper_x) = if segment.x1 < segment.x2 {
            (segment.x1, segment.x2)
        } else {
            (segment.x2, segment.x1)
        };

        if segment.y1 == segment.y2 {
            let horizontal_slice = &mut grid[segment.y1][lower_x..=upper_x];
            let ones = horizontal_slice
                .iter_ones()
                .map(|idx| segment.y1 * GRID_DIM + lower_x + idx);

            intersect_set.extend(ones);
            horizontal_slice.fill(true);
        } else {
            let (lower_y, upper_y) = if segment.y1 < segment.y2 {
                (segment.y1, segment.y2)
            } else {
                (segment.y2, segment.y1)
            };

            let mut curr_x = if !segment.is_right_diagonal() {
                lower_x
            } else {
                upper_x
            };

            for (idx, row) in (&mut grid[lower_y..=upper_y]).into_iter().enumerate() {
                if let Some(mut bit) = row.get_mut(curr_x) {
                    if *bit {
                        intersect_set.insert((lower_y + idx) * GRID_DIM + curr_x);
                    } else {
                        *bit = true;
                    }
                }

                if segment.is_left_diagonal() {
                    curr_x += 1;
                } else if segment.is_right_diagonal() {
                    curr_x -= 1;
                }
            }
        }
    }

    println!("{}", intersect_set.len());
}
