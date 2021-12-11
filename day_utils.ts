import * as fs from 'fs';


export function getData(day: number, test: boolean,testData:string): string[] {
    const data = test?testData:fs.readFileSync(`./data/day_${day}_1${test ? "test" : ""}.dat`, 'utf-8');
    return data.split(/\r?\n/);
}
