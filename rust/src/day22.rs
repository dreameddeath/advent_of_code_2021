use crate::utils::Part;
use regex::{Regex, Captures};

use std::{collections::{BTreeSet, btree_set}, ops::RangeInclusive};

struct Cuboid {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    z: RangeInclusive<i32>,
}

struct AdaptativeGridAxis {
    coords: BTreeSet<i32>,
    range:(i32,i32)
}

struct AdaptativeGridAxisCuboidRange<'a>{
    min_max:(i32,i32),
    axis_iter:btree_set::Iter<'a,i32>,
    last_iter_value:Option<&'a i32>,
    next_iter_value:Option<&'a i32>,
    last_value:Option<i32>,
    next_value:Option<i32>,
    normalized_coord:u32,
}

impl<'a> AdaptativeGridAxisCuboidRange<'a>{
    fn new(cuboid_range:&'a RangeInclusive<i32>,axis:&'a AdaptativeGridAxis)->AdaptativeGridAxisCuboidRange<'a>{
        let min_max = if cuboid_range.end()<&axis.range.0 || cuboid_range.start()>&axis.range.1 {
            (i32::MAX,i32::MIN)
        }
        else{
            (std::cmp::max(*cuboid_range.start(),axis.range.0),std::cmp::min(*cuboid_range.end(),axis.range.1))
        };

        let res = AdaptativeGridAxisCuboidRange{
            min_max,
            axis_iter:axis.coords.iter(),
            last_iter_value:None,
            next_iter_value:None,
            last_value:None,
            next_value:None,
            normalized_coord:0
        };
        res.axis_iter_next();
        res
    }

    fn axis_iter_next(&mut self)->Option<()>{
        if let None = self.last_iter_value {
            self.normalized_coord = 0;
            self.next_iter_value = self.axis_iter.next();
        }
        else{
            self.normalized_coord+=1;
        }
        self.last_iter_value = self.next_iter_value;
        self.next_iter_value = self.axis_iter.next();
        if let None = self.next_iter_value {
            return None;
        }
        else{
            return Some(())
        }
    }
}

impl<'a> Iterator for AdaptativeGridAxisCuboidRange<'a> {
    type Item = (u32,u32);

    
    
    fn next(&mut self) -> Option<Self::Item> {

        loop{
            let start =self.last_value.or_else(||self.axis_iter.next().map(|v| *v))?;

            let next = std::cmp::min(self.cuboid_range.end(),self.axis_iter.next()?);
            self.normalized_coord+=1;
            if next<self.cuboid_range.start(){
                continue;
            }
            let curr_coord = self.normalized_coord;
            self.last_value=Some(*next);
        }
        
    }

    
    
}

impl AdaptativeGridAxis {
    fn new<F>(cuboids:&Vec<&Cuboid>,max:i32, f:F) -> AdaptativeGridAxis where F:Fn(&Cuboid)->&RangeInclusive<i32> {
        let btree = BTreeSet::new();
        btree.insert(max);
        btree.insert(-max);
        cuboids.iter().map(|cuboid| f(cuboid)).for_each(|range|{
            let start = range.start();
            if *start>=-max && *start<=max {
                btree.insert(*range.start());
            }
            let end = range.end();
            if *end>=-max && *end<=max{
                btree.insert(*range.end());
            }    
        });

        AdaptativeGridAxis {
            coords: btree,
            range:(-max,max)
        }

    }

    fn normalized_iter(&self,range:RangeInclusive<i32>)->
}

struct AdaptativeGrid {
    x_coords: AdaptativeGridAxis,
    y_coords: AdaptativeGridAxis,
    z_coords: AdaptativeGridAxis,
    values:Vec<u32>
}

impl AdaptativeGrid {
    fn new(cuboids:&Vec<&Cuboid>, max:i32) -> AdaptativeGrid {
        let x_coords= AdaptativeGridAxis::new(cuboids,max,|cuboid| &cuboid.x);
        let y_coords= AdaptativeGridAxis::new(cuboids,max,|cuboid| &cuboid.y);
        let z_coords= AdaptativeGridAxis::new(cuboids,max,|cuboid| &cuboid.z);
        let total_size = x_coords.coords.len()*y_coords.coords.len()*z_coords.coords.len();
        let compact_size = total_size / 32 + if total_size%32 > 0 { 1 } else { 0 };
        let values:Vec<u32> = vec![0;compact_size];
        
        AdaptativeGrid {
            x_coords,
            y_coords,
            z_coords,
            values
        }
    }
}




struct Instruction{
    on:bool,
    cuboid:Cuboid
}

#[derive(Debug)]
enum ParsingError {
    BadFormat
}

static parser:Regex = Regex::new(r"(on|off)\s+x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)").unwrap();

fn parse_int(captures:&Captures,pos:usize)->Result<i32,ParsingError>{
    captures.get(pos).ok_or(ParsingError::BadFormat)
        .and_then(|v| v.as_str().parse::<i32>().or_else(ParsingError::BadFormat))
}

fn parse_line(line: &str) -> Result<Instruction, ParsingError> {
    let captures = parser.captures(line).ok_or(ParsingError::BadFormat)?;
    let on = captures.get(1).ok_or(ParsingError::BadFormat)? == "on";
    let x1 = parse_int(&captures,2)?;
    let x2 = parse_int(&captures,3)?;
    let y1 = parse_int(&captures,4)?;
    let y2 = parse_int(&captures,5)?;
    let z1 = parse_int(&captures,6)?;
    let z2 = parse_int(&captures,7)?;

    return Ok(Instruction{
        on,
        cuboid:Cuboid{
            x:RangeInclusive::new(x1,x2),
            y:RangeInclusive::new(y1,y2),
            z:RangeInclusive::new(z1,z2),
        }
    })
}

fn parse(lines: &Vec<String>) -> Result<Vec<Instruction>, ParsingError> {
    return lines.iter().map(|string| parse_line(&string)).collect();
}


pub fn puzzle(part: &Part, lines: &Vec<String>) {
    let instructions = parse(lines).unwrap();
    match part {
        Part::Part1 => {
            let grid = AdaptativeGrid::new(&instructions.iter().map(|it| &it.cuboid).collect(),50);
            for instruction in instructions{
                for x in grid.x_coords.normalized_iter(instruction.cuboid.x) {
                    for y in grid.y_coords.normalized_iter(instruction.cuboid.y) {
                        for z in grid.x_coords.normalized_iter(instruction.cuboid.z) {
                        }
                    }
                }
            }
            println!("Result Version {}", mag)
        }
        Part::Part2 => {
            let snailfish_pairs_ref = &snailfish_pairs;
            let combinations: Vec<u32> = snailfish_pairs_ref
                .iter()
                .flat_map(|i1| {
                    snailfish_pairs_ref.iter().map(move |i2| {
                        if std::ptr::eq(i1, i2) {
                            0
                        } else {
                            magnitude(&sum(i1.clone(), i2.clone(), false))
                        }
                    })
                })
                .collect();
            let max = combinations
                .iter()
                .fold(0u32, |curr, val| std::cmp::max(curr, *val));
            println!("Result Eval {}", max);
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_explode_simple_left() {
        let mut value = parse_line("[[[[[9,8],1],2],3],4]").unwrap();
        let res = explode(&mut value, 0);
        assert_eq!(
            res,
            Some(ExplodeResult::Exploded(
                Some(ExplodeAction::PendingLeft(9)),
                None
            ))
        );
        assert_eq!(value.to_string(), "[[[[0,9],2],3],4]")
    }

    #[test]
    fn test_explode_simple_right() {
        let mut value = parse_line("[7,[6,[5,[4,[3,2]]]]]").unwrap();
        let res = explode(&mut value, 0);
        assert_eq!(
            res,
            Some(ExplodeResult::Exploded(
                None,
                Some(ExplodeAction::PendingRight(2))
            ))
        );
        assert_eq!(value.to_string(), "[7,[6,[5,[7,0]]]]")
    }

    #[test]
    fn test_explode_simple_middle() {
        let mut value = parse_line("[[6,[5,[4,[3,2]]]],1]").unwrap();
        let res = explode(&mut value, 0);
        assert_eq!(res, Some(ExplodeResult::Exploded(None, None)));
        assert_eq!(value.to_string(), "[[6,[5,[7,0]]],3]")
    }
    #[test]
    fn test_explode_priority_left() {
        let mut value = parse_line("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").unwrap();
        let res = explode(&mut value, 0);
        assert_eq!(res, Some(ExplodeResult::Exploded(None, None)));
        assert_eq!(value.to_string(), "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");

        //Second time
        let res2 = explode(&mut value, 0);
        assert_eq!(
            res2,
            Some(ExplodeResult::Exploded(
                None,
                Some(ExplodeAction::PendingRight(2))
            ))
        );
        assert_eq!(value.to_string(), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    }

    #[test]
    fn test_sum() {
        let sum_res = sum(
            parse_line("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap(),
            parse_line("[1,1]").unwrap(),
            false,
        );
        assert_eq!(sum_res.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn test_complex() {
        let complex_test: Vec<&str> = vec![
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]",
        ];
        let items_res: Result<Vec<SnailFishItem>, ParsingError> =
            complex_test.iter().map(|line| parse_line(*line)).collect();
        let items = items_res.unwrap();
        let mut intermediates: Vec<String> = Vec::with_capacity(items.len());
        let final_result = items
            .into_iter()
            .reduce(|src, dest| {
                let intermediate = sum(src, dest, false);
                intermediates.push(intermediate.to_string());
                intermediate
            })
            .unwrap();
        assert_eq!(
            intermediates,
            vec![
                "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
                "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
                "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]",
                "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]",
                "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]",
                "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]",
                "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]",
                "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]",
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            ]
        );

        assert_eq!(
            final_result.to_string(),
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
        );
    }

    #[test]
    fn test_magnitude_calc() {
        let item =
            parse_line("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]").unwrap();
        let result = magnitude(&item);

        assert_eq!(result, 4140);
    }

    impl SnailFishItem {
        fn to_string(&self) -> String {
            format!("{}", self)
        }
    }
}
