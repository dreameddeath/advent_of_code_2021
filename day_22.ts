import { Part, run, Type } from "./day_utils"
const testData = ``


function puzzle(lines: string[], part: Part): void {
    if (part === Part.PART_1) {
        const result = 1;
        console.log(`Results ${result}`);
    }
    else {
        const result = 2;
        console.log(`Results ${result}`);
    }
}

run(22, testData, [Type.TEST, Type.RUN], puzzle, [Part.PART_1, Part.PART_2])