import * as fs from 'fs';
import { type } from 'os';

export enum Type {
    TEST,
    RUN
}

export enum Part {
    ALL = "BOTH",
    PART_1 = "PART 1",
    PART_2 = "PART 2"
}


export function getData(day: number, test: Type, testData: string): string[] {
    const data = (test == Type.TEST) ? testData : fs.readFileSync(`./data/day_${day}_1.dat`, 'utf-8');
    return data.split(/\r?\n/);
}

export function run(day: number, testData: string, types: Type[], fct: (lines: string[], part: Part) => void, parts: Part[] = [Part.ALL]): void {
    parts.forEach(part => {
        types.forEach(type => {
            const name = Type[type];
            console.log(`[${name}][${part}] Running`)
            const start = new Date()
            fct(getData(day, type, testData), part)
            const duration = (new Date()).getTime() - start.getTime()
            console.log(`[${name}][${part}] Done in ${duration} ms`)
        })
    })
}

export function* generator(max: number): Generator<number> {
    let i = 0;
    while (i < max) {
        yield (i++)
    }
}
