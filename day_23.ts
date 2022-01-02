import { PriorityQueue } from "./priority_queue"
import { generator, Part, run, Type } from "./day_utils"
const testData = `#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########`

const parser = /^..#(\w|\.)#(\w|\.)#(\w|\.)#(\w|\.)#/;

const COSTS = {
    "A": 1,
    "B": 10,
    "C": 100,
    "D": 1000
}

const HALLWAY_POS_LINK = {
    "A": 2,
    "B": 4,
    "C": 6,
    "D": 8
}

const FORBIDDEN_POS_IN_HALLWAY_MAP: { [key: string]: boolean } = {
    2: true,
    4: true,
    6: true,
    8: true
};

const CACHE_PATHS: Map<string, CellRef[]> = new Map();
const DEBUG = false;

type AmphipodType = "A" | "B" | "C" | "D";
const AMPHIPOD_TYPES_LIST: AmphipodType[] = ["A", "B", "C", "D"];


function get_key2(world: World2): string {
    const hallway = world.hallway.map(val => val ?? ".").join("");
    const rooms = AMPHIPOD_TYPES_LIST.map(key => key + "#" + world.rooms[key].map(val => val ?? ".").join("")).join("|");

    return hallway + "|" + rooms;
}

function world_to_string(world: World2): string {
    const depth = world.rooms["A"].length;
    return ["#".repeat(13),
    "#" + world.hallway.map(val => val ?? ".").join("") + "#",
    ...[...generator(depth)].map(pos => "  #" + AMPHIPOD_TYPES_LIST.map(key => world.rooms[key][pos] ?? ".").join("#") + "#  "),
    "cost ===>" + world.cost
    ].join("\n");
}

function world_history(start_world: World2, moves: Move2[]): string {
    let curr_world = start_world;
    const history: string[] = []
    for (const move of moves) {
        history.push(world_to_string(curr_world));
        curr_world = apply_move2(curr_world, move);
    }
    history.push(world_to_string(curr_world));
    return history.join("\n\n");
}

function print_world_history(world: World2, moves: Move2[]) {
    console.log(world_history(world, moves));
}


function solve(init_world: World2): number | undefined {
    CACHE_PATHS.clear();
    const priorityQueue: PriorityQueue<World2> = new PriorityQueue((world) => world.estimated_total_cost,true);

    priorityQueue.put(init_world, get_key2(init_world));
    while (priorityQueue.isNotEmpty()) {
        const nextWorldQueued = priorityQueue.pop()
        const nextWorld = nextWorldQueued?.item;
        if (nextWorld === undefined) {
            break;
        }

        if (nextWorld.all_finished_refs.length === nextWorld.nb_amphipod) {
            if (DEBUG) {
                print_world_history(init_world, nextWorld.moves);
            }
            console.log(`Noeuds traités ${priorityQueue.explored()}`)
            return nextWorld.cost;
        }

        const possible_worlds = get_possible_target_world2(nextWorld);
        for (const possible_world of possible_worlds) {
            const key = get_key2(possible_world);
            priorityQueue.put(possible_world, key);
        }
    }
    return undefined;
}


function puzzle(lines: string[], part: Part): void {
    let depth = 2;
    if (part === Part.PART_2) {
        depth = 4;
        lines.splice(3, 0,
            "  #D#C#B#A#",
            "  #D#B#A#C#"
        )
    }
    const world = parse2(lines, depth);

    if (part === Part.PART_1) {
        const min_cost = solve(world);
        console.log(`Results ${min_cost}`);
    }
    else {
        const min_cost = solve(world);
        console.log(`Results ${min_cost}`);
    }
}

run(23, testData, [Type.TEST, Type.RUN], puzzle, [Part.PART_1, Part.PART_2])

type CellRef = {
    type: "hallway" | AmphipodType,
    pos: number
}

type World2 = {
    cost: number,
    hallway: (AmphipodType | undefined | null)[],
    rooms: { [key: string]: (AmphipodType | undefined | null)[] }
    all_refs: CellRef[],
    nb_amphipod: number,
    all_finished_refs: CellRef[],
    moves: Move2[],
    estimated_total_cost: number,
}

function get_cache_path_key(source: CellRef, target: CellRef): string {
    return source.type + ":" + source.pos + "=>" + target.type + ":" + target.pos
}

function get_path(source: CellRef, target: CellRef): CellRef[] {
    const cache_key = get_cache_path_key(source, target);
    const cache_result = CACHE_PATHS.get(cache_key);
    if (cache_result) {
        return cache_result;
    }
    let result: CellRef[] = [];
    if (source.type === target.type) {
        const direction = source.pos > target.pos ? -1 : 1;
        let pos = source.pos;
        while (pos !== target.pos) {
            pos += direction;
            result.push({
                pos,
                type: source.type
            })
        }
    }
    else if (source.type === "hallway" && target.type !== "hallway") {
        const target_hallway_pos = HALLWAY_POS_LINK[target.type];
        const hallway_path = get_path(source, { type: "hallway", pos: target_hallway_pos });
        const target_room_path = get_path({ type: target.type, pos: 0 }, target);
        result = [...hallway_path, { type: target.type, pos: 0 }, ...target_room_path];
    }
    else if (source.type !== "hallway") {
        const target_hallway_pos = HALLWAY_POS_LINK[source.type];
        const room_path = get_path(source, { type: source.type, pos: 0 });
        const other_part = get_path({ type: "hallway", pos: target_hallway_pos }, target);
        result = [...room_path, { type: "hallway", pos: target_hallway_pos }, ...other_part];
    }
    CACHE_PATHS.set(cache_key, result);
    return result;
}


function get_cell_value(world: World2, ref: CellRef): AmphipodType | undefined {
    let result = undefined
    if (ref.type === "hallway") {
        result = world.hallway[ref.pos];
    }
    else {
        result = world.rooms[ref.type][ref.pos];
    }
    if (result === undefined || result === null) {
        return undefined
    }
    return result;
}

function get_path_cost(type: AmphipodType, world: World2, path: CellRef[]): number | undefined {
    let cost = 0;
    for (const ref of path) {
        if (get_cell_value(world, ref) !== undefined) {
            return undefined;
        }
        cost += COSTS[type];
    }
    return cost;
}

type Move2 = {
    source: CellRef,
    target: CellRef,
    cost: number;
    amphipod_type: AmphipodType;
}

function is_room_available(world: World2, type: AmphipodType): boolean {
    return world.rooms[type].every(val => val === undefined || val === null || val === type)
}

function get_all_possible_move2(world: World2): Move2[] {
    const result: Move2[] = [];
    const source_refs = world.all_refs
        .filter(ref => world.all_finished_refs.find(finished_ref => finished_ref === ref) === undefined)
        .map(ref => [ref, get_cell_value(world, ref)])
        .filter(tuple => typeof tuple[1] === "string")
        .map(tuple => tuple as [CellRef, AmphipodType]);

    for (const source_ref of source_refs) {
        const source_cell_ref = source_ref[0];
        const amphibod_type = source_ref[1];
        for (const target_ref of world.all_refs) {
            //No reason to move inside same place
            const is_inside_or_between_room_move = source_cell_ref.type === target_ref.type;
            if (is_inside_or_between_room_move) {
                continue;
            }
            //source == target or target already used
            if (target_ref === source_cell_ref || get_cell_value(world, target_ref) !== undefined) {
                continue;
            }
            if (source_cell_ref.type === "hallway" || (target_ref.type !== "hallway" && source_cell_ref.type !== target_ref.type)) {
                if (target_ref.type !== amphibod_type) {
                    continue;
                }
                const is_available_room = is_room_available(world, amphibod_type);
                if (!is_available_room) {
                    continue;
                }
            }
            if (target_ref.type === "hallway" && FORBIDDEN_POS_IN_HALLWAY_MAP[target_ref.pos]) {
                continue;
            }
            const path = get_path(source_cell_ref, target_ref);
            const cost = get_path_cost(amphibod_type, world, path);
            if (cost !== undefined && path.length > 0) {
                result.push({
                    cost: cost,
                    target: target_ref,
                    source: source_cell_ref,
                    amphipod_type: amphibod_type
                })
            }
        }
    }
    return result;
}


function get_possible_target_world2(world: World2): World2[] {
    return get_all_possible_move2(world).map(move => apply_move2(world, move))
}

function set_ref_value(world: World2, ref: CellRef, val: AmphipodType | undefined) {
    if (ref.type === "hallway") {
        world.hallway[ref.pos] = val;
    }
    else {
        if (world.rooms[ref.type] === undefined) {
            world.rooms[ref.type] = [];
        }
        world.rooms[ref.type][ref.pos] = val;
    }
}

function apply_move2(world: World2, move: Move2): World2 {
    const newRooms: { [key: string]: (AmphipodType | undefined | null)[] } = {};
    for (const key in world.rooms) {
        newRooms[key] = [...world.rooms[key]];
    }
    const newWorld: World2 = {
        cost: world.cost + move.cost,
        all_refs: world.all_refs,
        hallway: [...world.hallway],
        rooms: newRooms,
        all_finished_refs: [...world.all_finished_refs],
        nb_amphipod: world.nb_amphipod,
        moves: [...world.moves, move],
        estimated_total_cost: world.estimated_total_cost + move.cost
    }
    set_ref_value(newWorld, move.source, undefined);
    set_ref_value(newWorld, move.target, move.amphipod_type);
    newWorld.estimated_total_cost -= calc_estimate_cost_to_end(world, move.source);
    newWorld.estimated_total_cost += calc_estimate_cost_to_end(newWorld, move.target);
    update_finished_ref_if_applicable(newWorld, move.target);
    return newWorld
}

function update_finished_ref_if_applicable(world: World2, ref: CellRef) {
    if (get_cell_value(world, ref) === ref.type) {
        const is_room_full = world.rooms[ref.type].slice(ref.pos).every(v => v === ref.type)
        if (is_room_full) {
            world.all_finished_refs.push(ref);
        }
    }
}

function calc_estimate_cost_to_end(world: World2, ref: CellRef): number {
    return 0;
    /*const type = get_cell_value(world, ref);
    if (type === undefined) {
        return 0;
    }
    if (world.all_finished_refs.find(fref => fref === ref) !== undefined) {
        return 0;
    }
    let path_size = 0;
    if (ref.type == type) {
        const is_valid_room = world.rooms[ref.type].every(v => v === undefined || v === null || v === type);
        if (is_valid_room) {
            return 1;
        }
        const intermediate_ref: CellRef = { type: "hallway", pos: HALLWAY_POS_LINK[type] + 1 };
        path_size += get_path(ref, intermediate_ref).length +
            get_path(intermediate_ref, { type: type, pos: 0 }).length
            ;
    }
    else {
        path_size += get_path(ref, { type: type, pos: 0 }).length
    }

    return path_size * COSTS[type];*/
}

function parse2(lines: string[], depth: number): World2 {
    const all_refs: CellRef[] = [...generator(11)].map(pos => <CellRef>{ type: "hallway", pos });
    (["A", "B", "C", "D"] as AmphipodType[]).forEach(type => {
        [...generator(depth)].forEach(pos => all_refs.push({ type: type, pos }))
    })
    const world: World2 = {
        hallway: [],
        rooms: {},
        all_refs,
        cost: 0,
        all_finished_refs: [],
        nb_amphipod: 0,
        moves: [],
        estimated_total_cost: 0
    }
    all_refs.forEach(ref => set_ref_value(world, ref, undefined));
    const amphipod_to_place_in_rooms: (CellRef & { value: AmphipodType })[] = lines.slice(2, 2 + depth).flatMap((line, index) => {
        const matcher = line.match(parser);
        return ([["A", 1], ["B", 2], ["C", 3], ["D", 4]] as [string, number][]).flatMap(entry => {
            if (!matcher || matcher[entry[1]] === undefined || matcher[entry[1]] === ".") {
                return []
            }
            return [{
                type: entry[0] as AmphipodType,
                pos: index,
                value: matcher[entry[1]] as AmphipodType
            }]
        })
    });
    amphipod_to_place_in_rooms.forEach(placement => set_ref_value(world, placement, placement.value));
    lines[1].split("").slice(1, -1).forEach((v, index) => {
        if (v !== "." && v !== "#") {
            set_ref_value(world, { type: "hallway", pos: index }, v as AmphipodType)
        }
    });
    all_refs.forEach(ref => update_finished_ref_if_applicable(world, ref));
    world.nb_amphipod = all_refs.filter(ref => get_cell_value(world, ref) !== undefined).length;
    world.estimated_total_cost = all_refs.reduce((sum, ref) => sum + calc_estimate_cost_to_end(world, ref), 0)
    return world;
}
