use itertools::Itertools;

fn char_to_corrupt_points(ch: u8) -> u32 {
    match ch {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        _ => panic!("Invalid character"),
    }
}

fn get_first_corrupt_char(line: &str) -> Result<u8, Vec<u8>> {
    let mut brace_stack = Vec::new();

    for ch in line.as_bytes().iter().copied() {
        match ch {
            b'(' | b'[' | b'{' | b'<' => brace_stack.push(ch),
            b']' | b'}' | b'>' => {
                if brace_stack.pop().map(|c| c + 2) != Some(ch) {
                    return Ok(ch);
                }
            }
            b')' => {
                if brace_stack.pop().map(|c| c + 1) != Some(ch) {
                    return Ok(ch);
                }
            }
            _ => continue,
        }
    }

    Err(brace_stack)
}

// /// Part 1
// fn main() {
//     let score = include_str!("../../inputs/day10.txt")
//         .lines()
//         .flat_map(|line| get_first_corrupt_char(line))
//         .map(char_to_corrupt_points)
//         .sum::<u32>();

//     println!("{score}");
// }

fn char_completion_points(ch: u8) -> u64 {
    match ch {
        b'(' => 1,
        b'[' => 2,
        b'{' => 3,
        b'<' => 4,
        _ => panic!("Invalid character"),
    }
}
fn remainder_to_completion_score(remainder: Vec<u8>) -> u64 {
    remainder
        .into_iter()
        .rev()
        .fold(0, |acc, curr| acc * 5 + char_completion_points(curr))
}

/// Part 2
fn main() {
    let scores = include_str!("../../inputs/day10.txt")
        .lines()
        .filter_map(|line| get_first_corrupt_char(line).err())
        .map(remainder_to_completion_score)
        .sorted_unstable()
        .collect_vec();

    println!("{}", scores[scores.len() / 2]);
}
