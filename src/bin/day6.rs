// /// Part 1
// fn simulate_step(fish_collection: &mut Vec<u8>) {
//     let mut new_count = 0;

//     for fish in &mut fish_collection[..] {
//         if *fish == 0 {
//             *fish = 6;
//             new_count += 1;
//         } else {
//             *fish -= 1;
//         }
//     }

//     fish_collection.resize(fish_collection.len() + new_count, 8);
// }

// fn main() {
//     let file = include_str!("../../inputs/day6.txt");
//     let mut fish_vec = file
//         .lines()
//         .flat_map(|s| s.split(','))
//         .flat_map(str::parse::<u8>)
//         .collect::<Vec<_>>();

//     for _ in 0..256 {
//         simulate_step(&mut fish_vec);
//     }

//     println!("{}", fish_vec.len());
// }

/// Part 2
fn simulate_step(fish_collection: &mut [u64; 9]) {
    let new_fish_count = fish_collection[0];

    fish_collection[..7].rotate_left(1);
    fish_collection[6] += fish_collection[7];
    fish_collection[7] = fish_collection[8];
    fish_collection[8] = new_fish_count;
}

fn main() {
    let file = include_str!("../../inputs/day6.txt");
    let fish_vec = file
        .lines()
        .flat_map(|s| s.split(','))
        .flat_map(str::parse::<u8>)
        .collect::<Vec<_>>();
    let mut fish_count_array = [0u64; 9];

    for fish in fish_vec {
        fish_count_array[fish as usize] += 1;
    }

    for _ in 0..256 {
        simulate_step(&mut fish_count_array);
    }

    println!("{}", fish_count_array.into_iter().sum::<u64>());
}
