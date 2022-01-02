use crate::utils::Part;
use regex::{Captures, Regex};
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Write;

/*****
 *          ENCODING WORLD in 64 bits (being a key + world definition)
 *   General encoding :
 *     encoding A (00), B(01), C(10) D(11)
 *
 *   Hallway :
 *     Only 7 hallway room usable 3 bits needed : 000 empty xx1 full with xx being A, B , C or D
 *     using 00 as empty allows to quickly check if path "ok" with condition (mask & hallway bit) === 0
 *
 *   Room : naive storage 3bits per cell (4 values + Empty case)
 *      Optimized encoding using 2 bits of metadata + 8 bits of data
 *      Metdata - The first bit tell full or not second bit tell if usable or not
 *          00 FULL OK  : finished state (all rooms filled with right amphipod)
 *          01 FULL BAD : filled with some bad data (all rooms filled with some bad amphipod)
 *          10 USABLE   : some place available for right amphipod (nothing to move from)
 *          11 BAD      : cannot accept new pods, may be empty
 *       Room : 4 cells . When not full one to tell the offset first empty (left one)
 *          Cell Offset :
 *              11 ==> fully empty
 *              10 ==> 1 used
 *              01 ==> 2 used
 *              00 ==> 3 used (use current to fill)
 *
 *
 *   Global layout:
 *   [########## 32 bits ###########][########## 32 bits ###########]
 *   [3][      21 bits      ][8bits ][           32 bits            ]
 *   dd_HA.H9.H7.H5.H3.H1.H0.|META R||ROOM D||ROOM C||ROOM B||ROOM A|
 *                           DDCCBBAA
 *   dd==> max depth (11 ou 01)
 *     Note : the world is "finished if the first 32 bits are equal to 0"
 *
*/

type World = u64;

const ROOM_DEPTH_OFFSET: u8 = 62;
//Room metadata
const ROOM_DATA_NB_BITS: u8 = 8;

const ROOM_METADATA_NB_BITS: u8 = 2;
// left bit  :0 (full), 1 (cell available),
// right bit :0 (valid room), 1 (bad pod existing)
const ROOM_METADATA_AVAILABLE: u8 = 0b10;
const ROOM_METADATA_NOT_FULL_BIT_MASK: u8 = 0b10;
const ROOM_METADATA_NEED_MOVE_FROM_BIT_MASK: u8 = 0b01;
const ROOM_METADATA_BAD_POD_MASK: u8 = 0b01;

const HEIGHT_BITS_MASK: u64 = 0xFF;
const ROOMS_OFFSET: u8 = 32;
const HALLWAY_OFFSET: u8 = ROOMS_OFFSET + 8;
const HALLWAY_MASK: u64 = ((1 << 21) - 1) << HALLWAY_OFFSET;
#[inline]
fn get_rooms_metadata(world: World) -> u8 {
    ((world >> ROOMS_OFFSET) & HEIGHT_BITS_MASK) as u8
}

#[inline]
fn get_room_metadata(world: World, room: u8) -> u8 {
    (get_rooms_metadata(world) >> (room * ROOM_METADATA_NB_BITS)) & 0b11
}

#[inline]
fn get_room_available_for_set_with_depth(world: World, room: u8) -> Option<u8> {
    let metadata = get_room_metadata(world, room);
    if metadata != ROOM_METADATA_AVAILABLE {
        return None;
    }
    if metadata & ROOM_METADATA_NOT_FULL_BIT_MASK == 0 {
        return Some(0);
    }
    return Some(get_room_data(world, room) & 0b11);
}

#[inline]
fn get_room_data(world: World, room: u8) -> u8 {
    ((world >> (room * ROOM_DATA_NB_BITS)) & HEIGHT_BITS_MASK) as u8
}

fn get_room_pod_from_depth(world: World, room: u8, depth: u8) -> Option<u8> {
    let metadata = get_room_metadata(world, room);
    let data = get_room_data(world, room);
    let is_full = (metadata & ROOM_METADATA_NOT_FULL_BIT_MASK) == 0;
    let is_empty = !is_full && (data == ((world >> ROOM_DEPTH_OFFSET) as u8));
    let curr_depth = if is_full { 0 } else { (data & 0b11) + 1 };
    return if is_empty || curr_depth > depth {
        None
    } else {
        Some((data >> (depth * 2)) & 0b11)
    };
}

fn get_room_movable_pod_and_depth(world: World, room: u8) -> Option<(u8, u8)> {
    let metadata = get_room_metadata(world, room);
    if (metadata & ROOM_METADATA_NEED_MOVE_FROM_BIT_MASK) == 0 {
        None
    } else {
        let data = get_room_data(world, room);
        let is_full = (metadata & ROOM_METADATA_NOT_FULL_BIT_MASK) == 0;
        let is_empty = data == ((world >> ROOM_DEPTH_OFFSET) as u8);
        let first = data & 0b11;
        return if is_full {
            Some((first, 0))
        } else if is_empty {
            None
        } else {
            Some((data >> ((first + 1) * 2) & 0b11, first + 1))
        };
    }
}

fn set_room_with_pod(world: World, room: u8, pod: u8) -> World {
    //hypothesis ==> some place left (at least one)
    let metadata = get_room_metadata(world, room);
    let current_data = get_room_data(world, room);
    let offset = current_data & 0b11;
    let new_data_mask = HEIGHT_BITS_MASK << (room * ROOM_DATA_NB_BITS);
    let bad_pod_bit = if pod == room {
        metadata & ROOM_METADATA_BAD_POD_MASK
    } else {
        ROOM_METADATA_BAD_POD_MASK
    };
    let bad_pod_bit_offsetted =
        (bad_pod_bit as u64) << (room * ROOM_METADATA_NB_BITS) << ROOMS_OFFSET;
    if offset == 0 {
        let new_data = (current_data & (!0b11)) | pod;
        let reset_bit_not_full_mask = ((ROOM_METADATA_NOT_FULL_BIT_MASK as u64)
            << (room * ROOM_METADATA_NB_BITS))
            << ROOMS_OFFSET;
        let cleaned_world = world & !(new_data_mask | reset_bit_not_full_mask);
        let new_data_offsetted = (new_data as u64) << (room * ROOM_DATA_NB_BITS);
        return cleaned_world | new_data_offsetted | bad_pod_bit_offsetted;
    } else {
        let set_mask = (0b11 << (offset * 2)) | 0b11;
        let new_data = (current_data & (!set_mask)) | pod << (offset * 2) | (offset - 1);
        let cleaned_world = world & !new_data_mask;
        let new_data_offsetted = (new_data as u64) << (room * ROOM_DATA_NB_BITS);
        return cleaned_world | new_data_offsetted | bad_pod_bit_offsetted;
    }
}

fn pop_room_pod(world: World, room: u8) -> World {
    let depth = (world >> ROOM_DEPTH_OFFSET) as u8;
    let metadata = get_room_metadata(world, room);

    //hypothesis ==> some place left (at least one)
    let current_data = get_room_data(world, room);
    let is_not_full = (metadata & ROOM_METADATA_NOT_FULL_BIT_MASK) != 0;
    let new_offset = if is_not_full {
        std::cmp::min((current_data & 0b11) + 1, depth)
    } else {
        0
    };
    let new_data = current_data & (0xFF << (new_offset + 1) * 2) | new_offset;
    let new_data_offsetted = (new_data as u64) << (room * ROOM_DATA_NB_BITS);
    let not_full_bit_offsetted =
        (ROOM_METADATA_NOT_FULL_BIT_MASK as u64) << (room * ROOM_METADATA_NB_BITS) << ROOMS_OFFSET;
    let new_data_mask = HEIGHT_BITS_MASK << (room * ROOM_DATA_NB_BITS);
    if metadata & ROOM_METADATA_BAD_POD_MASK == 0 {
        let cleaned_world = world & !new_data_mask;
        return cleaned_world | new_data_offsetted | not_full_bit_offsetted;
    } else {
        for offset in new_offset + 1..=depth {
            let pod_value = (new_data >> offset * 2) & 0b11;
            if pod_value != room {
                let cleaned_world = world & !(new_data_mask);
                return cleaned_world | new_data_offsetted | not_full_bit_offsetted;
            }
        }
        let bad_pod_bit_clear =
            ((ROOM_METADATA_BAD_POD_MASK as u64) << (room * ROOM_METADATA_NB_BITS)) << ROOMS_OFFSET;

        let cleaned_world = world & !(new_data_mask | bad_pod_bit_clear);
        return cleaned_world | new_data_offsetted | not_full_bit_offsetted;
    }
}

fn set_hallway_cell(world: World, pos: u8, pod: u8) -> World {
    let new_val = (pod << 1) | 1;
    let mask = (0b111 << (pos * 3)) << HALLWAY_OFFSET;
    let new_val_offsetted = (new_val as u64) << (pos * 3) << HALLWAY_OFFSET;

    return (world & !mask) | new_val_offsetted;
}

fn clear_hallway_cell(world: World, pos: u8) -> World {
    let mask = (0b111 << (pos * 3)) << HALLWAY_OFFSET;
    return world & !mask;
}

fn get_hallway_used_cells(world: World) -> Vec<(u8, u8)> {
    let mut vec = Vec::<(u8, u8)>::with_capacity(7);
    let curr_hallway = world >> HALLWAY_OFFSET;
    for pos in H_POS_0..=H_POS_10 {
        let curr = ((curr_hallway >> (pos * 3)) & 0b111) as u8;
        if curr != 0 {
            vec.push((curr >> 1, pos));
        }
    }
    vec
}
const H_POS_0: u8 = 0;
const H_POS_1: u8 = 1;
const H_POS_3: u8 = 2;
const H_POS_5: u8 = 3;
const H_POS_7: u8 = 4;
const H_POS_9: u8 = 5;
const H_POS_10: u8 = 6;

const POD_A: u8 = 0b00;
const POD_B: u8 = 0b01;
const POD_C: u8 = 0b10;
const POD_D: u8 = 0b11;

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

fn parse(lines: &Vec<String>, empty_world: u64) -> Result<World, ParsingError> {
    let mut world: World = empty_world;

    let pods = parse_pods(lines)?;

    pods.iter().rev().for_each(|(a, b, c, d)| {
        world = set_room_with_pod(world, POD_A, *a);
        world = set_room_with_pod(world, POD_B, *b);
        world = set_room_with_pod(world, POD_C, *c);
        world = set_room_with_pod(world, POD_D, *d);
    });

    return Ok(world);
}

fn init_world(part: &Part) -> World {
    let depth = match part {
        Part::Part1 => 0b01,
        _ => 0b11,
    };
    let init_metadata = ROOM_METADATA_AVAILABLE as World;
    let init_data = depth;
    let mut metadata: World = 0;
    let mut world: World = 0;
    for pod in POD_A..=POD_D {
        metadata |= init_metadata << (pod * ROOM_METADATA_NB_BITS);
        world |= init_data << (pod * ROOM_DATA_NB_BITS);
    }
    world | (metadata << ROOMS_OFFSET) | (depth << ROOM_DEPTH_OFFSET)
}

#[derive(Eq, PartialEq)]
struct WorldToEvaluate {
    estimated_cost: i32,
    world: World,
    move_data: u8,
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

fn build_move(is_to_hallway: bool, room: u8, pos: u8, pod: u8) -> u8 {
    return pos << 1 | room << (3 + 1) | pod << (5 + 1) | if is_to_hallway { 0b1 } else { 0b0 };
}

fn apply_move(world: World, move_data: u8) -> World {
    let is_to_hallway = (move_data & 0b1) == 1;
    let h_pos = (move_data >> 1) & 0b111;
    let room = (move_data >> (3 + 1)) & 0b11;
    let pod = (move_data >> (5 + 1)) & 0b11;
    if is_to_hallway {
        return set_hallway_cell(pop_room_pod(world, room), h_pos, pod);
    } else {
        return set_room_with_pod(clear_hallway_cell(world, h_pos), room, pod);
    }
}

fn calc_effective_cost_info_from_hallway(pos: u8, room: u8) -> i32 {
    let (_, _, distance) = calc_room_cell_to_hallway_pos_info(room, 0, pos);
    return distance;
}

fn calc_room_cell_to_hallway_pos_info(room: u8, depth: u8, hpos: u8) -> (u8, u8, i32) {
    let pos_room_to_right = 2 + room; // place itself just on the right of room pos
    let (min_hpos, max_hpos, distance) = if pos_room_to_right <= hpos {
        let effective_start_pos = pos_room_to_right;
        let effective_implicit_cells = std::cmp::min(hpos, H_POS_9) - effective_start_pos;

        (
            effective_start_pos,
            hpos,
            effective_implicit_cells + (hpos - effective_start_pos) + 1,
        )
    } else {
        let effective_start_pos = pos_room_to_right - 1;
        let effective_implicit_cells = effective_start_pos - std::cmp::max(hpos, H_POS_1);

        (
            hpos,
            effective_start_pos,
            effective_implicit_cells + (effective_start_pos - hpos) + 1,
        )
    };
    let new_unitary_cost = (depth + 1 + distance) as i32;
    return (min_hpos, max_hpos, new_unitary_cost);
}
fn build_world_to_evaluate(
    world_ctxt: &WorldToEvaluate,
    is_to_hallway: bool,
    pod: u8,
    room: u8,
    depth: u8,
    pos: u8,
) -> Option<WorldToEvaluate> {
    let (min_hpos, max_hpos, new_unitary_cost) =
        calc_room_cell_to_hallway_pos_info(room, depth, pos);
    let effective_min_hpos = if !is_to_hallway && min_hpos == pos {
        pos + 1
    } else {
        min_hpos
    };
    let effective_max_hpos = if !is_to_hallway && max_hpos == pos {
        pos - 1
    } else {
        max_hpos
    };
    let mask = (HALLWAY_MASK >> ((H_POS_10 - effective_max_hpos) * 3))
        & (HALLWAY_MASK << (effective_min_hpos * 3));
    if world_ctxt.world & mask != 0 {
        return None;
    }
    let new_unitary_cost = new_unitary_cost;
    let new_estimated_cost = new_unitary_cost as i32
        + if is_to_hallway {
            calc_effective_cost_info_from_hallway(pos, pod)
                - calc_effective_cost_info_between_room(depth, room, pod)
        } else {
            -calc_effective_cost_info_from_hallway(pos, pod)
        };
    let cost_pod = i32::pow(10, pod as u32);
    let move_data = build_move(is_to_hallway, room, pos, pod);
    return Some(WorldToEvaluate {
        world: apply_move(world_ctxt.world, move_data),
        estimated_cost: world_ctxt.estimated_cost + new_estimated_cost * cost_pod,
        move_data,
    });
}

fn calc_next_worlds(world_ctxt: &WorldToEvaluate) -> Vec<WorldToEvaluate> {
    let world = world_ctxt.world;
    let mut result = Vec::<WorldToEvaluate>::with_capacity(7);
    for (pod, pos) in get_hallway_used_cells(world) {
        if let Some(depth) = get_room_available_for_set_with_depth(world, pod) {
            if let Some(world_built) =
                build_world_to_evaluate(&world_ctxt, false, pod, pod, depth, pos)
            {
                result.push(world_built);
            }
        }
    }

    for room in POD_A..=POD_D {
        if let Some((pod, depth)) = get_room_movable_pod_and_depth(world, room) {
            for pos in H_POS_0..=H_POS_10 {
                if let Some(world_built) =
                    build_world_to_evaluate(&world_ctxt, true, pod, room, depth, pos)
                {
                    result.push(world_built);
                }
            }
        }
    }
    result
}

fn pod_to_char(pod: u8) -> char {
    match pod {
        POD_A => 'A',
        POD_B => 'B',
        POD_C => 'C',
        POD_D => 'D',
        _ => '.',
    }
}

fn hallway_to_str(world: World) -> String {
    let hallway = world >> HALLWAY_OFFSET;
    let hallway_pos_to_str = |h: u64, pos: u8| -> char {
        let curr = (h >> (pos * 3) & 0b111) as u8;
        if (curr & 0b1) == 0 {
            return '.';
        }
        pod_to_char(curr >> 1)
    };
    let mut h_str = String::new();
    write!(
        &mut h_str,
        "#{}{}.{}.{}.{}.{}{}#",
        hallway_pos_to_str(hallway, H_POS_0),
        hallway_pos_to_str(hallway, H_POS_1),
        hallway_pos_to_str(hallway, H_POS_3),
        hallway_pos_to_str(hallway, H_POS_5),
        hallway_pos_to_str(hallway, H_POS_7),
        hallway_pos_to_str(hallway, H_POS_9),
        hallway_pos_to_str(hallway, H_POS_10)
    )
    .unwrap();
    return h_str;
}

fn room_depth_to_str(world: World, depth: u8) -> String {
    let room_and_depth_to_str = |w: u64, r: u8, d: u8| -> char {
        match get_room_pod_from_depth(w, r, d) {
            Some(p) => pod_to_char(p),
            None => '.',
        }
    };
    let mut r = String::new();
    write!(
        &mut r,
        "  #{}#{}#{}#{}#  ",
        room_and_depth_to_str(world, POD_A, depth),
        room_and_depth_to_str(world, POD_B, depth),
        room_and_depth_to_str(world, POD_C, depth),
        room_and_depth_to_str(world, POD_D, depth),
    )
    .unwrap();
    return r;
}

fn world_to_string(world: World) -> String {
    let mut result: Vec<String> = vec![String::from("#############"), hallway_to_str(world)];
    let depth = (world >> ROOM_DEPTH_OFFSET) as u8;
    for d in 0..=depth {
        result.push(room_depth_to_str(world, d))
    }
    return result.join("\n");
}

fn is_world_finished(world: World) -> bool {
    world & (0x3FFFFFFF << ROOMS_OFFSET) == 0
}

fn print_world(world: World) {
    println!("{}", world_to_string(world));
}

fn rebuild_history(
    last_world_data: &WorldToEvaluate,
    processed: &HashMap<World, (u8, i32)>,
) -> Vec<World> {
    let mut history = vec![last_world_data.world];
    let mut last_move = last_world_data.move_data;
    let mut last_world = last_world_data.world;
    let mut last_cost = last_world_data.estimated_cost;

    while last_move != 0xFF {
        println!("Cout {}", last_cost);
        let invert_move = (last_move & !0b1) | if last_move & 0b1 == 0b1 { 0b0 } else { 0b1 };
        last_world = apply_move(last_world, invert_move);
        print_world(last_world);
        history.push(last_world);
        let (move_data, cost) = processed.get(&last_world).unwrap();
        last_move = *move_data;
        last_cost = *cost;
    }

    return history;
}

fn abs_diff(a: u8, b: u8) -> u8 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn calc_room_to_hallway_pos(room: u8) -> u8 {
    (room + 1) * 2
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

fn calc_init_estimated_cost(world: World) -> i32 {
    let mut sum: i32 = 0;
    let max_depth = (world >> ROOM_DEPTH_OFFSET) as u8;

    for room in POD_A..=POD_D {
        let mut is_valid_for_room = true;
        for depth in (0..=max_depth).rev() {
            let pod_opt = get_room_pod_from_depth(world, room, depth);
            if let Some(pod) = pod_opt {
                if !is_valid_for_room || pod != room {
                    is_valid_for_room = false;
                    sum += calc_effective_cost_info_between_room(depth, room, pod)
                        * i32::pow(10, pod as u32);
                }
            }
        }
    }
    return sum;
}
const DEBUG: bool = false;
fn solve(start_world: World) -> Option<i32> {
    let mut processed: HashMap<World, (u8, i32)> = HashMap::new();
    let mut priority_queue: BinaryHeap<WorldToEvaluate> = BinaryHeap::new();
    priority_queue.push(WorldToEvaluate {
        estimated_cost: calc_init_estimated_cost(start_world),
        world: start_world,
        move_data: 0xff,
    });

    while let Some(next) = priority_queue.pop() {
        if DEBUG {
            println!("Processing \n{}", world_to_string(next.world));
        }
        if is_world_finished(next.world) {
            println!(
                "Noeuds traités {} avec cout estimé {}",
                processed.len(),
                next.estimated_cost
            );
            if DEBUG {
                rebuild_history(&next, &processed);
            }

            return Some(next.estimated_cost);
        }
        for next_world in calc_next_worlds(&next) {
            if !processed.contains_key(&next_world.world) {
                if DEBUG {
                    println!("Queuing \n{}", world_to_string(next_world.world));
                }
                priority_queue.push(next_world);
            }
        }
        processed.insert(next.world, (next.move_data, next.estimated_cost));
    }
    return None;
}

pub fn puzzle(part: &Part, lines: &Vec<String>) {
    let effective_lines = if let Part::Part1 = part {
        lines.clone()
    } else {
        let mut new_lines = lines.clone();
        new_lines.insert(3, String::from("  #D#C#B#A#"));
        new_lines.insert(4, String::from("  #D#B#A#C#"));
        new_lines
    };
    let start_world = parse(&effective_lines, init_world(part)).unwrap();
    if DEBUG {
        println!("{}", world_to_string(start_world));
    }

    match part {
        Part::Part1 => {
            let res = solve(start_world).unwrap();
            println!("Result {}", res);
        }
        Part::Part2 => {
            let res = solve(start_world).unwrap();
            println!("Result {}", res);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_world_part1() {
        let world = init_world(&Part::Part1);
        assert_eq!(get_room_available_for_set_with_depth(world, POD_A), Some(1));
        assert_eq!(get_room_available_for_set_with_depth(world, POD_B), Some(1));
        assert_eq!(get_room_available_for_set_with_depth(world, POD_C), Some(1));
        assert_eq!(get_room_available_for_set_with_depth(world, POD_D), Some(1));
        assert_eq!(get_room_movable_pod_and_depth(world, POD_A), None);
        assert_eq!(get_room_pod_from_depth(world, POD_A, 0), None);
        assert_eq!(get_room_pod_from_depth(world, POD_A, 1), None);
        assert_eq!(get_room_movable_pod_and_depth(world, POD_B), None);
        assert_eq!(get_room_pod_from_depth(world, POD_B, 0), None);
        assert_eq!(get_room_pod_from_depth(world, POD_B, 1), None);
        assert_eq!(get_room_movable_pod_and_depth(world, POD_C), None);
        assert_eq!(get_room_pod_from_depth(world, POD_C, 0), None);
        assert_eq!(get_room_pod_from_depth(world, POD_C, 1), None);

        assert_eq!(get_room_movable_pod_and_depth(world, POD_D), None);
        assert_eq!(get_room_pod_from_depth(world, POD_D, 0), None);
        assert_eq!(get_room_pod_from_depth(world, POD_D, 1), None);

        assert_eq!(world >> ROOM_DEPTH_OFFSET, 0b01);
    }

    #[test]
    fn test_calc_set_pod() {
        let mut world = init_world(&Part::Part1);
        world = set_room_with_pod(world, POD_A, POD_A);
        assert_eq!(world, (world & !0b1111));
        assert_eq!(get_room_movable_pod_and_depth(world, POD_A), None);
        assert_eq!(get_room_available_for_set_with_depth(world, POD_A), Some(0));
        assert_eq!(get_room_pod_from_depth(world, POD_A, 0), None);
        assert_eq!(get_room_pod_from_depth(world, POD_A, 1), Some(POD_A));

        world = set_room_with_pod(world, POD_A, POD_A);
        assert_eq!(get_room_movable_pod_and_depth(world, POD_A), None);
        assert_eq!(get_room_available_for_set_with_depth(world, POD_A), None);
        assert_eq!(get_room_pod_from_depth(world, POD_A, 0), Some(POD_A));
        assert_eq!(get_room_pod_from_depth(world, POD_A, 1), Some(POD_A));

        assert_eq!(world >> ROOMS_OFFSET & 0b11, 0);
    }

    #[test]
    fn test_calc_set_bad_pod() {
        let mut world = init_world(&Part::Part1);
        world = set_room_with_pod(world, POD_B, POD_B);
        assert_eq!(get_room_movable_pod_and_depth(world, POD_B), None);
        assert_eq!(get_room_available_for_set_with_depth(world, POD_B), Some(0));
        assert_eq!(get_room_pod_from_depth(world, POD_B, 0), None);
        assert_eq!(get_room_pod_from_depth(world, POD_B, 1), Some(POD_B));

        world = set_room_with_pod(world, POD_B, POD_C);
        assert_eq!(
            get_room_movable_pod_and_depth(world, POD_B),
            Some((POD_C, 0))
        );
        assert_eq!(get_room_pod_from_depth(world, POD_B, 0), Some(POD_C));
        assert_eq!(get_room_pod_from_depth(world, POD_B, 1), Some(POD_B));
        assert_eq!(get_room_available_for_set_with_depth(world, POD_B), None);
    }

    #[test]
    fn test_calc_set_bad_pod2() {
        let mut world = init_world(&Part::Part1);
        world = set_room_with_pod(world, POD_C, POD_A);
        assert_eq!(
            get_room_movable_pod_and_depth(world, POD_C),
            Some((POD_A, 1))
        );
        assert_eq!(get_room_available_for_set_with_depth(world, POD_C), None);
        assert_eq!(get_room_pod_from_depth(world, POD_C, 0), None);
        assert_eq!(get_room_pod_from_depth(world, POD_C, 1), Some(POD_A));

        world = set_room_with_pod(world, POD_C, POD_C);
        assert_eq!(
            get_room_movable_pod_and_depth(world, POD_C),
            Some((POD_C, 0))
        );
        assert_eq!(get_room_available_for_set_with_depth(world, POD_C), None);
        assert_eq!(get_room_pod_from_depth(world, POD_C, 0), Some(POD_C));
        assert_eq!(get_room_pod_from_depth(world, POD_C, 1), Some(POD_A));

        world = pop_room_pod(world, POD_C);
        assert_eq!(
            get_room_movable_pod_and_depth(world, POD_C),
            Some((POD_A, 1))
        );
        assert_eq!(get_room_available_for_set_with_depth(world, POD_C), None);
        world = pop_room_pod(world, POD_C);
        assert_eq!(get_room_movable_pod_and_depth(world, POD_C), None);
        assert_eq!(get_room_available_for_set_with_depth(world, POD_C), Some(1));
        assert_eq!(get_room_pod_from_depth(world, POD_C, 0), None);
        assert_eq!(get_room_pod_from_depth(world, POD_C, 1), None);
    }

    #[test]
    fn test_calc_data_distance_calc() {
        assert_eq!(
            calc_room_cell_to_hallway_pos_info(POD_A, 0, H_POS_0),
            (H_POS_0, H_POS_1, 3)
        );
        assert_eq!(
            calc_room_cell_to_hallway_pos_info(POD_A, 0, H_POS_1),
            (H_POS_1, H_POS_1, 2)
        );
        assert_eq!(
            calc_room_cell_to_hallway_pos_info(POD_A, 0, H_POS_3),
            (H_POS_3, H_POS_3, 2)
        );
        assert_eq!(
            calc_room_cell_to_hallway_pos_info(POD_A, 0, H_POS_5),
            (H_POS_3, H_POS_5, 4)
        );
        assert_eq!(
            calc_room_cell_to_hallway_pos_info(POD_A, 0, H_POS_7),
            (H_POS_3, H_POS_7, 6)
        );
        assert_eq!(
            calc_room_cell_to_hallway_pos_info(POD_A, 0, H_POS_9),
            (H_POS_3, H_POS_9, 8)
        );
        assert_eq!(
            calc_room_cell_to_hallway_pos_info(POD_A, 0, H_POS_10),
            (H_POS_3, H_POS_10, 9)
        );

        assert_eq!(
            calc_room_cell_to_hallway_pos_info(POD_A, 1, H_POS_0),
            (H_POS_0, H_POS_1, 4)
        );

        assert_eq!(
            calc_room_cell_to_hallway_pos_info(POD_B, 0, H_POS_0),
            (H_POS_0, H_POS_3, 5)
        );
        assert_eq!(
            calc_room_cell_to_hallway_pos_info(POD_B, 0, H_POS_3),
            (H_POS_3, H_POS_3, 2)
        );
        assert_eq!(
            calc_room_cell_to_hallway_pos_info(POD_B, 0, H_POS_5),
            (H_POS_5, H_POS_5, 2)
        );
        assert_eq!(
            calc_room_cell_to_hallway_pos_info(POD_B, 0, H_POS_10),
            (H_POS_5, H_POS_10, 7)
        );

        assert_eq!(
            calc_room_cell_to_hallway_pos_info(POD_C, 0, H_POS_0),
            (H_POS_0, H_POS_5, 7)
        );
        assert_eq!(
            calc_room_cell_to_hallway_pos_info(POD_C, 0, H_POS_5),
            (H_POS_5, H_POS_5, 2)
        );
        assert_eq!(
            calc_room_cell_to_hallway_pos_info(POD_C, 0, H_POS_7),
            (H_POS_7, H_POS_7, 2)
        );
        assert_eq!(
            calc_room_cell_to_hallway_pos_info(POD_C, 0, H_POS_10),
            (H_POS_7, H_POS_10, 5)
        );

        assert_eq!(
            calc_room_cell_to_hallway_pos_info(POD_D, 0, H_POS_0),
            (H_POS_0, H_POS_7, 9)
        );
        assert_eq!(
            calc_room_cell_to_hallway_pos_info(POD_D, 0, H_POS_7),
            (H_POS_7, H_POS_7, 2)
        );
        assert_eq!(
            calc_room_cell_to_hallway_pos_info(POD_D, 0, H_POS_9),
            (H_POS_9, H_POS_9, 2)
        );
        assert_eq!(
            calc_room_cell_to_hallway_pos_info(POD_D, 0, H_POS_10),
            (H_POS_9, H_POS_10, 3)
        );
    }

    #[test]
    fn test_world_finished() {
        let mut world = init_world(&Part::Part1);
        world = set_room_with_pod(world, POD_A, POD_A);
        world = set_room_with_pod(world, POD_A, POD_A);
        world = set_room_with_pod(world, POD_B, POD_B);
        world = set_room_with_pod(world, POD_B, POD_B);
        world = set_room_with_pod(world, POD_C, POD_C);
        world = set_room_with_pod(world, POD_C, POD_C);
        world = set_room_with_pod(world, POD_D, POD_D);
        world = set_room_with_pod(world, POD_D, POD_D);

        assert_eq!(is_world_finished(world), true);
    }
}
