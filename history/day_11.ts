import { generator, getData, Type } from "../day_utils"
const testData = `5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526`

const lines = getData(11, Type.RUN, testData)

const octopuses: Octopuses = lines.map(it => it.split("").map(val => parseInt(val)))

type Octopuses = number[][]

type Pos = {
    x: number,
    y: number
}

function getNeighboursPos(currPos: Pos, octopuses: Octopuses, fct: (val: number) => boolean): Pos[] {
    const neighbourPositions: Pos[] = [
        { x: currPos.x - 1, y: currPos.y }, { x: currPos.x + 1, y: currPos.y }, //Pos Left/Right
        { x: currPos.x - 1, y: currPos.y - 1 }, { x: currPos.x + 1, y: currPos.y - 1 }, //Diag Up Left/Up Right
        { x: currPos.x - 1, y: currPos.y + 1 }, { x: currPos.x + 1, y: currPos.y + 1 }, //Pos Down Left/Down Right
        { x: currPos.x, y: currPos.y - 1 }, { x: currPos.x, y: currPos.y + 1 }  //Pos Up/down
    ];
    return neighbourPositions
        .filter(pos => octopuses[pos.y] !== undefined && octopuses[pos.y][pos.x] !== undefined)
        .filter(pos => fct(octopuses[pos.y][pos.x]))
}


function incBy(octopuses: Octopuses, positions: Pos[], val: number) {
    positions.forEach(pos => {
        octopuses[pos.y][pos.x] += val
    })
}

function getFlashing(octopuses: Octopuses): Pos[] {
    return octopuses.flatMap((line, y) => line.flatMap((val, x) => <Pos & { val: number }>{ x, y, val })).filter(it => it.val > 9)
}


function performStep(flashes: number, octopuses: Octopuses, index: number): number {
    const allPos: Pos[] = octopuses.flatMap((line, y) => line.flatMap((val, x) => <Pos>{ x, y }))

    incBy(octopuses, allPos, 1)
    do {
        const listToFlash: Pos[] = getFlashing(octopuses)
        if (listToFlash.length === 0) break;
        flashes += listToFlash.length
        listToFlash.forEach(
            pos => {
                octopuses[pos.y][pos.x] = 0
                incBy(octopuses, getNeighboursPos(pos, octopuses, (val) => val !== 0), 1)
            }
        )

    } while (true)
    const flashed = octopuses.flatMap((line) => line.flatMap((val) => val)).filter(val => val === 0)
    if (flashed.length === allPos.length) {
        console.log(`Synchronized at ${index + 1}`)
    }
    if (index === 100) {
        console.log(`Result at 100 :${flashes}`)
    }
    return flashes
}


[...generator(1000)].reduce((flashes, _number, index) => performStep(flashes, octopuses, index), 0)
