use itertools::Itertools;

type Board = Vec<Vec<u32>>;

const BOARD_DIM: u32 = 5;
const BOARD_SIZE: usize = (BOARD_DIM * BOARD_DIM) as usize;
const HORIZONTAL_BIT_PATTERN: u32 = (!0) << (u32::BITS - BOARD_DIM) >> (u32::BITS - BOARD_DIM);
const VERTICAL_BIT_PATTERN: u32 = {
    let mut bit_pattern = 0;
    let mut i = 0;

    while i < 25 {
        bit_pattern |= 1 << i;
        i += 5;
    }

    bit_pattern
};

fn to_bingo_board(board_str: &str) -> Board {
    board_str
        .split('\n')
        .map(|s| {
            s.split_ascii_whitespace()
                .flat_map(str::parse::<u32>)
                .collect_vec()
        })
        .collect_vec()
}

fn get_winner(
    chosen: u32,
    row_idx: usize,
    col_idx: usize,
    board: &[Vec<u32>],
    bitset: &mut u32,
) -> Option<u32> {
    let bit_pos = row_idx * BOARD_DIM as usize + col_idx;
    *bitset |= 1 << bit_pos;
    let bitset = *bitset;

    for start in 0..5 {
        let h_mask = HORIZONTAL_BIT_PATTERN << (start * 5);
        let v_mask = VERTICAL_BIT_PATTERN << start;

        if bitset & h_mask == h_mask || bitset & v_mask == v_mask {
            let remainder = (0..BOARD_SIZE)
                .map(|i| (i, (bitset >> i) & 1))
                .filter_map(|(idx, bit)| (bit == 0).then(|| board[idx / 5][idx % 5]))
                .sum::<u32>();

            return Some(chosen * remainder);
        }
    }

    None
}

// /// Part 1
// fn main() {
//     let file = include_str!("../../inputs/day4.txt");
//     let after_nums = file.find("\n\n").unwrap();
//     let nums_str = file[..after_nums]
//         .split(',')
//         .map(|s| s.parse::<u32>().unwrap())
//         .collect::<Vec<_>>();
//     let boards = file[(after_nums + 2..)]
//         .split("\n\n")
//         .filter(|s| !s.is_empty())
//         .map(str::trim_end)
//         .map(to_bingo_board)
//         .collect::<Vec<_>>();
//     let mut bitsets: Vec<u32> = Vec::new();
//     bitsets.resize(boards.len(), 0);

//     for num in nums_str {
//         for (b_idx, board) in boards.iter().enumerate() {
//             for (r_idx, row) in board.iter().enumerate() {
//                 for (c_idx, _) in row.iter().enumerate().filter(|(_, &b_num)| num == b_num) {
//                     if let Some(res) = get_winner(num, r_idx, c_idx, board, &mut bitsets[b_idx]) {
//                         println!("{res}");

//                         return;
//                     }
//                 }
//             }
//         }
//     }
// }

/// Part 2
fn main() {
    let file = include_str!("../../inputs/day4.txt");
    let after_nums = file.find("\n\n").unwrap();
    let nums_str = file[..after_nums]
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let boards = file[(after_nums + 2..)]
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(str::trim_end)
        .map(to_bingo_board)
        .collect::<Vec<_>>();
    let mut sets: Vec<u32> = Vec::new();
    sets.resize(boards.len(), 0);

    let mut solved_set = 0u128;
    let board_ct = boards.len() as u32;
    let fully_solved = ((!0) << (u128::BITS - board_ct)) >> (u128::BITS - board_ct);

    for num in nums_str {
        for (b_idx, board) in boards.iter().enumerate() {
            if (solved_set >> b_idx) & 1 == 1 {
                continue;
            }

            for (r_idx, row) in board.iter().enumerate() {
                let winner = row
                    .iter()
                    .enumerate()
                    .filter(|(_, &b_num)| num == b_num)
                    .flat_map(|(c_idx, _)| get_winner(num, r_idx, c_idx, board, &mut sets[b_idx]))
                    .next();

                if let Some(result) = winner {
                    solved_set |= 1 << b_idx;

                    if solved_set & fully_solved == fully_solved {
                        println!("{result}");

                        return;
                    }
                }
            }
        }
    }
}
