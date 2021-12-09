import * as fs from 'fs';

class HeightMap {

}

type ParsingResult = {
    result: number,
    savedList: number[][],
}


const data = fs.readFileSync('./data/day_9_1.dat', 'utf-8');

const lines = data.split(/\r?\n/);
const lineSize = lines[0].length;

function calcContribution(current: number[], previous?: number[], next?: number[]): number {
    return current.reduce((contrib, val, index, all) => {
        const above = previous?.[index] ?? 10
        const below = next?.[index] ?? 10;
        const before = all[index - 1] ?? 10;
        const after = all[index + 1] ?? 10;
        return contrib + (([above, below, before, after].filter(other => other > val).length === 4) ? val + 1 : 0)
    },
        0)
}

const parsedLines: ParsingResult =
    lines
        .map(line => line.split("").map(it => parseInt(it)))
        .reduce((state, line: number[], index: number, array) => {
            const newList:number[][] = state.savedList.concat([line]);
            let counter = state.result;
            if (newList.length === 3) {
                counter += calcContribution(newList[1], newList[0], newList[2])
            }
            if(index===1){
                counter += calcContribution(newList[0], undefined, newList[1])
            }
            if (index === (array.length - 1)) {
                counter += calcContribution(newList[2], newList[1], undefined)
            }
            return <ParsingResult>{ savedList: newList.slice(-2), result: counter }
        },
            <ParsingResult>{ savedList: [], result: 0 }
        )

console.log("Result " + parsedLines.result)