import { PriorityQueue } from "./priority_queue"
import { generator, Part, run, Type } from "./day_utils"
const testData = `1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581`

type CaveMap = {
    map: number[][],
    maxY: number,
    maxX: number
}
type Pos = {
    x: number,
    y: number
}
type WeightedPos = {
    cost_from_origin: number
    estimated_total_cost: number,
    comparision_cost: number,
    pos: Pos,
    from: WeightedPos | undefined
}

function parse(lines: string[]): number[][] {
    return lines.map(line => line.split("").map(num => parseInt(num)));
}

function generate_siblings(curr_pos: Pos, caveMap: CaveMap): Pos[] {
    return [
        { x: curr_pos.x + 1, y: curr_pos.y },
        { x: curr_pos.x - 1, y: curr_pos.y },
        { x: curr_pos.x, y: curr_pos.y + 1 },
        { x: curr_pos.x, y: curr_pos.y - 1 },
    ]
        .filter(pos => pos.x >= 0 && pos.x <= caveMap.maxX && pos.y >= 0 && pos.y <= caveMap.maxY)

}

function node_to_explore(curr_pos: WeightedPos, caveMap: CaveMap): WeightedPos[] {
    return generate_siblings(curr_pos.pos, caveMap)
        .map(pos => build_weighted_pos(pos, caveMap, curr_pos))
}

function build_weighted_pos(pos: Pos, caveMap: CaveMap, origin: WeightedPos | undefined = undefined): WeightedPos {
    const current_cost_from_orig = (origin?.cost_from_origin ?? -caveMap.map[pos.y][pos.x]) + caveMap.map[pos.y][pos.x];
    const estimated = current_cost_from_orig + (caveMap.maxX - pos.x) + (caveMap.maxY - pos.y);
    return {
        cost_from_origin: current_cost_from_orig,
        estimated_total_cost: estimated,
        comparision_cost: estimated,
        pos,
        from: origin
    }
}


function lookup_min_path(caveMap: CaveMap): [number, Pos[]] {
    const listToExplore = new PriorityQueue<WeightedPos>((w) => w.estimated_total_cost, true);
    listToExplore.put(build_weighted_pos({ x: 0, y: 0 }, caveMap), "0|0");
    while (listToExplore.isNotEmpty()) {
        const nextNodeRaw = listToExplore.pop();
        const nextNode = nextNodeRaw?.item
        if (nextNode === undefined) {
            break;
        }
        if (nextNode.pos.x === caveMap.maxX && nextNode.pos.y === caveMap.maxY) {
            console.log(`Node explored ${listToExplore.explored()}`)
            const path = []
            let currNode: WeightedPos | undefined = nextNode
            do {
                path.push(currNode.pos)
                currNode = currNode.from;
            } while (currNode !== undefined)
            return [nextNode.cost_from_origin, path.reverse()]
        }
        const siblings = node_to_explore(nextNode, caveMap);
        for (const sibling of siblings) {
            listToExplore.put(sibling, `${sibling.pos.x}|${sibling.pos.y}`)
        }
    }
    return [-1, []];
}

function preprocess_map(map: number[][], part: Part): number[][] {
    if (part === Part.PART_1) {
        return map;
    }
    return [...generator(5)].flatMap(
        (tiley) => map.map((origLine) => [...generator(5)].flatMap((tilex) => origLine.map(val => ((val - 1 + tilex + tiley) % 9 + 1)))))

}


function puzzle(lines: string[], part: Part): void {
    const map = preprocess_map(parse(lines), part);
    const caveMap: CaveMap = {
        map,
        maxX: map[0].length - 1,
        maxY: map.length - 1
    }

    const result = lookup_min_path(caveMap);

    console.log(`Result ${result[0]}`);
}

run(15, testData, [Type.TEST, Type.RUN], puzzle, [Part.PART_1, Part.PART_2]);
console.log("All Done")