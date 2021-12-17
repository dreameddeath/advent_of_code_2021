import { Part, run, Type } from "../day_utils"
const testData = `target area: x=20..30, y=-10..-5`

type Data = {
    minX: number,
    maxX: number,
    minY: number,
    maxY: number
}

function parse(line: string): Data {
    const match = line.match(/[^:]+:\s+x=(-?\d+)\.\.(-?\d+),\s+y=(-?\d+)\.\.(-?\d+)/);
    if (match && match.length > 4) {
        const x1 = parseInt(match[1]);
        const x2 = parseInt(match[2]);
        const y1 = parseInt(match[3]);
        const y2 = parseInt(match[4]);
        return {
            minX: Math.min(x1, x2),
            maxX: Math.max(x1, x2),
            minY: Math.min(y1, y2),
            maxY: Math.max(y1, y2)
        }
    }
    throw "ERROR"
}

function is_in_range(pos: number, min_pos: number, max_pos: number): boolean {
    return pos >= min_pos && pos <= max_pos;
}

type SimResult = {
    steps_in_reach: number[],
    min_step_in_reach: number,
    max_step_in_reach: number,
    max_reached: number,
    has_ended_in_reach: boolean
}

function simulateX(minX: number, maxX: number, init_vx: number): SimResult {
    let x_pos = 0;
    let curr_vx = init_vx;
    let max_x_reached = 0;
    let reaching_steps: number[] = []
    let currStep = 0;
    do {
        currStep++
        x_pos += curr_vx;
        max_x_reached = Math.max(max_x_reached, x_pos);
        if (curr_vx !== 0) {
            curr_vx += (curr_vx > 0) ? - 1 : 1;
        }

        if (is_in_range(x_pos, minX, maxX)) {
            reaching_steps.push(currStep);
        }
    } while (curr_vx != 0 && x_pos <= maxX)
    return {
        steps_in_reach: reaching_steps,
        min_step_in_reach: reaching_steps[0],
        max_step_in_reach: reaching_steps[reaching_steps.length - 1],
        max_reached: max_x_reached,
        has_ended_in_reach: is_in_range(x_pos, minX, maxX)
    }
}

function simulateY(minY: number, maxY: number, init_vy: number): SimResult {
    let y_pos = 0;
    let max_y_reached = 0;
    let curr_vy = init_vy;
    let reaching_steps: number[] = []
    let currStep = 0;
    do {
        currStep++
        y_pos += curr_vy;
        max_y_reached = Math.max(max_y_reached, y_pos);
        curr_vy -= 1;
        if (is_in_range(y_pos, minY, maxY)) {
            reaching_steps.push(currStep);
        }
    } while (y_pos >= minY || curr_vy > 0)
    return {
        steps_in_reach: reaching_steps,
        max_reached: max_y_reached,
        min_step_in_reach: reaching_steps[0],
        max_step_in_reach: reaching_steps[reaching_steps.length - 1],
        has_ended_in_reach: false
    }
}

type AllSimSucces = { result: SimResult, initial_v: number }[]
function simulateAll_vy(minY: number, maxY: number): AllSimSucces {
    const results: AllSimSucces = []
    for (let vy = -1000; vy < 1000; vy++) {
        const result = simulateY(minY, maxY, vy);
        if (result.steps_in_reach.length > 0) {
            results.push({ result, initial_v: vy })
        }
    }
    return results;
}

function simulateAll_vx(minX: number, maxX: number): AllSimSucces {
    const results: AllSimSucces = []
    for (let vx = 1; vx <= maxX + 1; vx++) {
        const result = simulateX(minX, maxX, vx);
        if (result.steps_in_reach.length > 0) {
            results.push({ result, initial_v: vx })
        }
    }
    return results;
}

function are_steps_compatibles(vx_res: SimResult, vy_res: SimResult) {
    if (vx_res.min_step_in_reach > vy_res.max_step_in_reach) {
        return false;
    }
    if (!vx_res.has_ended_in_reach && vx_res.max_step_in_reach < vy_res.min_step_in_reach) {
        return false;
    }
    return true;
}

function combine(allVx: AllSimSucces, allVy: AllSimSucces): number {
    const combinations = new Set<string>();

    allVx.forEach(
        vx_res => allVy.forEach(
            vy_res => {
                if (are_steps_compatibles(vx_res.result, vy_res.result)) {
                    combinations.add(`${vx_res.initial_v},${vy_res.initial_v}`)
                }
            }
        )
    )
    return combinations.size
}

function puzzle(lines: string[], part: Part): void {

    const data = parse(lines[0]);
    const allVY = simulateAll_vy(data.minY, data.maxY);
    const maxY = allVY.reduce((max, res) => Math.max(res.result.max_reached, max), 0)
    const allVx = simulateAll_vx(data.minX, data.maxX);
    const result = combine(allVx, allVY);
    //const maxStep = min_vx(data);
    console.log("Max :" + maxY + "\nCombinations " + result);
}

run(17, testData, [Type.TEST, Type.RUN], puzzle, [Part.PART_1])