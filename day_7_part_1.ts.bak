import * as fs from 'fs';

const data = fs.readFileSync('./data/day_7_1.dat', 'utf-8');

const lines = data.split(/\r?\n/);
const startingPositions = lines[0].split(",").map(it => parseInt(it));
startingPositions.sort((a, b) => a - b)
const mean = startingPositions[Math.floor(startingPositions.length / 2)]
const avg = Math.round(startingPositions.reduce((sum, curr) => sum + curr) / startingPositions.length)
const start = Math.min(mean, avg);
const end = Math.max(mean, avg);

function calcDistance(targetPos: number): number {
    return startingPositions.reduce((total, pos) => total + Math.abs(pos - targetPos), 0)
}
let result = calcDistance(start)
let pos = start;
for (; pos <= end; pos++) {
    const newResult = calcDistance(pos);
    if(newResult>result){
        break;
    }
    result = newResult;
}
console.log(`"Result ${result} for ${pos-1}`)