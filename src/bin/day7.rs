// /// Part 1
// fn main() {
//     let file = include_str!("../../inputs/day7.txt");
//     let mut crab_vec = file
//         .lines()
//         .flat_map(|s| s.split(','))
//         .flat_map(str::parse::<u32>)
//         .collect::<Vec<_>>();

//     crab_vec.sort_unstable();

//     let target_pos = crab_vec[crab_vec.len() / 2];
//     let cost: u32 = crab_vec
//         .into_iter()
//         .map(|pos| pos.abs_diff(target_pos))
//         .sum();

//     println!("{cost}");
// }

/// Part 2
fn main() {
    let file = include_str!("../../inputs/day7.txt");
    let vec = file
        .lines()
        .flat_map(|s| s.split(','))
        .flat_map(str::parse::<u32>)
        .collect::<Vec<_>>();

    let target_pos = (vec.iter().copied().sum::<u32>() as f64 / vec.len() as f64).floor() as u32;
    let cost: u32 = vec
        .into_iter()
        .map(|pos| (pos.abs_diff(target_pos) * (pos.abs_diff(target_pos) + 1) / 2))
        .sum();

    println!("{cost}");
}
