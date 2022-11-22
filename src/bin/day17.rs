use std::ops::RangeInclusive;

use itertools::Itertools;

type TargetRng<T> = RangeInclusive<T>;

fn hits(x_rng: TargetRng<u32>, y_rng: TargetRng<i32>, mut x_vel: u32, mut y_vel: i32) -> bool {
    let (mut curr_x, mut curr_y) = if y_vel > 0 {
        let steps = (2 * y_vel + 1) as u32;
        let upper_bound = x_vel;
        x_vel = x_vel.saturating_sub(steps);
        y_vel = -y_vel - 1;
        let lower_bound = x_vel + 1;
        let x_offset = (lower_bound + upper_bound) * (upper_bound - lower_bound + 1) / 2;

        (x_offset, 0)
    } else {
        (0, 0)
    };

    while curr_x <= *x_rng.end() && curr_y >= *y_rng.start() {
        curr_x += x_vel;
        curr_y += y_vel;
        x_vel = x_vel.saturating_sub(1);
        y_vel -= 1;

        if x_rng.contains(&curr_x) && y_rng.contains(&curr_y) {
            return true;
        }
    }

    false
}

fn main() {
    // we can simplify calculations a bit
    // by calculating the x coordinate when the probe
    // gets on the same level as the launch site (y = 0)
    // the next y velocity will be the -original y velocity - 1
    // the next x velocity will be original x velocity - num of steps so far clamped to 0
    // number of steps to get to that point will be 2 * y velocity + 1
    // this means we need to maximize the number of steps taken to get to that point
    // and minimize the x coor while still hitting the target
    // thing is is that y velocity affects number of steps which affects how far it goes
    // unless we only look at options where the x velocity is 0 by that time
    // which means we just need to find the highest y velocity that still hits
    // the highest y coor for this will be abs(lowest y coor) - 1
    // to calculate the highest y pos from a coor, it'll be 1 + 2 ... + v.y
    // which is (1 + v.y) * v.y / 2.
    // 2 + 3 + 4 + 5

    let file = include_str!("../../inputs/day17.txt").trim_end();
    let stripped = &file[15..];
    let (from_x, to_x, from_y, to_y) = stripped
        .split(", y=")
        .flat_map(|s| s.split(".."))
        .flat_map(str::parse::<i32>)
        .collect_tuple()
        .unwrap();
    let highest_y_velocity = from_y.abs() - 1;
    let highest_y = (1 + highest_y_velocity) * highest_y_velocity / 2;

    println!("Part 1: {highest_y}");

    let mut counter = 0;

    for y in from_y..=highest_y_velocity {
        for x in 1..=to_x {
            if hits(from_x as u32..=to_x as u32, from_y..=to_y, x as u32, y) {
                counter += 1;
            }
        }
    }

    println!("Part 2: {counter}");
}
