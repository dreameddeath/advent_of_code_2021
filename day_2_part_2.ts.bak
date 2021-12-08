import * as fs from 'fs';

type Direction = "forward" | "down" | "up"

type Directive = {
    direction: Direction,
    size: number
}

type State = {
    position: number
    depth: number,
    aim: number
}
const data = fs.readFileSync('./data/day_2_1.dat', 'utf-8');

// split the contents by new line
const initState: State = { position: 0, depth: 0, aim: 0 }
const count = data.split(/\r?\n/)
    .map(line => {
        const parts = line.split(/\s+/)
        return <Directive>{
            direction: parts[0] as Direction,
            size: parseInt(parts[1])
        }
    }).reduce(
        (state: State, val) => {
            switch (val.direction) {
                case 'up': return { ...state, aim: state.aim - val.size }
                case 'down': return { ...state, aim: state.aim + val.size }
                case 'forward': return { ...state, position: state.position + val.size, depth: state.depth + (state.aim * val.size) }
                default: return state
            }
        }, initState)
console.log("Count " + count.depth * count.position)