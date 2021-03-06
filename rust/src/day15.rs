use crate::utils::Part;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
//use std::collections::HashSet;
use hashbrown::HashSet;
use priority_queue::PriorityQueue;
use hashbrown::hash_map::DefaultHashBuilder;

struct Map {
    lines: Vec<Vec<u8>>,
    max_x: u16,
    max_y: u16,
}

impl Map {
    fn new(content: Vec<Vec<u8>>) -> Map {
        let max_x = content.get(0).map(|line| line.len() - 1).unwrap_or(0) as u16;
        let max_y = (content.len() - 1) as u16;
        return Map {
            lines: content,
            max_x: max_x,
            max_y: max_y,
        };
    }
}

#[derive(PartialEq, Hash, Eq, Clone, Copy)]
struct Pos {
    x: u16,
    y: u16,
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.cmp(&other.x).then(self.y.cmp(&other.y))
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq,Hash)]
struct WeightedPos {
    pos: Pos,
    previous: Option<Pos>,
    cost_from_origin: u16,
    estimated_total_cost: u16,
}

impl Ord for WeightedPos {
    fn cmp(&self, other: &Self) -> Ordering {
        other.estimated_total_cost.cmp(&self.estimated_total_cost)
    }
}

impl PartialOrd for WeightedPos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_line(line: &String) -> Option<Vec<u8>> {
    let chars: Vec<&str> = line.split("").collect();
    let result = chars
        .iter()
        .filter_map(|chr| chr.parse().ok())
        .collect::<Vec<u8>>();
    return if result.len() == line.len() {
        Some(result)
    } else {
        None
    };
}

fn to_lookup(node: &WeightedPos, map: &Map) -> Vec<WeightedPos> {
    return [
        if node.pos.x == 0 {
            None
        } else {
            Some(Pos {
                x: node.pos.x - 1,
                y: node.pos.y,
            })
        },
        if node.pos.x > map.max_x {
            None
        } else {
            Some(Pos {
                x: node.pos.x + 1,
                y: node.pos.y,
            })
        },
        if node.pos.y == 0 {
            None
        } else {
            Some(Pos {
                x: node.pos.x,
                y: node.pos.y - 1,
            })
        },
        if node.pos.y > map.max_y {
            None
        } else {
            Some(Pos {
                x: node.pos.x,
                y: node.pos.y + 1,
            })
        },
    ]
    .iter()
    .filter_map(|&it| it)
    .filter_map(|pos| {
        map.lines
            .get(pos.y as usize)
            .and_then(|line| line.get(pos.x as usize))
            .map(|val| (pos, val))
    })
    .map(|tuple| {
        let cost = node.cost_from_origin + u16::from(*tuple.1);
        WeightedPos {
            pos: tuple.0.clone(),
            cost_from_origin: cost,
            estimated_total_cost: calc_estimated_total_cost(&cost, &tuple.0, map),
            previous: Some(node.pos),
        }
    })
    .collect();
}

fn calc_estimated_total_cost(cost: &u16, pos: &Pos, map: &Map) -> u16 {
    cost + (map.max_x - pos.x) + (map.max_y - pos.y)
}

fn a_star_lookup(map: &Map) -> u16 {
    //let mut processed: HashSet<Pos> = HashSet::new();
    let mut processed = vec![false;(map.max_x as usize +1)*(map.max_y as usize +1)];
    let calc_offset = |pos:&Pos| pos.x as usize + (pos.y as usize *(map.max_x as usize+1));
    let mut priority_queue: BinaryHeap<WeightedPos> = BinaryHeap::new();
    priority_queue.push(WeightedPos {
        pos: Pos { x: 0, y: 0 },
        cost_from_origin: 0,
        previous: None,
        estimated_total_cost: calc_estimated_total_cost(&0, &Pos { x: 0, y: 0 }, map),
    });

    while let Some(item) = priority_queue.pop() {
        let offset = calc_offset(&item.pos);
        if processed[offset] {
            continue;
        }
        if item.pos.x == map.max_x && item.pos.y == map.max_y {
            println!("Processed node {}", processed.len());
            return item.cost_from_origin;
        }

        let mut new_nodes = to_lookup(&item, map);
        while let Some(new_node) = new_nodes.pop() {
            if processed[calc_offset(&new_node.pos)] { 
                continue;
            }
            priority_queue.push(new_node);
        }
        processed[offset]=true;
    }
    return 0;
}

fn parse(lines: &Vec<String>) -> Map {
    let content: Vec<Vec<u8>> = lines
        .into_iter()
        .filter_map(|line| parse_line(&line))
        .collect();
    return Map::new(content);
}

fn extend_map(map: &Map) -> Map {
    let mut vec: Vec<Vec<u8>> = Vec::with_capacity(map.lines.len() * 5);
    for y in 0..5 {
        for orig_line in &map.lines {
            let mut vec_line: Vec<u8> = Vec::with_capacity(orig_line.len() * 5);
            for x in 0..5 {
                for val in orig_line {
                    vec_line.push((val - 1 + x as u8 + y as u8) % 9 + 1)
                }
            }
            vec.push(vec_line);
        }
    }
    return Map::new(vec);
}

pub fn puzzle(part: &Part, lines: &Vec<String>) {
    let map = parse(lines);
    match part {
        Part::Part1 => {
            let result = a_star_lookup(&map);
            println!("Result {}", result)
        }
        Part::Part2 => {
            let new_map = extend_map(&map);
            let result = a_star_lookup(&new_map);
            println!("Result {}", result);
        }
    }
}
