import * as fs from 'fs';
import { type } from 'os';

export enum Type {
    TEST,
    RUN
}

export function getData(day: number, test: Type, testData: string): string[] {
    const data = (test == Type.TEST) ? testData : fs.readFileSync(`./data/day_${day}_1.dat`, 'utf-8');
    return data.split(/\r?\n/);
}

export function run(day: number, testData: string, types: Type[], fct: (lines: string[]) => void): void {
    types.forEach(type => {
        const name = Type[type];
        console.log(`Running ${name}`)
        const start = new Date()
        fct(getData(day, type, testData))
        const duration = (new Date()).getTime() - start.getTime()
        console.log(`Done ${name} in ${duration} ms`)
    })
}

export function* generator(max: number): Generator<number> {
    let i = 0;
    while (i < max) {
        yield (i++)
    }
}
