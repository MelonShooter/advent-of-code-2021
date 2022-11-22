use arrayvec::ArrayVec;
use bitvec::{array::BitArray, slice::BitSlice, BitArr};
use itertools::Itertools;

const DIM: usize = 100;

// /// Part 1
// fn main() {
//     let nums = include_str!("../../inputs/day9.txt")
//         .lines()
//         .map(|line| {
//             line.as_bytes()
//                 .iter()
//                 .map(|&b| b - b'0')
//                 .collect::<ArrayVec<_, DIM>>()
//         })
//         .collect::<ArrayVec<_, DIM>>();

//     let mut risk_level = 0u32;

//     fn check_up_down(num: u8, row: usize, col: usize, nums: &[ArrayVec<u8, DIM>]) -> bool {
//         if row == 0 {
//             num < nums[1][col]
//         } else if row == DIM - 1 {
//             num < nums[DIM - 2][col]
//         } else {
//             num < nums[row + 1][col] && num < nums[row - 1][col]
//         }
//     }

//     for (row_idx, row) in nums.iter().enumerate() {
//         let (&first, &last) = (row.first().unwrap(), row.last().unwrap());

//         if first < row[1] && check_up_down(first, row_idx, 0, nums.as_slice()) {
//             risk_level += 1 + first as u32;
//         }

//         if last < row[DIM - 2] && check_up_down(last, row_idx, DIM - 1, nums.as_slice()) {
//             risk_level += 1 + last as u32;
//         }

//         for (w_idx, (left, num, right)) in row.iter().copied().tuple_windows().enumerate() {
//             let col_idx = w_idx + 1;
//             let above = if row_idx > 0 {
//                 nums[row_idx - 1][col_idx]
//             } else {
//                 u8::MAX
//             };
//             let below = if row_idx + 1 != DIM {
//                 nums[row_idx + 1][col_idx]
//             } else {
//                 u8::MAX
//             };

//             if num < left && num < right && num < above && num < below {
//                 risk_level += 1 + num as u32;
//             }
//         }
//     }

//     println!("{risk_level}");
// }

fn get_low_points(nums: &[ArrayVec<u8, DIM>]) -> Vec<(usize, usize)> {
    fn check_up_down(num: u8, row: usize, col: usize, nums: &[ArrayVec<u8, DIM>]) -> bool {
        if row == 0 {
            num < nums[1][col]
        } else if row == DIM - 1 {
            num < nums[DIM - 2][col]
        } else {
            num < nums[row + 1][col] && num < nums[row - 1][col]
        }
    }

    let mut points = Vec::new();

    for (row_idx, row) in nums.iter().enumerate() {
        let (&first, &last) = (row.first().unwrap(), row.last().unwrap());

        if first < row[1] && check_up_down(first, row_idx, 0, nums) {
            points.push((row_idx, 0));
        }

        if last < row[DIM - 2] && check_up_down(last, row_idx, DIM - 1, nums) {
            points.push((row_idx, DIM - 1));
        }

        for (w_idx, (left, num, right)) in row.iter().copied().tuple_windows().enumerate() {
            let col_idx = w_idx + 1;
            let above = if row_idx > 0 {
                nums[row_idx - 1][col_idx]
            } else {
                u8::MAX
            };
            let below = if row_idx + 1 != DIM {
                nums[row_idx + 1][col_idx]
            } else {
                u8::MAX
            };

            if num < left && num < right && num < above && num < below {
                points.push((row_idx, col_idx));
            }
        }
    }

    points
}

fn count_basin_recursive(
    visited: &mut BitSlice,
    nums: &[ArrayVec<u8, DIM>],
    row: usize,
    col: usize,
) -> usize {
    if *visited.get(row * DIM + col).unwrap() || nums[row][col] == 9 {
        return 0;
    }

    visited.set(row * DIM + col, true);

    let mut sum = 0;

    // Left
    if col > 0 {
        sum += count_basin_recursive(visited, nums, row, col - 1);
    }

    // Right
    if col < DIM - 1 {
        sum += count_basin_recursive(visited, nums, row, col + 1);
    }

    // Up
    if row > 0 {
        sum += count_basin_recursive(visited, nums, row - 1, col);
    }

    // Down
    if row < DIM - 1 {
        sum += count_basin_recursive(visited, nums, row + 1, col);
    }

    sum + 1
}

fn count_basin(nums: &[ArrayVec<u8, DIM>], row: usize, col: usize) -> usize {
    let mut visited: BitArr!(for DIM * DIM) = BitArray::ZERO;

    count_basin_recursive(visited.as_mut_bitslice(), nums, row, col)
}

fn main() {
    let nums = include_str!("../../inputs/day9.txt")
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|&b| b - b'0')
                .collect::<ArrayVec<_, DIM>>()
        })
        .collect::<ArrayVec<_, DIM>>();
    let answer = get_low_points(nums.as_slice())
        .into_iter()
        .map(|(row, col)| count_basin(nums.as_slice(), row, col))
        .sorted_unstable()
        .rev()
        .take(3)
        .fold(1, |acc, n| acc * n);

    println!("{answer}");
}
