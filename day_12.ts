import { Part, run, Type } from "./day_utils"
const testData = `fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW`

type Graph = Map<string, string[]>

type Path = string[]


function isBigCave(name: string): boolean {
    return name[0] === name[0].toUpperCase()
}

function getMatchingCaves(potentialCaves: string[], currPath: Path, allow_duplicate_small_cave: boolean): [boolean, string][] {
    return potentialCaves
        .filter(it => it !== "start")
        .flatMap(it => {
            if (isBigCave(it)) {
                return [[allow_duplicate_small_cave, it]]
            }
            else {
                const isExisting = currPath.includes(it);
                if (allow_duplicate_small_cave || !isExisting) {
                    return [[allow_duplicate_small_cave && !isExisting, it]]
                }
                else {
                    return []
                }
            }
        })
}


function lookupPathesFast(currentCave: string, graph: Graph, currPath: Path, allPathes: Path[], allow_duplicate_small_cave: boolean) {
    if (currentCave === 'end') {
        allPathes.push([...currPath])
        return allPathes
    }
    const nodes = getMatchingCaves(graph.get(currentCave) ?? [], currPath, allow_duplicate_small_cave)
    for (const next of nodes) {
        currPath.push(next[1]);
        lookupPathesFast(next[1], graph, currPath, allPathes, next[0]);
        currPath.pop();
    }
}


function puzzle(lines: string[], part: Part): void {
    const graph: Graph = new Map<string, string[]>()
    lines.map(line => line.split("-"))
        .forEach(parts => {
            if (!graph.has(parts[0])) {
                graph.set(parts[0], [])
            }
            if (!graph.has(parts[1])) {
                graph.set(parts[1], [])
            }
            graph.get(parts[0])?.push(parts[1])
            graph.get(parts[1])?.push(parts[0])
        })

    const paths: string[][] = [];
    lookupPathesFast("start", graph, ["start"], paths, (part === Part.PART_1) ? false : true)
    console.log(`Result : ${paths.length}`)
}

run(12, testData, [Type.TEST, Type.RUN], puzzle, [Part.PART_1, Part.PART_2])
