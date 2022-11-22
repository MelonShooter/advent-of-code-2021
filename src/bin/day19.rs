use std::collections::HashSet;

use itertools::Itertools;
use nalgebra::{Point3, Translation3};

type Beacon = Point3<i16>;
type Scanner = Vec<Beacon>;

struct ScannerMap {
    uniformly_rotated_scanners: Vec<Scanner>,
    beacon_locations: HashSet<Beacon>,
}

impl ScannerMap {
    fn calculate_map(
        visited: Vec<bool>,
        locations: &mut HashSet<Beacon>,
        primary_idx: usize,
        location_offset: Translation3<i16>,
        others: &mut [Scanner],
        target_idx: usize,
    ) {
        // skip rotation part if primary_idx is target_idx, but add to locations set and find connections
        // we also need to check if the beacon has been visited
        todo!()
    }

    fn new(primary: Scanner, mut others: Vec<Scanner>) -> Self {
        let mut locations = HashSet::new();

        others.push(primary);

        let first_idx = others.len() - 1;

        // We need to rotate others relative to primary by finding pairs of overlapping beacons then calculating a beacon location
        // from that, making sure to be translating from the primary beacon.
        Self::calculate_map(
            Vec::new(),
            &mut locations,
            first_idx,
            Translation3::identity(),
            &mut others,
            first_idx,
        );

        ScannerMap {
            uniformly_rotated_scanners: others,
            beacon_locations: locations,
        }
    }

    fn unique_beacon_ct(&self) -> usize {
        self.beacon_locations.len()
    }
}

fn parse_data() -> Vec<Scanner> {
    fn to_point(s: &str) -> Point3<i16> {
        let (x, y, z) = s
            .split(',')
            .map(|n| n.parse::<i16>().unwrap())
            .collect_tuple()
            .unwrap();

        Point3::new(x, y, z)
    }

    let mut data = Vec::new();

    for scanner_str in include_str!("../../inputs/day19.txt").split("\n\n") {
        let scanner_vec = scanner_str.lines().skip(1).map(to_point).collect();

        data.push(scanner_vec);
    }

    data
}

fn main() {
    let scanners = parse_data();
    let map = ScannerMap::new(scanners[0].clone(), scanners[1..].to_vec());
    let unique_beacon_ct = map.unique_beacon_ct();

    println!("Part 1: {unique_beacon_ct}");
}
