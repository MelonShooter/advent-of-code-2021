// /// Part 1
// fn process_line(line: &str) -> usize {
//     let divider = line.find(" | ").unwrap();

//     line[(divider + 3)..]
//         .split(' ')
//         .filter(|s| s.len() != 5 && s.len() != 6)
//         .count()
// }

use std::collections::HashMap;

use arrayvec::ArrayVec;

fn string_to_set(s: &str) -> u8 {
    s.as_bytes()
        .iter()
        .fold(0, |acc, &b| acc | (1 << (b - b'a')))
}

/// Part 2
fn process_line(line: &str) -> usize {
    let mut map = HashMap::new();
    let divider = line.find(" | ").unwrap();
    let mut one_set = 0;
    let mut four_set = 0;
    let mut five_segments = ArrayVec::<_, 3>::new();
    let mut six_segments = ArrayVec::<_, 3>::new();

    for (string, set) in line[..divider].split(' ').map(|s| (s, string_to_set(s))) {
        match string.len() {
            5 => five_segments.push(set),
            6 => six_segments.push(set),
            3 => {
                map.insert(set, 7);
            }
            7 => {
                map.insert(set, 8);
            }
            2 => {
                one_set = set;
                map.insert(set, 1);
            }
            4 => {
                four_set = set;
                map.insert(set, 4);
            }
            _ => (),
        };
    }

    for six_segment_set in six_segments {
        if six_segment_set & four_set == four_set {
            map.insert(six_segment_set, 9);
        } else if six_segment_set & one_set != one_set {
            map.insert(six_segment_set, 6);
        } else {
            map.insert(six_segment_set, 0);
        }
    }

    for five_segment_set in five_segments {
        let with_one = (five_segment_set & one_set).count_ones();
        let with_four = (five_segment_set & four_set).count_ones();

        if with_one == 2 {
            map.insert(five_segment_set, 3);
        } else if with_four == 2 {
            map.insert(five_segment_set, 2);
        } else {
            map.insert(five_segment_set, 5);
        }
    }

    line[(divider + 3)..]
        .split(' ')
        .map(string_to_set)
        .map(|set| *map.get(&set).unwrap())
        .fold(0, |acc, num| acc * 10 + num)
}

fn main() {
    let count = include_str!("../../inputs/day8.txt")
        .lines()
        .map(process_line)
        .sum::<usize>();

    println!("{count}");
}
