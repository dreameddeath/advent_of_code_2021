import { Part, run, Type } from "../day_utils"
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

type Edge = {
    start: string,
    end: string
}

type Graph = Map<string, Set<string>>
const TWICE_SMALL_CAVE_MARKER: unique symbol = Symbol("twice_small_cave")

type Path = (string | typeof TWICE_SMALL_CAVE_MARKER)[]
type NextCaveFilter = (name: string, currPath: Path) => boolean
function isBigCave(name: string): boolean {
    return name[0] === name[0].toUpperCase()
}

function pathToAppend(name: string, currPath: Path): Path {
    return (currPath.find(it => !isBigCave(name) && (it === name)) !== undefined) ? [name, TWICE_SMALL_CAVE_MARKER] : [name]
}

function filterPartOne(name: string, currPath: Path): boolean {
    return name !== "start" && (isBigCave(name) || currPath.find(it => it === name) === undefined)
}

function filterPartTwo(name: string, currPath: Path): boolean {
    if (name === "start") return false
    return name !== "start" && (
        isBigCave(name) ||
        currPath.filter(it => it === name || it === TWICE_SMALL_CAVE_MARKER).length <= 1
    )
}


function lookupPathes(currentCave: string, graph: Graph, currPath: Path, allPathes: Path[], filter: NextCaveFilter): Path[] {
    if (currentCave === 'end') {
        return allPathes.concat([currPath])
    }
    const nodes = graph.get(currentCave) ?? new Set<string>()
    return [...nodes.values()]
        .filter(name => filter(name, currPath))
        .reduce((pathes, nextCave) =>
            lookupPathes(nextCave, graph, currPath.concat(pathToAppend(nextCave, currPath)), pathes, filter), allPathes)
}

function puzzle(lines: string[], part: Part): void {
    const graph: Graph = new Map<string, Set<string>>()
    lines.map(line => line.split("-"))
        .forEach(parts => {
            if (!graph.has(parts[0])) {
                graph.set(parts[0], new Set<string>())
            }
            if (!graph.has(parts[1])) {
                graph.set(parts[1], new Set<string>())
            }
            graph.get(parts[0])?.add(parts[1])
            graph.get(parts[1])?.add(parts[0])
        })

    const paths = lookupPathes("start", graph, ["start"], [], (part === Part.PART_1) ? filterPartOne : filterPartTwo)
    //paths.forEach(it => console.log(it))
    console.log(`Result : ${paths.length}`)
}

run(12, testData, [Type.TEST, Type.RUN], puzzle, [Part.PART_1, Part.PART_2])
