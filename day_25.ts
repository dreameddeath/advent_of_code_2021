import { Part, run, Type } from "./day_utils"
const testData = `v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>`

const EMPTY = '.'
const RIGHT = '>'
const DOWN = 'v';

type SeaFloorType = '.' | '>' | 'v';
type MOVABLE_TYPE = '>' | 'v';
type SeaFloorRow = SeaFloorType[]
type SeaFloor = SeaFloorRow[];
function parse_line(line: string): SeaFloorType[] {
    return line.split("").map(c => c as SeaFloorType);
}
function parse(lines: string[]): SeaFloor {
    return lines.map(line => parse_line(line));
}

function move(seafloor: SeaFloor, type: MOVABLE_TYPE): number {
    const max_line = seafloor.length - 1;
    const max_col = seafloor[0].length - 1;
    let moves = 0;
    if (type === DOWN) {
        for (let x = 0; x <= max_col; ++x) {
            const can_move_last = seafloor[0][x] === EMPTY;
            for (let y = 0; y <= max_line; ++y) {
                if (seafloor[y][x] !== DOWN) continue;
                if (y === max_line) {
                    if (can_move_last) {
                        seafloor[y][x] = EMPTY;
                        seafloor[0][x] = DOWN;
                        moves++
                    }
                }
                else if (seafloor[y + 1][x] === EMPTY) {
                    seafloor[y][x] = EMPTY;
                    seafloor[y + 1][x] = DOWN
                    moves++
                    y++;
                }
            }
        }
    }
    else {
        for (let y = 0; y <= max_line; ++y) {
            const can_move_last = seafloor[y][0] === EMPTY;
            for (let x = 0; x <= max_col; ++x) {
                if (seafloor[y][x] !== RIGHT) continue;
                if (x === max_col) {
                    if (can_move_last) {
                        seafloor[y][x] = EMPTY;
                        seafloor[y][0] = RIGHT;
                        moves++
                    }
                }
                else if (seafloor[y][x + 1] === EMPTY) {
                    seafloor[y][x] = EMPTY;
                    seafloor[y][x + 1] = RIGHT;
                    moves++
                    x++;
                }
            }
        }
    }
    return moves;
}

function max_moves(seafloor: SeaFloor): number {
    let nb_loops = 0
    while (true) {
        nb_loops++;
        const moves_right = move(seafloor, RIGHT);
        const moves_down = move(seafloor, DOWN);
        if ((moves_down + moves_right) === 0) return nb_loops;
    }
}

function puzzle(lines: string[], part: Part): void {
    const data = parse(lines);
    if (part === Part.PART_1) {
        const result = max_moves(data);
        console.log(`Results ${result}`);
    }
    else {
        console.log(`Results ${2}`);
    }
}

run(25, testData, [Type.TEST, Type.RUN], puzzle, [Part.PART_1, Part.PART_2])