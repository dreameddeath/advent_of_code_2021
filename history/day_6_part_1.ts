import * as fs from 'fs';

const data = fs.readFileSync('./data/day_6_1.dat', 'utf-8');

const lines = data.split(/\r?\n/);

const cache: { [key: string]: number } = {}



function simulate(startNumber: number, iterations: number): number {
    if (iterations <= 0) {
        return 1
    }
    const key = `${startNumber}:${iterations}`;
    const inCache = cache[key];
    if (inCache) return inCache
    if (startNumber === 0) {
        cache[key] = simulate(8, iterations - 1) + simulate(6, iterations - 1);
    }
    else {
        cache[key] = simulate(startNumber-1, iterations - 1);
    }
    return cache[key]
}
for (let interations = 0; interations < 256; interations++) {
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].forEach((val) => simulate(val, interations))
}

const result = lines[0].split(",").map(it => simulate(parseInt(it),256)).reduce((tot, future) => tot + BigInt(future), 0n)

console.log("Result " + result)