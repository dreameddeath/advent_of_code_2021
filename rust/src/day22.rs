use crate::utils::Part;
use regex::{Captures, Regex};
use std::{
    collections::BTreeSet, mem::size_of, ops::RangeInclusive, time::Instant,
};

struct Cuboid {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    z: RangeInclusive<i32>,
}

type RealSizeTuple = (i32, u32, i32);
struct AdaptativeGridAxis {
    items: Vec<RealSizeTuple>,
    range: (i32, i32),
}

impl AdaptativeGridAxis {
    fn new<'a, F>(cuboids: &'a [&'a Cuboid], max: i32, f: F) -> AdaptativeGridAxis
    where
        F: Fn(&'a Cuboid) -> &'a RangeInclusive<i32>,
    {
        let mut btree = BTreeSet::new();
        btree.insert(max + 1);
        btree.insert(-max);
        cuboids.iter().map(|cuboid| f(cuboid)).for_each(|range| {
            let start = range.start();
            if *start >= -max && *start <= max {
                btree.insert(*range.start());
            }
            let end = range.end();
            if *end >= -max && *end <= max {
                btree.insert(*range.end() + 1);
            }
        });

        AdaptativeGridAxis {
            items: btree
                .into_iter()
                .collect::<Vec<i32>>()
                .as_slice()
                .windows(2)
                .map(|v| (v[0], v[1]))
                .enumerate()
                .map(|(_, (r1, r2))| (r1, (r2 as i64 - r1 as i64) as u32, r2 - 1))
                .collect(),
            range: (-max, max),
        }
    }

    fn real_range_to_short(&self, range: &RangeInclusive<i32>) -> Option<RangeInclusive<u32>> {
        if *range.end() < self.range.0 {
            return None;
        } else if *range.start() > self.range.1 {
            return None;
        }
        let effective_start = std::cmp::max(*range.start(), self.range.0);
        let effective_end = std::cmp::min(*range.end(), self.range.1);
        let found = self
            .items
            .iter()
            .enumerate()
            .filter(|(_, (rs, _, re))| *rs == effective_start || *re == effective_end)
            .take(2)
            .map(|(pos, _)| pos as u32)
            .collect::<Vec<u32>>();
        return match found.first() {
            None => None,
            Some(first) => match found.last() {
                None => None,
                Some(last) => Some(RangeInclusive::new(*first, *last)),
            },
        };
    }

    fn memory_estimate(&self) -> usize {
        self.items.capacity() * size_of::<RealSizeTuple>()
    }
}

struct Circular<'a, R> {
    vector: &'a Vec<R>,
    iter: std::slice::Iter<'a, R>,
    curr: &'a R,
}

impl<'a, R> Circular<'a, R> {
    fn new(vector: &'a Vec<R>) -> Circular<'a, R> {
        let mut iter = vector.iter();
        let curr = iter.next().unwrap();
        return Circular { vector, iter, curr };
    }
    fn move_next(&mut self) -> bool {
        let nex_opt = self.iter.next();
        let mut has_rotate = false;
        self.curr = match nex_opt {
            Some(v) => v,
            None => {
                has_rotate = true;
                self.iter = self.vector.iter();
                self.iter.next().unwrap()
            }
        };
        has_rotate
    }
}

struct AdaptativeGrid {
    x_coords: AdaptativeGridAxis,
    y_coords: AdaptativeGridAxis,
    z_coords: AdaptativeGridAxis,
    values: Vec<u32>,
}

const PACKED_TYPE_SIZE: u8 = 32;
const PACKED_TYPE_ALL_SET: u32 = u32::MAX;

impl AdaptativeGrid {
    fn new<'a>(cuboids: &Vec<&Cuboid>, max: i32) -> AdaptativeGrid {
        let x_coords = AdaptativeGridAxis::new(cuboids, max, |cuboid| &cuboid.x);
        let y_coords = AdaptativeGridAxis::new(cuboids, max, |cuboid| &cuboid.y);
        let z_coords = AdaptativeGridAxis::new(cuboids, max, |cuboid| &cuboid.z);
        let total_size = x_coords.items.len() * y_coords.items.len() * z_coords.items.len();
        let compact_size = total_size / (PACKED_TYPE_SIZE as usize)
            + if total_size % (PACKED_TYPE_SIZE as usize) > 0 {
                1
            } else {
                0
            };
        let values: Vec<u32> = vec![0; compact_size];

        AdaptativeGrid {
            x_coords,
            y_coords,
            z_coords,
            values,
        }
    }

    fn memory_estimate(&self) -> usize {
        self.values.capacity() * size_of::<u32>()
            + self.z_coords.memory_estimate()
            + self.y_coords.memory_estimate()
            + self.x_coords.memory_estimate()
    }
    fn array_pos(&self, x: u32, y: u32, z: u32) -> (usize, u8) {
        let offset = z as usize * self.y_coords.items.len() * self.x_coords.items.len()
            + y as usize * self.x_coords.items.len()
            + x as usize;

        return (
            offset / (PACKED_TYPE_SIZE as usize),
            (offset % (PACKED_TYPE_SIZE as usize)) as u8,
        );
    }

    fn set(&mut self, cuboid: &Cuboid, v: bool) -> Option<()> {
        let x_range = self.x_coords.real_range_to_short(&cuboid.x)?;
        let y_range = self.y_coords.real_range_to_short(&cuboid.y)?;
        let z_range = self.z_coords.real_range_to_short(&cuboid.z)?;
        let value = if v { PACKED_TYPE_ALL_SET } else { 0 };
        for z in z_range {
            for y in *y_range.start()..=*y_range.end() {
                let start = self.array_pos(*x_range.start(), y, z);
                let end = self.array_pos(*x_range.end(), y, z);

                for i in start.0..=end.0 {
                    self.values[i] = if i == start.0 || i == end.0 {
                        let mask = calc_mask(i, start, end);
                        (self.values[i] & !mask) | (value & mask)
                    } else {
                        value
                    }
                }
            }
        }
        return Some(());
    }

    fn count(&self) -> u64 {
        let mut z_circular = Circular::new(&self.z_coords.items);
        let mut y_circular = Circular::new(&self.y_coords.items);
        let mut x_circular = Circular::new(&self.x_coords.items);
        let mut sum: u64 = 0;

        for v in &self.values {
            for bit in (0..=(PACKED_TYPE_SIZE - 1)).rev() {
                sum += if (v & (1 << bit)) == 0 {
                    0
                } else {
                    z_circular.curr.1 as u64 * y_circular.curr.1 as u64 * x_circular.curr.1 as u64
                };
                let _ = x_circular.move_next() && y_circular.move_next() && z_circular.move_next();
            }
        }

        return sum;
    }
}

fn calc_mask(pos: usize, start: (usize, u8), end: (usize, u8)) -> u32 {
    let mut mask = PACKED_TYPE_ALL_SET;
    if start.0 == pos {
        mask &= PACKED_TYPE_ALL_SET >> start.1;
    }
    if end.0 == pos {
        mask &= PACKED_TYPE_ALL_SET << (31 - end.1)
    }
    mask
}

struct Instruction {
    on: bool,
    cuboid: Cuboid,
}

#[derive(Debug)]
enum ParsingError {
    BadFormat,
}

fn parse_int(captures: &Captures, pos: usize) -> Result<i32, ParsingError> {
    captures
        .get(pos)
        .ok_or(ParsingError::BadFormat)
        .and_then(|v| {
            v.as_str()
                .parse::<i32>()
                .ok()
                .ok_or(ParsingError::BadFormat)
        })
}

fn parse_line(line: &str, parser: &Regex) -> Result<Instruction, ParsingError> {
    let captures = parser.captures(line).ok_or(ParsingError::BadFormat)?;
    let on = captures.get(1).ok_or(ParsingError::BadFormat)?.as_str() == "on";
    let x1 = parse_int(&captures, 2)?;
    let x2 = parse_int(&captures, 3)?;
    let y1 = parse_int(&captures, 4)?;
    let y2 = parse_int(&captures, 5)?;
    let z1 = parse_int(&captures, 6)?;
    let z2 = parse_int(&captures, 7)?;

    return Ok(Instruction {
        on,
        cuboid: Cuboid {
            x: RangeInclusive::new(x1, x2),
            y: RangeInclusive::new(y1, y2),
            z: RangeInclusive::new(z1, z2),
        },
    });
}

fn parse(lines: &Vec<String>) -> Result<Vec<Instruction>, ParsingError> {
    let parser: Regex =
        Regex::new(r"(on|off)\s+x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)")
            .or(Err(ParsingError::BadFormat))?;
    return lines
        .iter()
        .map(|string| parse_line(&string, &parser))
        .collect();
}

pub fn puzzle(part: &Part, lines: &Vec<String>) {
    let instructions = parse(lines).unwrap();
    match part {
        Part::Part1 => {
            let mut grid =
                AdaptativeGrid::new(&instructions.iter().map(|it| &it.cuboid).collect(), 50);
            for instruction in instructions {
                grid.set(&instruction.cuboid, instruction.on);
            }

            let result = grid.count();
            println!("Result Version {}", result)
        }
        Part::Part2 => {
            let mut grid = AdaptativeGrid::new(
                &instructions.iter().map(|it| &it.cuboid).collect(),
                i32::MAX - 1,
            );
            for instruction in instructions {
                grid.set(&instruction.cuboid, instruction.on);
            }
            let start = Instant::now();
            let result = grid.count();
            let duration = start.elapsed().as_millis();
            println!("Result Eval {}, counted in {} ms with memory of {} bytes", result, duration,grid.memory_estimate());
        }
    }
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_calc_mask() {
        assert_eq!(calc_mask(0, (0, 0), (1, 0)), u32::MAX);
        assert_eq!(calc_mask(0, (0, 1), (1, 0)), u32::MAX >> 1);
        assert_eq!(calc_mask(0, (0, 10), (1, 0)), u32::MAX >> 10);
        assert_eq!(calc_mask(0, (0, 31), (1, 0)), 1);

        assert_eq!(calc_mask(1, (0, 0), (1, 0)), u32::MAX << 31);
        assert_eq!(calc_mask(1, (0, 1), (1, 1)), u32::MAX << 30);
        assert_eq!(calc_mask(1, (0, 1), (1, 10)), u32::MAX << 21);
        assert_eq!(calc_mask(1, (0, 1), (1, 31)), u32::MAX);

        assert_eq!(
            calc_mask(1, (1, 1), (1, 30)),
            (u32::MAX >> 1) & (u32::MAX << 1)
        );

        assert_eq!(calc_mask(1, (1, 0), (1, 0)), 1 << 31);
    }

    #[test]
    fn test_simple_instruction() {
        let instructions = [Instruction {
            on: true,
            cuboid: Cuboid {
                x: RangeInclusive::new(-50, -50),
                y: RangeInclusive::new(-50, -50),
                z: RangeInclusive::new(-50, -49),
            },
        }];

        let mut grid = AdaptativeGrid::new(&instructions.iter().map(|it| &it.cuboid).collect(), 50);
        assert_eq!(grid.array_pos(0, 0, 0), (0, 0));
        assert_eq!(grid.array_pos(1, 0, 0), (0, 1));
        assert_eq!(grid.array_pos(0, 1, 0), (0, 2));
        assert_eq!(grid.array_pos(1, 1, 0), (0, 3));

        grid.set(&instructions[0].cuboid, true);

        assert_eq!(grid.values[0], 1 << 31);
        assert_eq!(grid.count(), 2);
    }

    #[test]
    fn test_simple_instruction_ending() {
        let instructions = [Instruction {
            on: true,
            cuboid: Cuboid {
                x: RangeInclusive::new(50, 50),
                y: RangeInclusive::new(50, 50),
                z: RangeInclusive::new(49, 50),
            },
        }];

        let mut grid = AdaptativeGrid::new(&instructions.iter().map(|it| &it.cuboid).collect(), 50);
        grid.set(&instructions[0].cuboid, true);
        assert_eq!(grid.count(), 2);
    }

    #[test]
    fn test_cuboid_embbeded_instruction() {
        let cuboids = [
            Cuboid {
                x: RangeInclusive::new(-50, -50),
                y: RangeInclusive::new(-50, -50),
                z: RangeInclusive::new(-50, -49),
            },
            Cuboid {
                x: RangeInclusive::new(-50, -48),
                y: RangeInclusive::new(-50, -48),
                z: RangeInclusive::new(-50, -48),
            },
        ];

        let mut grid = AdaptativeGrid::new(&cuboids.iter().collect(), 50);

        grid.set(&cuboids[0], true);
        grid.set(&cuboids[1], true);

        assert_eq!(grid.count(), 27);
    }

    #[test]
    fn test_cuboid_embbeded_substract_instruction() {
        let cuboids = [
            Cuboid {
                x: RangeInclusive::new(-50, -48),
                y: RangeInclusive::new(-50, -48),
                z: RangeInclusive::new(-50, -48),
            },
            Cuboid {
                x: RangeInclusive::new(-49, -49),
                y: RangeInclusive::new(-49, -49),
                z: RangeInclusive::new(-49, -49),
            },
        ];

        let mut grid = AdaptativeGrid::new(&cuboids.iter().collect(), 50);

        grid.set(&cuboids[0], true);
        grid.set(&cuboids[1], false);

        assert_eq!(grid.count(), 26);
    }
}
