use crate::utils::Part;
use regex::{Captures, Regex};
use std::collections::{BinaryHeap};
use std::fmt::Write;
use hashbrown::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct World {
    hallway: u32,    // 11*3bits = 33 !!!
    rooms: [u16; 4], // 4*3bits = 12 + 4bits of bad positionning
}

const POD_A: u8 = 0b00;
const POD_B: u8 = 0b01;
const POD_C: u8 = 0b10;
const POD_D: u8 = 0b11;
const POD_A_CELL: u8 = POD_A << 1 | 0b1;
const POD_B_CELL: u8 = POD_B << 1 | 0b1;
const POD_C_CELL: u8 = POD_C << 1 | 0b1;
const POD_D_CELL: u8 = POD_D << 1 | 0b1;

const COST: [u32; 4] = [1, 10, 100, 1000];
const HALLWAY_USEFULL_POS: [u8; 7] = [0, 1, 3, 5, 7, 9, 10];
const HALLWAY_SHIFTS: [u8; 11] = [11, 13, 0xFF, 15, 0xFF, 17, 0xFF, 19, 0xFF, 21, 23];
const H_POS_MAX: u8 = 10;
const HALLWAY_FULL_MASK: u32 = (1 << (H_POS_MAX + 1)) - 1;

fn get_room_movable_pod_and_depth(world: &World, max_depth: u8, room: u8) -> Option<(u8, u8)> {
    let curr_room = &world.rooms[room as usize];
    if curr_room & 0b1111 == 0 {
        return None;
    }
    for curr_depth in 0..=max_depth {
        let bad_bits = *curr_room & (0b1 << curr_depth);
        if bad_bits != 0 {
            let curr_data = (*curr_room >> (curr_depth * 3 + 5)) & 0b11;
            return Some((curr_data as u8, curr_depth));
        }
    }
    return None;
}
fn is_room_available(world: &World, room: u8) -> bool {
    (world.rooms[room as usize] & 0b1111) == 0
}

fn get_room_available_for_set_with_depth(world: &World, max_depth: u8, room: u8) -> u8 {
    get_room_available_for_set_with_depth_from_room(&world.rooms[room as usize], max_depth)
}

fn get_room_available_for_set_with_depth_from_room(curr_room: &u16, max_depth: u8) -> u8 {
    for curr_depth in (0..=max_depth).rev() {
        let curr_data = (*curr_room >> (curr_depth * 3 + 4)) & 0b111;
        if curr_data == 0 {
            return curr_depth;
        }
    }
    //print_world(world, max_depth);
    panic!("should not occurs")
}

fn set_room_with_pod_for_init(world: &mut World, room: u8, pod: u8, depth: u8) {
    let curr_room = &mut world.rooms[room as usize];
    let is_already_bad = (*curr_room & 0b1111) != 0;
    let mask = 0b1 << depth | 0b111 << (depth * 3 + 4);
    let value_to_set = (calc_cell_value(pod) as u16) << (depth * 3 + 4);
    let bit_bad_pod = (if is_already_bad || pod != room {
        0b1
    } else {
        0b0
    }) << depth;
    *curr_room = (*curr_room & !mask) | value_to_set | bit_bad_pod;
}

fn calc_cell_value(pod: u8) -> u8 {
    (pod << 1) | 0b1
}

fn set_room_with_pod(world: &mut World, room: u8, depth: u8, pod: u8) {
    let curr_room = &mut world.rooms[room as usize];
    let new_value = (calc_cell_value(pod) as u16) << (depth * 3 + 4);
    *curr_room = *curr_room | new_value;
}

fn clear_room_pod(world: &mut World, room: u8, depth: u8) {
    let curr_room = &mut world.rooms[room as usize];
    let mask: u16 = 0b1 << depth | 0b111 << (depth * 3 + 4);
    *curr_room = *curr_room & !mask;
}

fn set_hallway_pod(world: &mut World, pos: u8, pod: u8) {
    world.hallway |= (pod as u32) << HALLWAY_SHIFTS[pos as usize] | 1 << pos;
}

fn clear_hallway_pod(world: &mut World, pos: u8) {
    world.hallway = world.hallway & !(0b11 << HALLWAY_SHIFTS[pos as usize] | (1 << pos));
}

fn is_hallway_move_possible_excluded(
    world: &World,
    start_pos_exluded: u8,
    end_pos_excluded: u8,
) -> bool {
    let start_pos = std::cmp::min(start_pos_exluded, end_pos_excluded) + 1;
    let end_pos = std::cmp::max(start_pos_exluded, end_pos_excluded) - 1;
    return is_hallway_move_possible_ordered(world, start_pos, end_pos);
}

fn is_hallway_move_possible_included(
    world: &World,
    start_pos_included: u8,
    end_pos_included: u8,
) -> bool {
    let start_pos = std::cmp::min(start_pos_included, end_pos_included);
    let end_pos = std::cmp::max(start_pos_included, end_pos_included);
    return is_hallway_move_possible_ordered(world, start_pos, end_pos);
}

fn is_hallway_move_possible_ordered(world: &World, start_pos: u8, end_pos: u8) -> bool {
    let start_mask: u32 = HALLWAY_FULL_MASK << start_pos;
    let end_mask: u32 = HALLWAY_FULL_MASK >> (10 - end_pos);
    return (world.hallway & start_mask & end_mask) == 0;
}

fn get_hallway_movable_cells(world: &World) -> Vec<(u8, u8)> {
    HALLWAY_USEFULL_POS
        .into_iter()
        .filter(|pos| (world.hallway & (0b1 << *pos)) != 0)
        .map(|pos| {
            (
                pos,
                ((world.hallway >> (HALLWAY_SHIFTS[pos as usize])) & 0b11) as u8,
            )
        })
        .filter(|(pos, v)| {
            is_hallway_move_possible_excluded(world, *pos, calc_room_to_hallway_pos(*v))
                && is_room_available(world, *v)
        })
        .collect()
}

#[derive(Debug)]
enum ParsingError {
    BadFormat,
}

fn parse_amphipod(captures: &Captures, pos: usize) -> Result<u8, ParsingError> {
    captures
        .get(pos)
        .ok_or(ParsingError::BadFormat)
        .and_then(|v| match v.as_str() {
            "A" => Ok(POD_A),
            "B" => Ok(POD_B),
            "C" => Ok(POD_C),
            "D" => Ok(POD_D),
            _ => Err(ParsingError::BadFormat),
        })
}

fn parse_line(line: &str, parser: &Regex) -> Result<(u8, u8, u8, u8), ParsingError> {
    let captures = parser.captures(line).ok_or(ParsingError::BadFormat)?;
    let a = parse_amphipod(&captures, 1)?;
    let b = parse_amphipod(&captures, 2)?;
    let c = parse_amphipod(&captures, 3)?;
    let d = parse_amphipod(&captures, 4)?;
    return Ok((a, b, c, d));
}

fn parse_pods(lines: &Vec<String>) -> Result<Vec<(u8, u8, u8, u8)>, ParsingError> {
    let parser: Regex =
        Regex::new(r"^..#(\w)#(\w)#(\w)#(\w)#.?.?$").or(Err(ParsingError::BadFormat))?;

    return lines
        .iter()
        .skip(2)
        .take(lines.len() - 3)
        .map(|line| parse_line(line, &parser))
        .collect();
}

fn parse(lines: &Vec<String>, target_world: &mut World) -> Result<(), ParsingError> {
    let pods = parse_pods(lines)?;

    pods.iter()
        .enumerate()
        .rev()
        .for_each(|(depth, (a, b, c, d))| {
            set_room_with_pod_for_init(target_world, POD_A, *a, depth as u8);
            set_room_with_pod_for_init(target_world, POD_B, *b, depth as u8);
            set_room_with_pod_for_init(target_world, POD_C, *c, depth as u8);
            set_room_with_pod_for_init(target_world, POD_D, *d, depth as u8);
        });

    return Ok(());
}

fn default_room(room: u8, max_depth: u8) -> u16 {
    if max_depth == 3 {
        return 0;
    } else {
        let used_cell = (room << 1 | 0b1) as u16;
        return used_cell << (4 + 2 * 3) | used_cell << (4 + 3 * 3);
    }
}

fn init_world(depth: u8) -> World {
    World {
        hallway: 0,
        rooms: [
            default_room(0, depth),
            default_room(1, depth),
            default_room(2, depth),
            default_room(3, depth),
        ],
    }
}

#[derive(Eq, PartialEq)]
struct WorldToEvaluate {
    cost: u32,
    estimated_cost: u32,
    world: World,
    move_data: Option<Move>,
}

impl Ord for WorldToEvaluate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.estimated_cost.cmp(&self.estimated_cost)
    }
}

impl PartialOrd for WorldToEvaluate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.estimated_cost.partial_cmp(&self.estimated_cost)
    }
}

fn abs_diff(a: u8, b: u8) -> u8 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn calc_cost_info(depth: u8, room: u8, pos: u8) -> u32 {
    (depth + 1 + abs_diff(pos, calc_room_to_hallway_pos(room))) as u32
}

fn calc_effective_cost_info_between_room(depth: u8, room: u8, pod: u8) -> i32 {
    if room == pod {
        (depth + 4) as i32
    } else {
        (depth
            + 2
            + abs_diff(
                calc_room_to_hallway_pos(room),
                calc_room_to_hallway_pos(pod),
            )) as i32
    }
}

fn calc_effective_cost_info_from_hallway(pos: u8, room: u8) -> i32 {
    (1 + abs_diff(pos, calc_room_to_hallway_pos(room))) as i32
}

fn apply_move(world: &World, move_data: &Move) -> (World, u32, i32) {
    let mut new_world = World {
        hallway: world.hallway,
        rooms: world.rooms,
    };
    match move_data {
        &Move::ToHallway {
            depth,
            pod,
            pos,
            room,
        } => {
            let cost = calc_cost_info(depth, room, pos);
            let effective_cost_offset = cost as i32
                + calc_effective_cost_info_from_hallway(pos, pod)
                - calc_effective_cost_info_between_room(depth, room, pod);
            clear_room_pod(&mut new_world, room, depth);
            set_hallway_pod(&mut new_world, pos, pod);
            return (
                new_world,
                cost * COST[pod as usize],
                effective_cost_offset * (COST[pod as usize] as i32),
            );
        }
        &Move::ToRoom {
            depth,
            room,
            pod,
            pos,
        } => {
            let cost = calc_cost_info(depth, pod, pos);
            let effective_cost_offset =
                cost as i32 - calc_effective_cost_info_from_hallway(pos, pod);
            clear_hallway_pod(&mut new_world, pos);
            set_room_with_pod(&mut new_world, room, depth, pod);
            return (
                new_world,
                cost * COST[pod as usize],
                effective_cost_offset * COST[pod as usize] as i32,
            );
        }
    }
}

#[derive(PartialEq, Eq)]
enum Move {
    ToHallway {
        pod: u8,
        depth: u8,
        room: u8,
        pos: u8,
    },
    ToRoom {
        pod: u8,
        depth: u8,
        pos: u8,
        room: u8,
    },
}

fn build_world_to_evaluate(world_ctxt: &WorldToEvaluate, move_data: Move) -> WorldToEvaluate {
    let (world, cost, estimated_cost) = apply_move(&world_ctxt.world, &move_data);
    WorldToEvaluate {
        cost: world_ctxt.cost + cost,
        estimated_cost: (world_ctxt.estimated_cost as i32 + estimated_cost) as u32,
        world,
        move_data: Some(move_data),
    }
}

fn calc_room_to_hallway_pos(room: u8) -> u8 {
    (room + 1) * 2
}

fn calc_next_worlds_v1(world_ctxt: &WorldToEvaluate, max_depth: u8) -> Vec<WorldToEvaluate> {
    let world = &world_ctxt.world;
    let mut result = Vec::<WorldToEvaluate>::with_capacity(7);
    if world_ctxt.world.hallway != 0 {
        for (pos, pod) in get_hallway_movable_cells(world) {
            let depth = get_room_available_for_set_with_depth(world, max_depth, pod);
            result.push(build_world_to_evaluate(
                world_ctxt,
                Move::ToRoom {
                    depth,
                    pod,
                    pos,
                    room: pod,
                },
            ))
        }
    }

    for room in POD_A..=POD_D {
        if let Some((pod, depth)) = get_room_movable_pod_and_depth(world, max_depth, room) {
            for pos in [0, 1, 3, 5, 7, 9, 10] {
                if is_hallway_move_possible_included(world, calc_room_to_hallway_pos(room), pos) {
                    result.push(build_world_to_evaluate(
                        world_ctxt,
                        Move::ToHallway {
                            depth,
                            pod,
                            pos,
                            room,
                        },
                    ))
                }
            }
        }
    }
    result
}

fn cell_to_char(curr: u8) -> char {
    match curr {
        POD_A_CELL => 'A',
        POD_B_CELL => 'B',
        POD_C_CELL => 'C',
        POD_D_CELL => 'D',
        _ => '.',
    }
}

fn val_to_char(curr: u8) -> char {
    match curr {
        POD_A => 'A',
        POD_B => 'B',
        POD_C => 'C',
        POD_D => 'D',
        _ => '.',
    }
}

fn hallway_to_str(world: &World) -> String {
    let mut h_str = String::new();
    write!(
        &mut h_str,
        "#{}#",
        (0..=H_POS_MAX)
            .into_iter()
            .map(|pos| if world.hallway & (0b1 << pos) == 0 {
                '.'
            } else {
                val_to_char((world.hallway >> (HALLWAY_SHIFTS[pos as usize]) & 0b11) as u8)
            })
            .collect::<String>()
    )
    .unwrap();
    return h_str;
}

fn room_depth_to_str(world: &World, depth: u8) -> String {
    let room_and_depth_to_str = |r: u8, d: u8| -> char {
        cell_to_char(((world.rooms[r as usize] >> (4 + 3 * d)) & 0b111) as u8)
    };
    let mut r = String::new();
    write!(
        &mut r,
        "  #{}#{}#{}#{}#  ",
        room_and_depth_to_str(POD_A, depth),
        room_and_depth_to_str(POD_B, depth),
        room_and_depth_to_str(POD_C, depth),
        room_and_depth_to_str(POD_D, depth),
    )
    .unwrap();
    return r;
}

fn world_to_string(world: &World, max_depth: u8) -> String {
    let mut result: Vec<String> = vec![String::from("#############"), hallway_to_str(world)];
    for d in 0..=max_depth {
        result.push(room_depth_to_str(world, d))
    }
    return result.join("\n");
}

fn is_world_finished(world: &World) -> bool {
    world.hallway == 0 && world.rooms.iter().all(|room| (room & 0b1111) == 0)
}

fn print_world(world: &World, max_depth: u8) {
    println!("{}", world_to_string(world, max_depth));
}

fn rebuild_history(
    last_world_data: &WorldToEvaluate,
    processed: &HashMap<World, (Option<Move>, u32)>,
    max_depth: u8,
) {
    let mut last_move = &last_world_data.move_data;
    let mut last_world = World {
        hallway: last_world_data.world.hallway,
        rooms: last_world_data.world.rooms,
    };

    let mut last_cost = last_world_data.cost;
    print_world(&last_world, max_depth);
    while let Some(move_data) = last_move {
        println!("Cout {}", last_cost);
        let invert_move = match move_data {
            Move::ToHallway {
                depth,
                pod,
                pos,
                room,
            } => Move::ToRoom {
                depth: *depth,
                pod: *pod,
                pos: *pos,
                room: *room,
            },
            Move::ToRoom {
                depth,
                pod,
                pos,
                room,
            } => Move::ToHallway {
                depth: *depth,
                pod: *pod,
                pos: *pos,
                room: *room,
            },
        };
        let (new_world, _, _) = apply_move(&last_world, &invert_move);
        last_world = new_world;
        print_world(&last_world, max_depth);
        let (move_data, cost) = processed.get(&last_world).unwrap();
        last_move = move_data;
        last_cost = *cost;
    }
}

fn calc_init_estimated_cost(world: &World, max_depth: u8) -> u32 {
    let mut sum: u32 = 0;
    'next_room: for room in POD_A..=POD_D {
        let curr_room = world.rooms[room as usize];
        for depth in 0..=max_depth {
            if curr_room & (0b1 << depth) == 0 {
                continue 'next_room;
            }
            let pod = ((curr_room >> (depth * 3 + 5)) & 0b11) as u8;
            sum +=
                calc_effective_cost_info_between_room(depth, room, pod) as u32 * COST[pod as usize];
        }
    }
    return sum;
}
const DEBUG: bool = false;
fn solve(start_world: World, max_depth: u8) -> Option<u32> {
    let mut processed: HashMap<World, (Option<Move>, u32)> = HashMap::new();
    let mut priority_queue: BinaryHeap<WorldToEvaluate> = BinaryHeap::new();
    priority_queue.push(WorldToEvaluate {
        cost: 0,
        estimated_cost: calc_init_estimated_cost(&start_world, max_depth),
        world: start_world,
        move_data: None,
    });

    while let Some(next) = priority_queue.pop() {
        if DEBUG {
            println!("Processing \n{}", world_to_string(&next.world, max_depth));
        }
        if is_world_finished(&next.world) {
            println!("Noeuds traités {}", processed.len());
            if DEBUG {
                rebuild_history(&next, &processed, max_depth);
            }
            return Some(next.cost);
        }
        for next_world in calc_next_worlds_v1(&next, max_depth) {
            if !processed.contains_key(&next_world.world) {
                if DEBUG {
                    println!(
                        "Queuing \n{}",
                        world_to_string(&next_world.world, max_depth)
                    );
                }
                priority_queue.push(next_world);
            }
        }
        processed.insert(next.world, (next.move_data, next.cost));
    }
    return None;
}

pub fn puzzle(part: &Part, lines: &Vec<String>) {
    let depth: u8 = if let Part::Part1 = part { 1 } else { 3 };
    let mut world = init_world(depth);
    let effective_lines = if let Part::Part1 = part {
        lines.clone()
    } else {
        let mut new_lines = lines.clone();
        new_lines.insert(3, String::from("  #D#C#B#A#"));
        new_lines.insert(4, String::from("  #D#B#A#C#"));
        new_lines
    };
    parse(&effective_lines, &mut world).unwrap();
    if DEBUG {
        println!("{}", world_to_string(&world, depth));
    }

    match part {
        Part::Part1 => {
            let res = solve(world, depth).unwrap();
            println!("Result {}", res);
        }
        Part::Part2 => {
            let res = solve(world, depth).unwrap();
            println!("Result {}", res);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_world_part1() {
        let world = init_world(1);
        assert_eq!(world.hallway, 0);
        assert_eq!(world.rooms[0], 0b001 << 13 | 0b001 << 10);
        assert_eq!(world.rooms[1], 0b011 << 13 | 0b011 << 10);
        assert_eq!(world.rooms[2], 0b101 << 13 | 0b101 << 10);
        assert_eq!(world.rooms[3], 0b111 << 13 | 0b111 << 10);
    }

    #[test]
    fn test_init_world_part2() {
        let world = init_world(3);
        assert_eq!(world.hallway, 0);
        assert_eq!(world.rooms[0], 0);
        assert_eq!(world.rooms[1], 0);
        assert_eq!(world.rooms[2], 0);
        assert_eq!(world.rooms[3], 0);
    }

    #[test]
    fn test_init_world_part1_set_room() {
        let mut world = init_world(1);
        set_room_with_pod_for_init(&mut world, POD_A, POD_B, 1);
        set_room_with_pod_for_init(&mut world, POD_A, POD_A, 0);
        set_room_with_pod_for_init(&mut world, POD_B, POD_B, 1);
        set_room_with_pod_for_init(&mut world, POD_B, POD_B, 0);

        assert_eq!(world.hallway, 0);
        assert_eq!(
            world.rooms[0],
            0b001 << 13 | 0b001 << 10 | 0b011 << 7 | 0b001 << 4 | 0b0011
        );
        assert_eq!(
            world.rooms[1],
            0b011 << 13 | 0b011 << 10 | 0b011 << 7 | 0b011 << 4 | 0b0000
        );
    }

    #[test]
    fn test_init_world_part1_simple_moves() {
        let mut world = init_world(1);
        set_room_with_pod_for_init(&mut world, POD_A, POD_B, 1);
        set_room_with_pod_for_init(&mut world, POD_A, POD_A, 0);
        set_room_with_pod_for_init(&mut world, POD_B, POD_B, 1);
        set_room_with_pod_for_init(&mut world, POD_B, POD_A, 0);

        set_hallway_pod(&mut world, 0, POD_A);
        apply_move(
            &world,
            &Move::ToHallway {
                depth: 0,
                pod: POD_A,
                pos: 0,
                room: POD_A,
            },
        );
        assert_eq!(get_hallway_movable_cells(&world), vec![]);
        assert_eq!(is_room_available(&world, POD_A), false);
        assert_eq!(is_hallway_move_possible_included(&world, 0, 3), false);
    }

    #[test]
    fn test_init_world_part2_set_room() {
        let mut world = init_world(3);
        set_room_with_pod_for_init(&mut world, POD_A, POD_B, 3);
        set_room_with_pod_for_init(&mut world, POD_A, POD_A, 2);
        set_room_with_pod_for_init(&mut world, POD_A, POD_A, 1);
        set_room_with_pod_for_init(&mut world, POD_A, POD_A, 0);
        set_room_with_pod_for_init(&mut world, POD_B, POD_B, 3);
        set_room_with_pod_for_init(&mut world, POD_B, POD_B, 2);
        set_room_with_pod_for_init(&mut world, POD_B, POD_B, 1);
        set_room_with_pod_for_init(&mut world, POD_B, POD_B, 0);

        assert_eq!(world.hallway, 0);
        assert_eq!(
            world.rooms[0],
            0b011 << 13 | 0b001 << 10 | 0b001 << 7 | 0b001 << 4 | 0b1111
        );
        assert_eq!(
            world.rooms[1],
            0b011 << 13 | 0b011 << 10 | 0b011 << 7 | 0b011 << 4 | 0b0000
        );
    }

    #[test]
    fn test_calc_cost_info() {
        assert_eq!(calc_cost_info(0, 0, 0), 3);
        assert_eq!(calc_cost_info(1, 0, 0), 4);
    }

    #[test]
    fn test_calc_effective_cost_between_rooms() {
        assert_eq!(calc_effective_cost_info_between_room(0, POD_A, POD_A), 4);
        assert_eq!(calc_effective_cost_info_between_room(1, POD_A, POD_A), 5);
        assert_eq!(calc_effective_cost_info_between_room(0, POD_A, POD_B), 4);
        assert_eq!(calc_effective_cost_info_between_room(1, POD_A, POD_B), 5);
        assert_eq!(calc_effective_cost_info_between_room(1, POD_B, POD_A), 5);
    }
}
