use arrayvec::ArrayVec;

const DIM: usize = 10;
const ARRAY_SIZE: usize = DIM * DIM;

fn do_flash(idx: usize, octopi: &mut ArrayVec<u8, ARRAY_SIZE>, flashed: &mut u128) -> usize {
    if (*flashed >> idx) & 1 == 1 {
        return 0;
    }

    *flashed |= 1 << idx;
    octopi[idx] = 0;

    let mut flash_count = 1;
    let mut neighbors = ArrayVec::<_, 8>::new();
    let has_up = idx > DIM - 1;
    let has_down = idx < ARRAY_SIZE - DIM;

    // Up
    if has_up {
        neighbors.push(idx - DIM);
    }

    // Down
    if has_down {
        neighbors.push(idx + DIM);
    }

    // Left
    if idx % DIM != 0 {
        neighbors.push(idx - 1);

        // Up to the left
        if has_up {
            neighbors.push(idx - 1 - DIM);
        }

        // Down to the left
        if has_down {
            neighbors.push(idx - 1 + DIM);
        }
    }

    // Right
    if idx % DIM != DIM - 1 {
        neighbors.push(idx + 1);

        // Up to the right
        if has_up {
            neighbors.push(idx + 1 - DIM);
        }

        // Down to the right
        if has_down {
            neighbors.push(idx + 1 + DIM);
        }
    }

    for neighbor in neighbors {
        if (*flashed >> neighbor) & 1 == 0 {
            octopi[neighbor] += 1;

            if octopi[neighbor] > 9 {
                flash_count += do_flash(neighbor, octopi, flashed);
            }
        }
    }

    flash_count
}

fn step(octopi: &mut ArrayVec<u8, ARRAY_SIZE>) -> usize {
    let mut flash_idxs = ArrayVec::<_, ARRAY_SIZE>::new();
    let mut flashed = 0u128;

    for (idx, octupus) in octopi.iter_mut().enumerate() {
        *octupus += 1;

        if *octupus > 9 {
            flash_idxs.push(idx);
        }
    }

    let mut flash_count = 0;

    for flash_idx in flash_idxs {
        flash_count += do_flash(flash_idx, octopi, &mut flashed);
    }

    flash_count
}

// /// Part 1
// fn main() {
//     let mut octopi = include_str!("../../inputs/day11.txt")
//         .lines()
//         .flat_map(|s| s.as_bytes())
//         .map(|&c| c - b'0')
//         .collect::<ArrayVec<_, ARRAY_SIZE>>();

//     let mut flash_count = 0;

//     for _ in 0..100 {
//         flash_count += step(&mut octopi);
//     }

//     println!("{flash_count}");
// }

/// Part 2
fn main() {
    let mut octopi = include_str!("../../inputs/day11.txt")
        .lines()
        .flat_map(|s| s.as_bytes())
        .map(|&c| c - b'0')
        .collect::<ArrayVec<_, ARRAY_SIZE>>();

    let mut step_count = 0;

    loop {
        step_count += 1;

        if step(&mut octopi) == ARRAY_SIZE {
            break;
        }
    }

    println!("{step_count}");
}
