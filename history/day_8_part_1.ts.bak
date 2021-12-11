import * as fs from 'fs';

const data = fs.readFileSync('./data/day_8_1.dat', 'utf-8');

const lines = data.split(/\r?\n/);
type Segments = "a" | "b" | "c" | "d" | "e" | "f" | "g"
type Digit = Set<Segments>
type Line = {
    inputs: Digit[],
    outputs: Digit[]
}

function parseDigit(digitStr: string): Digit {
    return digitStr.split("").reduce((digit, char) => digit.add(char as Segments), new Set<Segments>())
}

const parts = lines.map(line => line.split("|")).map(([source, output]) => <Line>{
    inputs: source.trim().split(/\s+/).map(it => parseDigit(it)),
    outputs: output.trim().split(/\s+/).map(it => parseDigit(it))
});




const result = parts.map(line => line.outputs.filter(digit => [2, 3, 4, 7].find(size => size === digit.size) !== undefined).length).reduce((sum, part) => sum + part)

console.log(`"Result ${result} `)