import * as fs from 'fs';

type BitCount = [
    number,
    number
]


type State = BitCount[]
const data = fs.readFileSync('./data/day_3_1.dat', 'utf-8');

// split the contents by new line
const initState: State = []

const finalState = data.split(/\r?\n/)
    .map(line => line.split('').reverse() as ("0" | "1")[])
    .reduce(
        (state: State, val) =>
            val.reduce((newState: State, char, index) => {
                const result = [...newState]
                if (result[index] === undefined) {
                    result[index] = [0, 0]
                }
                result[index] = [result[index][0] + (char === "0" ? 1 : 0), result[index][1] + (char === "1" ? 1 : 0)]
                return result
            }, state)
        , initState)

const gamma = finalState.reduce((result, val, index) => result + (val[1] > val[0] ? 2n ** BigInt(index) : 0n), 0n)
const epsilon = finalState.reduce((result, val, index) => result + (val[1] < val[0] ? 2n ** BigInt(index) : 0n), 0n)

console.log("Result " + (gamma * epsilon))