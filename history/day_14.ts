import { generator, Part, run, Type } from "../day_utils"
const testData = `NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C`

type Rule = {
    binary: string,
    target: string,
}


type ParsedReturn = {
    start: PolymerMap;
    rules: Rule[],
    compiled: CompiledRules,
    lastChar: string
}

type CompiledRules = Map<string, string>
type PolymerMap = Map<string, bigint>

function parse(lines: string[]): ParsedReturn {
    const startParsing = lines[0].split("").reduce((state: { start: string[], last: string | undefined }, char) => {
        return { start: state.start.concat((state.last !== undefined) ? [state.last + char] : []), last: char }
    }, { start: [], last: undefined })

    const rules = lines.slice(2).flatMap(line => {
        const parts = line.split(' -> ');
        const components = parts[0].split("");
        return [<Rule>{
            binary: [...components].join(""),
            target: parts[1]
        }
        ]
    })
    const compiled = rules.reduce((compiled, rule) =>
        compiled.set(rule.binary, rule.target)
        , new Map<string, string>())
    const polymers = startParsing.start.reduce((map, item) => addToMap(map, item, 1n), new Map<string, bigint>())
    return { start: polymers, rules, compiled, lastChar: startParsing.start[startParsing.start.length - 1][1] };
}

function addToMap(map: PolymerMap, item: string, value: bigint): PolymerMap {
    const current = map.get(item) ?? 0n
    return map.set(item, current + value)
}

function apply(map: PolymerMap, rules: CompiledRules): PolymerMap {
    const newMap = new Map<string, bigint>()
    for (const entry of map.entries()) {
        const rule = rules.get(entry[0])
        if (rule) {
            [entry[0][0] + rule, rule + entry[0][1]].forEach(item => {
                addToMap(newMap, item, entry[1])
            })
        }
        else {
            addToMap(newMap, entry[0], entry[1])
        }
    }
    return newMap;
}



function count(map: PolymerMap, lastPair: string): void {
    const counter = new Map<string, bigint>();
    for (const entry of map.entries()) {
        const chars = [entry[0][0]]
        if (entry[0] === lastPair) {
            chars.push(entry[0][1])
        }
        chars.forEach(char => {
            addToMap(counter, char, entry[1])
        })
    }
    const sorted = [...counter.entries()].sort((a, b) => (a[1] < b[1]) ? -1 : ((a[1] == b[1]) ? 0 : 1));
    const first = sorted[0]
    const last = sorted[sorted.length - 1]
    const diff = ((last[0] === lastPair) ? 1n : 0n) + last[1] - first[1];
    console.log(`result ${diff}`)
    return
}


function puzzle(lines: string[], part: Part): void {
    const parseResult = parse(lines)
    if (part === Part.PART_1) {
        const final = [...generator(10)].reduce((map: PolymerMap, index) => apply(map, parseResult.compiled), parseResult.start)
        count(final, parseResult.lastChar)
    }
    else {
        const final = [...generator(40)].reduce((map: PolymerMap, index) => apply(map, parseResult.compiled), parseResult.start)
        count(final, parseResult.lastChar)
    }
}

run(14, testData, [Type.TEST, Type.RUN], puzzle, [Part.PART_1, Part.PART_2])
