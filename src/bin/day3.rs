use std::collections::HashSet;

const NUM_BINARY_LEN: usize = 12;

// type Gamma = u32;

// /// Part 1
// fn main() {
//     let file = include_str!("../../inputs/day3.txt");
//     let mut one_count = [0u32; NUM_BINARY_LEN];
//     let mut line_count = 0;

//     for line in file.split('\n').filter(|s| !s.is_empty()) {
//         line.as_bytes()
//             .iter()
//             .enumerate()
//             .filter(|&(_, &b)| b == b'1')
//             .for_each(|(idx, _)| one_count[idx] += 1);

//         line_count += 1;
//     }

//     let common_threshold = line_count / 2;
//     let mut gamma_rate: Gamma = 0;

//     for (offset, count) in one_count.into_iter().rev().enumerate() {
//         if count > common_threshold {
//             gamma_rate |= 1u32 << offset;
//         }
//     }

//     const TO_CLEAR: u32 = Gamma::BITS - NUM_BINARY_LEN as u32;

//     let epsilon_rate = ((!gamma_rate) << TO_CLEAR) >> TO_CLEAR;

//     println!("{}", gamma_rate * epsilon_rate);
// }

/// Part 2
fn main() {
    let file = include_str!("../../inputs/day3.txt");
    let mut oxygen_set = file
        .split('\n')
        .filter_map(|s| u32::from_str_radix(s, 2).ok())
        .collect::<HashSet<_>>();
    let mut co2_set = oxygen_set.clone();
    let mut oxygen = 0;
    let mut co2 = 0;

    for idx in 0..12 {
        if oxygen_set.len() != 1 {
            let one_count = oxygen_set
                .iter()
                .filter(|&num| (num >> (NUM_BINARY_LEN - idx - 1)) & 1 == 1)
                .count();
            let most_common = if one_count as f64 >= oxygen_set.len() as f64 / 2f64 {
                1
            } else {
                0
            };

            oxygen_set.retain(|num| (num >> (NUM_BINARY_LEN - idx - 1)) & 1 == most_common);
        }

        if co2_set.len() != 1 {
            let one_count = co2_set
                .iter()
                .filter(|&num| (num >> (NUM_BINARY_LEN - idx - 1)) & 1 == 1)
                .count();
            let least_common = if (one_count as f64) < co2_set.len() as f64 / 2f64 {
                1
            } else {
                0
            };
            // check if bit is the same at given positon
            co2_set.retain(|num| (num >> (NUM_BINARY_LEN - idx - 1)) & 1 == least_common);
        }

        if oxygen_set.len() == 1 {
            oxygen = *oxygen_set.iter().next().unwrap();
        }

        if co2_set.len() == 1 {
            co2 = *co2_set.iter().next().unwrap();
        }

        if oxygen_set.len() == 1 && co2_set.len() == 1 {
            break;
        }
    }

    // issue is we need to take into account filtering

    println!("{oxygen} {co2} {}", oxygen * co2);
}
