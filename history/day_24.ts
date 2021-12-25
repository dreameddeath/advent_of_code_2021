import { assert } from "console";
import { Part, run, Type } from "../day_utils"
const testData = ``

enum Operator {
    ADD = "add",
    MUL = "mul",
    DIV = "div",
    MODULO = "mod",
    EQUAL = "eql"
}

const parser = /(\w+)\s+(\w+)(?:\s+(-?\d+|\w+))?/;

type Input = {
    type: "input",
    variable: string,
}

type Operation = {
    type: "operator",
    operator: Operator,
    main_var: string,
    operand: {
        value: number
    } | {
        var: string,
    }
}

type Instruction = Input | Operation;

function parse_line(line: string): Instruction[] {
    const matcher = line.match(parser);
    if (matcher && matcher.length > 0) {
        if (matcher[1] === "inp") {
            return [{ type: "input", variable: matcher[2] }];
        }
        else {
            let value: number = parseInt(matcher[3]);
            let operand = isNaN(value) ? { var: matcher[3] } : { value: value };
            return [{ type: "operator", operator: matcher[1] as Operator, main_var: matcher[2], operand }];
        }
    }
    return [];
}

function parse(lines: string[]): Instruction[] {
    return lines.flatMap(line => parse_line(line));
}

type SubProgram = {
    instructions: Operation[],
    input: string,
    id: number,
}

type Memory = { [key: string]: number }


function group_in_subprograms(instructions: Instruction[]): SubProgram[] {
    const sub_programs: SubProgram[] = [];
    for (const instruction of instructions) {
        if (instruction.type === "input") {
            const new_sub_program: SubProgram = {
                input: instruction.variable,
                instructions: [],
                id: sub_programs.length
            };
            sub_programs.push(new_sub_program)
        }
        else {
            const curr_sub_program = sub_programs[sub_programs.length - 1];
            curr_sub_program.instructions.push(instruction);
        }
    }
    return sub_programs;
}

function process_operation(alu: Memory, operation: Operation) {
    const b = ("value" in operation.operand) ? operation.operand.value : alu[operation.operand.var];
    const a = alu[operation.main_var] ?? 0
    switch (operation.operator) {
        case Operator.ADD: alu[operation.main_var] = a + b; break;
        case Operator.MUL: alu[operation.main_var] = a * b; break;
        case Operator.DIV: alu[operation.main_var] = Math.floor(a / b); break;
        case Operator.EQUAL: alu[operation.main_var] = a === b ? 1 : 0; break;
        case Operator.MODULO: alu[operation.main_var] = a % b; break;
    }
}

function run_sub_program(alu: Memory, sub_program: SubProgram, input: number) {
    alu[sub_program.input] = input;
    sub_program.instructions.forEach(instruction => process_operation(alu, instruction));
}

function check_number(input: number[], sub_programs: SubProgram[]): boolean {
    const memory: Memory = {}
    for (let num = 0; num < 14; num++) {
        run_sub_program(memory, sub_programs[num], input[num]);
    }
    return memory["z"] === 0;
}

type Possibilities = [number, number][]

function run_all_possibilities(sub_program: SubProgram, orig_z: number): Possibilities {
    const result: Possibilities = [];
    for (const input of [1, 2, 3, 4, 5, 6, 7, 8, 9]) {
        const memory: Memory = { z: orig_z }
        run_sub_program(memory, sub_program, input);
        result.push([input, memory["z"]]);
    }
    return result;
}

type ZValuesPossibleInputMapToInputDigit = Map<number, { digit: number, target_z: number }[]>

type ZValuesAllowedPerProgram = {
    [key: number]: ZValuesPossibleInputMapToInputDigit
}



function solve(instructions: Instruction[]) {

    const sub_programs = group_in_subprograms(instructions);

    const z_values_for_program: ZValuesAllowedPerProgram = {};
    z_values_for_program[14] = new Map();
    z_values_for_program[14].set(0, []);


    while (Object.keys(z_values_for_program).length != 15) {
        let list_z_values: Set<number> = new Set([0]);
        for (const program of sub_programs) {
            if (z_values_for_program[program.id]) {
                break;
            }
            const set_next_z_value: Set<number> = new Set();
            console.log("starting program " + program.id + " with " + list_z_values.size + " values");
            const t0 = performance.now();
            const allowed_target_values = z_values_for_program[program.id + 1];
            if (allowed_target_values) {
                z_values_for_program[program.id] = new Map();
            }
            for (const z_value of list_z_values) {
                const results = run_all_possibilities(program, z_value);
                for (const result of results) {
                    const input_digit = result[0];
                    const result_z = result[1];
                    if (allowed_target_values===undefined) {
                        set_next_z_value.add(result_z);
                    }
                    else if(allowed_target_values.has(result_z)){
                        const new_found_value = {
                            digit: input_digit,
                            target_z: result_z
                        };
                        const hash_key = z_values_for_program[program.id].get(z_value);
                        if (hash_key) {
                            hash_key.push(new_found_value)
                        }
                        else {
                            z_values_for_program[program.id].set(z_value, [new_found_value])
                        }
                    }
                }
            }
            const t1 = performance.now();
            console.log("done program " + program.id + " producing " + set_next_z_value.size + " values in " + (t1 - t0) + " ms");
            list_z_values = set_next_z_value;
        }
    }
    //5 (21) 9 (558) 9 (14519) 9 (377510) 6 (14519) 9 (377509) 1 (14519) 2 (377507) 9 (14519) 8 (558) 1 (14520) 9 (558) 3 (21) 9
    //59996912981939
    // 1 (17) 7 (452) 2 (11756) 4 (305667) 1 (11756) 9 (305671) 1 (11756) 1 (305668) 8 (11756) 1 (452) 1 (11764) 9 (452) 1 (17) 5
    //17241911811915
    console.log("Results " + z_values_for_program);
}

function puzzle(lines: string[], part: Part): void {
    const instructions = parse(lines);
    assert(instructions.length === lines.length)
    if (part === Part.PART_1) {
        solve(instructions);
        console.log(`Results ${1}`);
    }
    else {
        console.log(`Results ${2}`);
    }
}

run(24, testData, [Type.RUN], puzzle, [Part.PART_1, Part.PART_2])