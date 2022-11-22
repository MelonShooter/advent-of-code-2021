use itertools::Itertools;

// /// Part 1
// fn main() {
//     let file = include_str!("../../inputs/day1.txt");
//     let result = file
//         .split('\n')
//         .filter_map(|s| s.parse::<u32>().ok())
//         .tuple_windows()
//         .filter(|(first, second)| second > first)
//         .count();

//     println!("{result}");
// }

/// Part 2
fn main() {
    let file = include_str!("../../inputs/day1.txt");
    let result = file
        .split('\n')
        .filter_map(|s| s.parse::<u32>().ok())
        .tuple_windows()
        .map(|(first, second, third)| first + second + third)
        .tuple_windows()
        .filter(|(first, second)| second > first)
        .count();

    println!("{result}");
}
