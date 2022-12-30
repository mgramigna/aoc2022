use itertools::Itertools;
use std::{collections::HashSet, ops::RangeInclusive, str::FromStr};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Coord {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Sensor {
    x: isize,
    y: isize,
    closest_beacon: Coord,
}

impl FromStr for Sensor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sensor_info, beacon_info) = s.split_once(":").unwrap();
        let (sx, sy) = sensor_info.split_once(", ").unwrap();
        let (bx, by) = beacon_info.split_once(", ").unwrap();

        Ok(Sensor {
            x: sx.parse().unwrap(),
            y: sy.parse().unwrap(),
            closest_beacon: Coord {
                x: bx.parse().unwrap(),
                y: by.parse().unwrap(),
            },
        })
    }
}

impl Sensor {
    fn manhattan_distance(&self, other: &Coord) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn get_range_at_row(&self, desired_row: isize) -> Option<RangeInclusive<isize>> {
        let radius = self.manhattan_distance(&self.closest_beacon);
        let offset = radius - (self.y - desired_row).abs();

        if offset < 0 {
            return None;
        }
        Some(self.x - offset..=self.x + offset)
    }
}

fn normalize_ranges(ranges: Vec<RangeInclusive<isize>>) -> Vec<RangeInclusive<isize>> {
    ranges
        .into_iter()
        .coalesce(|a, b| {
            if b.start() - 1 <= *a.end() {
                if b.end() > a.end() {
                    Ok(*a.start()..=*b.end())
                } else {
                    Ok(a)
                }
            } else {
                Err((a, b))
            }
        })
        .collect::<Vec<_>>()
}

fn main() {
    let input = include_str!("../inputs/day15.txt");

    let desired_row: isize = 2000000;

    let sensors = input
        .lines()
        .map(|l| {
            l.replace("Sensor at ", "")
                .replace(" closest beacon is at ", "")
                .replace("x=", "")
                .replace("y=", "")
        })
        .map(|l| l.parse::<Sensor>().unwrap())
        .collect::<Vec<_>>();

    let beacon_x_coords = sensors
        .iter()
        .map(|s| s.closest_beacon)
        .filter(|b| b.y == desired_row)
        .map(|b| b.x)
        .collect::<HashSet<_>>();

    let mut x_ranges: Vec<RangeInclusive<isize>> = sensors
        .iter()
        .filter_map(|s| s.get_range_at_row(desired_row))
        .collect();

    x_ranges.sort_by_key(|r| *r.start());

    let count = normalize_ranges(x_ranges)
        .iter()
        .map(|r| {
            let range_size = (r.end() - r.start() + 1) as usize;
            let num_beacons_in_range = beacon_x_coords.iter().filter(|x| r.contains(x)).count();
            range_size - num_beacons_in_range
        })
        .sum::<usize>();

    println!("{}", count);
}
