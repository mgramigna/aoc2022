use itertools::Itertools;
use std::{ops::RangeInclusive, str::FromStr};

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

    fn beacon_range_at_row(&self, desired_row: isize) -> Option<RangeInclusive<isize>> {
        let radius = self.manhattan_distance(&self.closest_beacon);
        let offset = radius - (self.y - desired_row).abs();

        if offset < 0 {
            return None;
        }
        Some(self.x - offset..=self.x + offset)
    }
}

fn normalize_ranges(sensors: &Vec<Sensor>, desired_row: isize) -> Vec<RangeInclusive<isize>> {
    let mut x_ranges: Vec<RangeInclusive<isize>> = sensors
        .iter()
        .filter_map(|s| s.beacon_range_at_row(desired_row))
        .collect();

    x_ranges.sort_by_key(|r| *r.start());

    x_ranges
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

    let beacon_bound = 4_000_000;

    let mut ans: usize = 0;
    for y in 0..=beacon_bound {
        let ranges_at_row = normalize_ranges(&sensors, y);
        if ranges_at_row.len() > 1 {
            let x = ranges_at_row.first().unwrap().end() + 1;
            ans = (x * beacon_bound + y) as usize;
            break;
        }
    }

    println!("{}", ans);
}
