use core::panic;
use std::collections::HashMap;

use itertools::{Itertools, MinMaxResult};

type PolymerCounts = HashMap<u8, usize>;
type PolymerPairs = HashMap<[u8; 2], usize>;
type PolymerRules = HashMap<&'static [u8], u8>;

fn parse_data() -> (PolymerPairs, PolymerCounts, PolymerRules) {
    let mut polymer_pair_map = HashMap::new();
    let (polymer, rules) = include_str!("../../inputs/day14.txt")
        .split_once("\n\n")
        .unwrap();
    let polymer_counts_map = polymer.bytes().counts();

    for window in polymer.as_bytes().windows(2) {
        *polymer_pair_map.entry([window[0], window[1]]).or_insert(0) += 1;
    }

    let polymer_rules_map = rules
        .lines()
        .flat_map(|s| s.split_once(" -> "))
        .map(|(key, val)| (key.as_bytes(), val.as_bytes()[0]))
        .collect();

    (polymer_pair_map, polymer_counts_map, polymer_rules_map)
}

fn step(pairs: &mut PolymerPairs, counts: &mut PolymerCounts, rules: &PolymerRules) {
    let mut new_pair_map = HashMap::new();

    for (key, val) in pairs.iter() {
        let ch = *rules.get(key.as_slice()).unwrap();

        *new_pair_map.entry([key[0], ch]).or_insert(0) += val;
        *new_pair_map.entry([ch, key[1]]).or_insert(0) += val;
        *counts.entry(ch).or_insert(0) += val;
    }

    *pairs = new_pair_map
}

fn main() {
    let (mut polymer_pairs, mut polymer_counts, polymer_rules) = parse_data();

    for _ in 0..10 {
        step(&mut polymer_pairs, &mut polymer_counts, &polymer_rules);
    }

    let (lowest, highest) = match polymer_counts.values().minmax() {
        MinMaxResult::MinMax(&min, &max) => (min, max),
        _ => panic!("Invalid result"),
    };

    println!("Part 1: {}", highest - lowest);

    for _ in 0..30 {
        step(&mut polymer_pairs, &mut polymer_counts, &polymer_rules);
    }

    let (lowest, highest) = match polymer_counts.values().minmax() {
        MinMaxResult::MinMax(&min, &max) => (min, max),
        _ => panic!("Invalid result"),
    };

    println!("Part 2: {}", highest - lowest);
}
