use bitvec::{array::BitArray, BitArr};
use itertools::Itertools;

#[derive(Copy, Clone)]
enum FoldType {
    Horizontal,
    Vertical,
}

#[derive(Copy, Clone)]
struct Fold {
    fold_type: FoldType,
    coor: usize,
}

struct Grid {
    grid: Vec<BitArr!(for 1400)>,
    x_size: usize,
    y_size: usize,
}

impl Grid {
    fn fold(&mut self, fold: Fold) {
        if let FoldType::Horizontal = fold.fold_type {
            for row in self.grid.iter_mut() {
                let (left, right) = row.split_at_mut(fold.coor);
                let right = &right[0..fold.coor + 1];

                for to_fold in right.iter_ones() {
                    let bit_idx = fold.coor - to_fold;

                    if let Some(bit) = left.get_mut(bit_idx) {
                        bit.commit(true);
                    }
                }
            }

            self.x_size = fold.coor;
        } else {
            let (top, bottom) = self.grid.split_at_mut(fold.coor);
            let bottom_slice = &bottom[1..fold.coor + 1];

            top.iter_mut()
                .zip(bottom_slice.into_iter().rev())
                .for_each(|(dest, src)| *dest |= src);

            self.y_size = fold.coor;
        }
    }

    fn dot_count(&self) -> usize {
        self.grid
            .iter()
            .take(self.y_size)
            .map(|bits| bits.iter_ones().take_while(|&b| b < self.x_size).count())
            .sum()
    }
}

fn parse_data() -> (Grid, Vec<Fold>) {
    let file = include_str!("../../inputs/day13.txt");
    let (points_str, folds_str) = file.split("\n\n").collect_tuple().unwrap();
    let mut grid = Vec::new();
    grid.resize(1000, BitArray::ZERO);

    let mut x_size = 0;
    let mut y_size = 0;

    for l in points_str.lines() {
        if let Some((x, y)) = l.split(',').flat_map(str::parse::<usize>).collect_tuple() {
            x_size = x_size.max(x);
            y_size = y_size.max(y);
            grid[y].set(x, true);
        }
    }

    x_size += 1;
    y_size += 1;

    let mut fold_vec = Vec::new();

    for line in folds_str.lines() {
        let fold_type = match line.as_bytes()[11] {
            b'x' => FoldType::Horizontal,
            b'y' => FoldType::Vertical,
            _ => continue,
        };
        let coordinate = match line[13..].parse() {
            Ok(coor) => coor,
            Err(_) => continue,
        };

        fold_vec.push(Fold {
            fold_type,
            coor: coordinate,
        });
    }

    let grid = Grid {
        grid,
        x_size,
        y_size,
    };

    (grid, fold_vec)
}

fn main() {
    let (mut points, folds) = parse_data();
    points.fold(folds[0]);

    println!("Part 1: {}", points.dot_count());

    for fold in folds.into_iter().skip(1) {
        points.fold(fold);
    }

    println!("Part 2");

    for row in points.grid.into_iter().take(points.y_size) {
        for bit in row.into_iter().take(points.x_size) {
            if bit {
                print!("# ");
            } else {
                print!(". ");
            }
        }

        println!();
    }
}
