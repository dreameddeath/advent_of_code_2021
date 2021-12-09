import * as fs from 'fs';

const data = fs.readFileSync('./data/day_9_1.dat', 'utf-8');

const lines = data.split(/\r?\n/);


const heatMap = lines
    .map(line => line.split("").map(it => parseInt(it)))

type Point = { x: number, y: number }
type CurrentBassin = Point[]

function isInCurrentBassin(pos: Point, bassin: CurrentBassin): boolean {
    return bassin.filter(item => item.x === pos.x && item.y === pos.y).length > 0
}

function getVal(pos: Point): number {
    return heatMap[pos.y]?.[pos.x] ?? 10
}

function getListToExplore(pos: Point, bassin: CurrentBassin): Point[] {
    return [
        { x: pos.x + 1, y: pos.y },
        { x: pos.x - 1, y: pos.y },
        { x: pos.x, y: pos.y - 1 },
        { x: pos.x, y: pos.y + 1 }]
        .filter(point =>
            !isInCurrentBassin(point, bassin)
            && getVal(point) < 9
        )
}

function appendToBassinIfApplicable(pos: Point, currBassin: CurrentBassin, min: number): Point[] {
    if (isInCurrentBassin(pos, currBassin)) {
        return []
    }
    const curr = getVal(pos)
    if (curr >= 9) return []

    const list = getListToExplore(pos, currBassin)
    const match = list.every(other => getVal(other) > min)

    if (match) {
        currBassin.push(pos);
        return list;
    }
    return []
}

function calcBassin(pos: Point): CurrentBassin {
    const currentBassin: CurrentBassin = []
    let listToExplore = [pos];
    const currVal = getVal(pos);
    if (currVal >= 9) return []
    const isMin = getListToExplore(pos, []).every(other => getVal(other) > currVal)
    if (!isMin) {
        return []
    }
    while (listToExplore.length > 0) {
        const point = listToExplore.pop()
        if (point === undefined) break;
        const newList = appendToBassinIfApplicable(point, currentBassin, currVal)
        listToExplore = listToExplore.concat(newList)
    }
    return currentBassin
}

const result = []
for (let y = 0; y < lines.length; ++y) {
    for (let x = 0; x < lines[y].length; ++x) {
        const pos: Point = { x, y }
        const found = calcBassin(pos)
        if (found.length > 0) {
            result.push({ bassin: found, pos })
        }
    }
}
result.sort((a, b) => b.bassin.length - a.bassin.length)

console.log("Result " + result[0].bassin.length * result[1].bassin.length * result[2].bassin.length)