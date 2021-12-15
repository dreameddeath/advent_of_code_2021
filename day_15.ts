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
type ExploredMap = (boolean | undefined)[][]

function parse(lines: string[]): number[][] {
    return lines.map(line => line.split("").map(num => parseInt(num)));
}

function generate_siblings(curr_pos: Pos, caveMap: CaveMap): Pos[] {
    return [{ x: curr_pos.x + 1, y: curr_pos.y },
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

function is_same(pos?: Pos, other?: Pos): boolean {
    return pos?.x === other?.x && pos?.y === other?.y;
}

function insert_into_priority_queue(item: WeightedPos, queue: WeightedPos[]) {
    const cost = item.comparision_cost;
    const allValues = queue.length;
    for (let x = 0; x < allValues; x++) {
        if (queue[x].comparision_cost < cost) {
            queue.splice(x, 0, item);
            return;
        }
    }
    queue.push(item)
}


function lookup_min_path(caveMap: CaveMap): [number, Pos[]] {
    const explored: WeightedPos[][] = [...generator(caveMap.map.length)].map(_ => []);
    const totalNodes = caveMap.map.length * caveMap.map[0].length

    const listToExplore: WeightedPos[] = [];
    insert_into_priority_queue(build_weighted_pos({ x: 0, y: 0 }, caveMap), listToExplore)
    while (listToExplore.length > 0) {
        const nextNode = listToExplore.pop();
        if (nextNode === undefined) {
            break;
        }
        if (nextNode.pos.x === caveMap.maxX && nextNode.pos.y === caveMap.maxY) {
            const exploredCount = explored.reduce((countTotal, line) => countTotal + line.reduce((countLine, cell) => countLine + ((cell !== undefined) ? 1 : 0), 0), 0);
            console.log(`Explored ${exploredCount} vs ${totalNodes}`)
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
            const existingInBestPath = explored[sibling.pos.y]?.[sibling.pos.x];
            if (existingInBestPath !== undefined) {
                continue;
            }
            const existingInSortedQueue = listToExplore.findIndex(node => is_same(node.pos, sibling.pos));
            if (existingInSortedQueue !== -1) {
                const current = listToExplore[existingInSortedQueue];
                if (current.comparision_cost < sibling.comparision_cost) {
                    continue;
                }
                listToExplore.splice(existingInSortedQueue, 1);
            }
            insert_into_priority_queue(sibling, listToExplore)
        }
        explored[nextNode.pos.y][nextNode.pos.x] = nextNode
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

run(15, testData, [Type.TEST, Type.RUN], puzzle, [Part.PART_1, Part.PART_2])