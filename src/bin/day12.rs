use std::collections::HashMap;

use itertools::Itertools;

fn get_cave_num(
    cave_map: &mut HashMap<&'static str, u32>,
    name: &'static str,
    small_counter: &mut u32,
    big_counter: &mut u32,
) -> u32 {
    *cave_map.entry(name).or_insert_with(|| {
        if name.starts_with(|c: char| c.is_ascii_lowercase()) {
            *small_counter += 1;

            *small_counter
        } else {
            *big_counter -= 1;

            *big_counter
        }
    })
}

fn get_paths() -> (HashMap<u32, Vec<u32>>, u32, u32) {
    let mut start_idx = 0;
    let mut end_idx = 0;
    let mut small_ct = 0;
    let mut big_ct = u32::MAX;
    let mut cave_map = HashMap::new();
    let mut path_map = HashMap::new();

    for line in include_str!("../../inputs/day12.txt").lines() {
        if let Some((first, second)) = line.split('-').collect_tuple() {
            let first_num = get_cave_num(&mut cave_map, first, &mut small_ct, &mut big_ct);
            let second_num = get_cave_num(&mut cave_map, second, &mut small_ct, &mut big_ct);

            if first == "start" {
                start_idx = first_num;
            } else if first == "end" {
                end_idx = first_num;
            }

            if second == "start" {
                start_idx = second_num;
            } else if second == "end" {
                end_idx = second_num;
            }

            path_map
                .entry(first_num)
                .or_insert_with(Vec::new)
                .push(second_num);
            path_map
                .entry(second_num)
                .or_insert_with(Vec::new)
                .push(first_num);
        }
    }

    println!("Cave mappings: {cave_map:?}");

    (path_map, start_idx, end_idx)
}

fn get_valid_path_count_part_1(
    paths: &HashMap<u32, Vec<u32>>,
    start: u32,
    end: u32,
    mut visited: u64,
) -> usize {
    if start == end {
        return 1;
    }

    if start < u64::BITS {
        visited |= 1 << start;
    }

    paths
        .get(&start)
        .unwrap()
        .into_iter()
        .filter(|&&neighbor| neighbor >= u64::BITS || (visited >> neighbor) & 1 == 0)
        .map(|&neighbor| get_valid_path_count_part_1(paths, neighbor, end, visited))
        .sum()
}

fn get_neighbor_score(
    paths: &HashMap<u32, Vec<u32>>,
    start: u32,
    end: u32,
    visited: u64,
    second_visited: Option<u32>,
) -> usize {
    paths
        .get(&start)
        .unwrap()
        .into_iter()
        .filter(|&&neighbor| neighbor >= u64::BITS || (visited >> neighbor) & 1 == 0)
        .map(|&neighbor| get_valid_path_count_part_2(paths, neighbor, end, visited, second_visited))
        .sum()
}

fn get_valid_path_count_part_2(
    paths: &HashMap<u32, Vec<u32>>,
    start: u32,
    end: u32,
    mut visited: u64,
    second_visited: Option<u32>,
) -> usize {
    if start == end {
        match second_visited.map(|n| (visited >> n) & 1) {
            Some(1) | None => return 1,
            _ => return 0,
        };
    }

    let mut score = 0;

    if start < u64::BITS {
        if second_visited.is_none() && (visited >> start) & 1 == 0 {
            score += get_neighbor_score(&paths, start, end, visited, Some(start));
        }

        visited |= 1 << start;
    }

    score + get_neighbor_score(paths, start, end, visited, second_visited)
}

fn main() {
    let (paths, start, end) = get_paths();
    let path_count_1 = get_valid_path_count_part_1(&paths, start, end, 0);

    println!("Part 1: {path_count_1}");

    let path_count_2 = get_valid_path_count_part_2(&paths, start, end, 1 << start, None);

    println!("Part 2: {path_count_2}");
}
