import * as fs from 'fs';

const data = fs.readFileSync('./data/day_5_1.dat', 'utf-8');

const lines = data.split(/\r?\n/);

const regexp = /(\d+),(\d+)\s*->\s*(\d+),(\d+)/
type Point = { x: number, y: number }
type Vector = { orig: Point, dest: Point }
type Grid = {
    [key: string]: number
}

let maxX: number = 0;
let maxY: number = 0

function updateMax(point: Point) {
    maxX = (maxX > point.x) ? maxX : point.x
    maxY = (maxY > point.y) ? maxY : point.y
}

function gridKey(point: Point): string {
    return `${point.x}:${point.y}`;
}

function getSign(x: number, y: number): number {
    if (x === y) return 0
    else if (x < y) return 1
    return -1
}

function drawLine(vector: Vector, grid: Grid) {
    let x = vector.orig.x
    let y = vector.orig.y
    const signX = getSign(vector.orig.x, vector.dest.x)
    const signY = getSign(vector.orig.y, vector.dest.y)
    do {
        const point = { x, y }
        updateMax(point)
        const key = gridKey(point)
        grid[key] = (grid[key] ?? 0) + 1
        if (x === vector.dest.x && y === vector.dest.y) break;
        x += signX
        y += signY
    } while (true)
    return grid
}

function toString(grid: Grid): string {
    const result = [];
    for(let y=0;y<=maxY;y++){
        for(let x=0;x<=maxX;x++){
            result.push(grid[gridKey({x,y})]?? "-")
        }
        result.push("\n");
    }
    return result.join("");
}

function draw(vector: Vector, grid: Grid): Grid {

    if (vector.dest.x === vector.orig.x) {
        drawLine(vector, grid)
    }
    else if (vector.dest.y === vector.orig.y) {
        drawLine(vector, grid)
    }
    else if (Math.abs(vector.dest.x - vector.orig.x) === Math.abs(vector.dest.y - vector.orig.y)) {
        drawLine(vector, grid)
    }
    return grid;
}
const grid = lines
    .map(line => {
        const result = line.match(regexp);
        if (result && result.length > 0) {
            return <Vector>{
                orig: {
                    x: parseInt(result[1]),
                    y: parseInt(result[2])
                },
                dest: {
                    x: parseInt(result[3]),
                    y: parseInt(result[4])
                },
            }
        }
    })
    .filter(vector => vector !== undefined)
    .map(vector => vector as Vector)
    .reduce((grid, vector) => draw(vector, grid), <Grid>{})

const allpoints = Object.entries(grid)
allpoints.sort((a, b) => a[0].localeCompare(b[0]))
const points = allpoints.filter(it => it[1] > 1)
console.log(toString(grid));
console.log("Result " + points.length + "\n" + points)