import * as fs from 'fs';

type BitCount = [
    number,
    number
]

type NumberBits = ("0" | "1")[]

type State = BitCount[]
const data = fs.readFileSync('./data/day_3_1.dat', 'utf-8');

// split the contents by new line


const parsedLines: NumberBits[] = data.split(/\r?\n/)
    .map(line => line.split('') as NumberBits)


function countBitsAtPos(list: NumberBits[], pos: number): BitCount {
    const initState: BitCount = [0, 0]
    return list.reduce((state, bits) =>
        [state[0] + (bits[pos] === "0" ? 1 : 0), state[1] + (bits[pos] === "1" ? 1 : 0)]
        , initState)
}

function keepLines(lines: NumberBits[], pos: number, filter: (bit: "0" | "1", count: BitCount) => boolean): NumberBits[] {
    if (lines.length === 1) { return lines }
    const count = countBitsAtPos(lines, pos)
    return lines.filter(bits => filter(bits[pos], count))
}

const oxygen = Array<number>(20).fill(0)
    .reduce(
        (lines, _, index) => keepLines(lines, index, (bit, count) => bit === (count[1] >= count[0] ? "1" : "0"))
        , parsedLines)

const co2 = Array<number>(20).fill(0)
    .reduce(
        (lines, _, index) => keepLines(lines, index, (bit, count) => bit === (count[1] >= count[0] ? "0" : "1"))
        , parsedLines)



const o2val = oxygen[0].reverse().reduce((result, val, index) => result + ((val === "1") ? (2n ** BigInt(index)) : 0n), 0n)
const co2val = co2[0].reverse().reduce((result, val, index) => result + ((val === "1") ? (2n ** BigInt(index)) : 0n), 0n)

console.log("Result " + (o2val * co2val))