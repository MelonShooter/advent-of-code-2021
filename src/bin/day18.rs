use std::{
    num::ParseIntError,
    ops::{Add, AddAssign},
    str::FromStr,
};

use itertools::Itertools;

type Idx = u16;

#[derive(Copy, Clone, Debug)]
enum Expr {
    Pair(Idx, Idx, Idx),
    Num(u16, Idx),
}

impl Expr {
    fn set_parent(&mut self, new_parent: Idx) {
        match self {
            Expr::Pair(_, _, parent) | Expr::Num(_, parent) => *parent = new_parent,
        }
    }
}

#[derive(Clone, Debug)]
struct ExprTree {
    exprs: Vec<Expr>,
}

impl FromStr for ExprTree {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        type ExprRes<T> = Result<T, ParseIntError>;
        fn parse_expr<'a, 'b>(tree: &'a mut Vec<Expr>, s: &'b str) -> ExprRes<(Idx, &'b str)> {
            let (new_expr, s) = if s.starts_with('[') {
                parse_pair(tree, s)?
            } else {
                let non_digit = s.find(|b: char| !b.is_ascii_digit()).unwrap();
                let num = s[..non_digit].parse()?;

                (Expr::Num(num, Idx::MAX), &s[non_digit..])
            };

            let s = &s[s.find(|c: char| c != ']').unwrap_or(0)..];
            let next_idx = tree.len();
            tree.push(new_expr);

            Ok((next_idx as u16, s))
        }

        fn parse_pair<'a, 'b>(tree: &'a mut Vec<Expr>, mut s: &'b str) -> ExprRes<(Expr, &'b str)> {
            s = &s[1..]; // Get rid of '['

            let (first_expr_idx, s) = parse_expr(tree, s)?;
            let (second_expr_idx, s) = parse_expr(tree, &s[1..])?; // Skip comma
            let parent_idx = tree.len() as u16;

            tree[first_expr_idx as usize].set_parent(parent_idx);
            tree[second_expr_idx as usize].set_parent(parent_idx);

            Ok((Expr::Pair(first_expr_idx, second_expr_idx, Idx::MAX), s))
        }

        let mut tree = Vec::new();
        parse_expr(&mut tree, s)?;

        Ok(ExprTree { exprs: tree })
    }
}

impl ExprTree {
    fn mag(&self) -> u64 {
        fn mag_at(tree: &ExprTree, idx: Idx) -> u64 {
            match tree.exprs[idx as usize] {
                Expr::Pair(left, right, _) => 3 * mag_at(tree, left) + 2 * mag_at(tree, right),
                Expr::Num(num, _) => num as u64,
            }
        }

        mag_at(self, self.exprs.len() as Idx - 1)
    }

    fn reduce(&mut self) {
        fn walk<T>(tree: &ExprTree, idx: Idx, depth: usize, pred: T) -> Option<Idx>
        where
            T: Fn(Expr, usize) -> bool + Copy,
        {
            let curr_expr = tree.exprs[idx as usize];

            if let Expr::Pair(left, right, _) = tree.exprs[idx as usize] {
                if let Some(left_res) = walk(tree, left, depth + 1, pred) {
                    return Some(left_res);
                } else if let Some(right_res) = walk(tree, right, depth + 1, pred) {
                    return Some(right_res);
                }
            }

            pred(curr_expr, depth).then(|| idx)
        }

        fn explode(tree: &mut ExprTree) -> bool {
            let to_explode = walk(tree, tree.exprs.len() as Idx - 1, 0, |e, depth| {
                depth >= 4 && matches!(e, Expr::Pair(..))
            });

            if let Some((idx, Expr::Pair(left, right, parent))) =
                to_explode.map(|idx| (idx, tree.exprs[idx as usize]))
            {
                let (left_val, right_val) = (tree.exprs[left as usize], tree.exprs[right as usize]);
                let mut curr_child = idx;
                let mut prev_ancestor = parent;
                let mut next_ancestor = parent;

                while let Some(&Expr::Pair(l, _, p)) = tree.exprs.get(prev_ancestor as usize) {
                    if l == curr_child {
                        curr_child = prev_ancestor;
                        prev_ancestor = p;
                    } else {
                        prev_ancestor = l;

                        while let Expr::Pair(_, r, _) = tree.exprs[prev_ancestor as usize] {
                            prev_ancestor = r;
                        }

                        if let (Expr::Num(num, p), Expr::Num(left_val, _)) =
                            (tree.exprs[prev_ancestor as usize], left_val)
                        {
                            tree.exprs[prev_ancestor as usize] = Expr::Num(num + left_val, p);
                        }

                        break;
                    }
                }

                while let Some(&Expr::Pair(_, r, p)) = tree.exprs.get(next_ancestor as usize) {
                    if r == curr_child {
                        curr_child = next_ancestor;
                        next_ancestor = p;
                    } else {
                        next_ancestor = r;

                        while let Expr::Pair(l, _, _) = tree.exprs[next_ancestor as usize] {
                            next_ancestor = l;
                        }

                        if let (Expr::Num(num, p), Expr::Num(right_val, _)) =
                            (tree.exprs[next_ancestor as usize], right_val)
                        {
                            tree.exprs[next_ancestor as usize] = Expr::Num(num + right_val, p);
                        }

                        break;
                    }
                }

                tree.exprs[idx as usize] = Expr::Num(0, parent);

                true
            } else {
                false
            }
        }

        fn split(tree: &mut ExprTree) -> bool {
            let len = tree.exprs.len();
            let to_split = walk(tree, len as Idx - 1, 0, |e, _| {
                matches!(e, Expr::Num(10.., _))
            });

            if let Some((idx, Expr::Num(n, parent))) =
                to_split.map(|idx| (idx, tree.exprs[idx as usize]))
            {
                tree.exprs[idx as usize] = Expr::Pair(len as Idx - 1, len as Idx, parent);
                tree.exprs.insert(len - 1, Expr::Num(n / 2, idx as Idx));
                tree.exprs.insert(len, Expr::Num(n / 2 + n % 2, idx as Idx));

                let last_idx = tree.exprs.len() - 1;

                // Change root's children's parent's index
                if let Expr::Pair(left, right, _) = tree.exprs[tree.exprs.len() - 1] {
                    tree.exprs[left as usize].set_parent(last_idx as Idx);
                    tree.exprs[right as usize].set_parent(last_idx as Idx);
                }

                true
            } else {
                false
            }
        }

        while explode(self) || split(self) {}
    }
}

impl AddAssign<&ExprTree> for ExprTree {
    fn add_assign(&mut self, rhs: &ExprTree) {
        let offset = self.exprs.len() as Idx;
        let rhs_tree_shifted = rhs.exprs.iter().map(|&rhs_expr| match rhs_expr {
            Expr::Pair(l, r, p) => Expr::Pair(l + offset, r + offset, p.saturating_add(offset)),
            Expr::Num(num, parent) => Expr::Num(num, parent + offset),
        });

        self.exprs.extend(rhs_tree_shifted);

        let lhs_idx = offset - 1;
        let rhs_idx = self.exprs.len() as Idx - 1;

        self.exprs.push(Expr::Pair(lhs_idx, rhs_idx, Idx::MAX));

        let new_root_idx = self.exprs.len() as u16 - 1;

        self.exprs[lhs_idx as usize].set_parent(new_root_idx);
        self.exprs[rhs_idx as usize].set_parent(new_root_idx);

        self.reduce();
    }
}

impl Add<&ExprTree> for &ExprTree {
    type Output = ExprTree;

    fn add(self, rhs: &ExprTree) -> Self::Output {
        let mut left_tree = self.clone();

        left_tree += rhs;
        left_tree
    }
}

fn main() {
    let exprs = include_str!("../../inputs/day18.txt")
        .lines()
        .map(|s| s.parse::<ExprTree>().unwrap())
        .collect_vec();

    let mut added_expr = &exprs[0] + &exprs[1];

    for expr in exprs.iter().skip(2) {
        added_expr += expr;
    }

    println!("Part 1: {}", added_expr.mag());

    let mut highest_mag = 0;

    for (idx, expr) in exprs.iter().enumerate() {
        for (other_idx, other_expr) in exprs.iter().enumerate() {
            if idx != other_idx {
                highest_mag = highest_mag.max((expr + other_expr).mag());
            }
        }
    }

    println!("Part 2: {highest_mag}");
}
