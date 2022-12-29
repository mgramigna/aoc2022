use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Item {
    Rock,
    SandSource,
    Air,
    Sand,
}

#[derive(Debug, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Cave {
    grid: Vec<Vec<Item>>,
    size_x: usize,
    size_y: usize,
    sand_source: Coord,
}

impl FromStr for Coord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str) = s.split_once(",").unwrap();

        Ok(Self {
            x: x_str.parse().unwrap(),
            y: y_str.parse().unwrap(),
        })
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for row in &self.grid {
            for col in row {
                match col {
                    Item::Rock => s += "# ",
                    Item::SandSource => s += "+ ",
                    Item::Air => s += ". ",
                    Item::Sand => s += "o ",
                }
            }
            s += "\n"
        }

        write!(f, "{}", s)
    }
}

const FLOOR_EXPANSION_SIZE: usize = 10;

impl Cave {
    fn new(size_x: usize, size_y: usize, sand_x: usize, sand_y: usize) -> Self {
        let grid: Vec<Vec<Item>> = vec![vec![Item::Air; size_x]; size_y];

        Self {
            grid,
            size_x,
            size_y,
            sand_source: Coord {
                x: sand_x,
                y: sand_y,
            },
        }
    }

    fn expand_floor(&mut self) {
        for (i, row) in self.grid.iter_mut().enumerate() {
            let item = if i == self.size_y - 1 {
                Item::Rock
            } else {
                Item::Air
            };
            row.splice(0..0, vec![item; FLOOR_EXPANSION_SIZE / 2]);
            row.splice(row.len().., vec![item; FLOOR_EXPANSION_SIZE / 2]);
        }
    }

    fn fill(&mut self, lines: &Vec<Vec<Coord>>) {
        for line in lines {
            let mut coords = line.iter();

            let mut curr = coords.next();
            while let Some(c) = curr {
                self.grid[c.y][c.x] = Item::Rock;

                if let Some(next) = coords.next() {
                    curr = Some(next);
                    if c.x == next.x {
                        if c.y < next.y {
                            for y in c.y + 1..next.y + 1 {
                                self.grid[y][c.x] = Item::Rock;
                            }
                        } else {
                            for y in next.y..c.y + 1 {
                                self.grid[y][c.x] = Item::Rock;
                            }
                        }
                    } else if c.y == next.y {
                        if c.x < next.x {
                            for x in c.x + 1..next.x + 1 {
                                self.grid[c.y][x] = Item::Rock;
                            }
                        } else {
                            for x in next.x..c.x + 1 {
                                self.grid[c.y][x] = Item::Rock;
                            }
                        }
                    }
                } else {
                    break;
                }
            }
        }

        for x in 0..self.size_x {
            self.grid[self.size_y - 1][x] = Item::Rock;
        }

        self.grid[self.sand_source.y][self.sand_source.x] = Item::SandSource;
    }

    fn produce_sand(&mut self) -> bool {
        let mut sand_coord = Coord {
            x: self.sand_source.x,
            y: self.sand_source.y,
        };

        loop {
            if self.grid[sand_coord.y + 1][sand_coord.x] == Item::Air {
                sand_coord.y += 1;
            } else if self.grid[sand_coord.y + 1][sand_coord.x - 1] == Item::Air {
                sand_coord.y += 1;
                sand_coord.x -= 1;
            } else if self.grid[sand_coord.y + 1][sand_coord.x + 1] == Item::Air {
                sand_coord.y += 1;
                sand_coord.x += 1;
            } else {
                self.grid[sand_coord.y][sand_coord.x] = Item::Sand;

                if sand_coord == self.sand_source {
                    return true;
                }

                break;
            }

            if sand_coord.y + 1 == self.size_y {
                break;
            }

            if sand_coord.x + 1 == self.size_x || sand_coord.x == 0 {
                self.expand_floor();
                self.sand_source.x += FLOOR_EXPANSION_SIZE / 2;
                sand_coord.x += FLOOR_EXPANSION_SIZE / 2;
            }
        }
        false
    }
}

fn main() {
    let input = include_str!("../inputs/day14.txt");

    let lines: Vec<Vec<Coord>> = input
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|s| s.parse::<Coord>().unwrap())
                .collect()
        })
        .collect();

    let min_x: usize = lines.iter().flatten().map(|c| c.x).min().unwrap();
    let max_y: usize = lines.iter().flatten().map(|c| c.y).max().unwrap();

    let normalized_lines: Vec<Vec<Coord>> = lines
        .iter()
        .map(|l| {
            l.iter()
                .map(|c| Coord {
                    x: c.x - min_x,
                    y: c.y,
                })
                .collect()
        })
        .collect();

    let max_x: usize = normalized_lines
        .iter()
        .flatten()
        .map(|c| c.x)
        .max()
        .unwrap();

    let mut cave: Cave = Cave::new(max_x + 1, max_y + 3, 500 - min_x, 0);

    cave.fill(&normalized_lines);

    let mut rested = 1;
    loop {
        let voided = cave.produce_sand();
        if voided {
            break;
        }

        rested += 1;
    }

    println!("{}", rested);
}
