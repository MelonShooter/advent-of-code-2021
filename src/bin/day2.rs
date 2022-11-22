// /// Part 1
// fn main() {
//     let file = include_str!("../../inputs/day2.txt");
//     let mut horizontal_pos = 0;
//     let mut depth = 0;

//     for line in file.split('\n').filter(|s| !s.is_empty()) {
//         if line.starts_with('f') {
//             horizontal_pos += line[8..].parse::<u32>().unwrap();
//         } else if line.starts_with('d') {
//             depth += line[5..].parse::<i32>().unwrap();
//         } else {
//             depth -= line[3..].parse::<i32>().unwrap();
//         }
//     }

//     println!("{}", horizontal_pos as i32 * depth);
// }

/// Part 2
fn main() {
    let file = include_str!("../../inputs/day2.txt");
    let mut horizontal_pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in file.split('\n').filter(|s| !s.is_empty()) {
        if line.starts_with('f') {
            let forward_amount = line[8..].parse::<u32>().unwrap();

            horizontal_pos += forward_amount;
            depth += aim * forward_amount as i32;
        } else if line.starts_with('d') {
            aim += line[5..].parse::<i32>().unwrap();
        } else {
            aim -= line[3..].parse::<i32>().unwrap();
        }
    }

    println!("{}", horizontal_pos as i32 * depth);
}
