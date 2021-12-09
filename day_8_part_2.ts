import * as fs from 'fs';

const data = fs.readFileSync('./data/day_8_1.dat', 'utf-8');

const lines = data.split(/\r?\n/);
type Segments = "a" | "b" | "c" | "d" | "e" | "f" | "g"
type Digit = Set<Segments>
type Line = {
    inputs: Digit[],
    outputs: Digit[]
}

function stringToDigit(digitStr: string): Digit {
    return digitStr.split("").reduce((digit, char) => digit.add(char as Segments), new Set<Segments>())
}

function digitToString(digit: Digit): string {
    return [...digit.values()].sort().join("")
}
const parts = lines.map(line => line.split("|")).map(([source, output]) => <Line>{
    inputs: source.trim().split(/\s+/).map(it => stringToDigit(it)),
    outputs: output.trim().split(/\s+/).map(it => stringToDigit(it))
});


type DigitMap = { [key: string]: number }
const refList = {
    0: new Set(["a", "b", "c", "e", "f", "g"]),
    1: new Set(["c", "f"]),
    2: new Set(["a", "c", "d", "e", "g"]),
    3: new Set(["a", "c", "d", "f", "g"])
}

function contains(digit: Digit, ref: Digit): boolean {
    return [...ref.values()].every(seg => digit.has(seg))
}

function areSame(digit: Digit, ref: Digit): boolean {
    return digitToString(ref) === digitToString(digit)
}

function remove(digit: Digit, ref: Digit): Digit {
    const result = new Set(digit)
    ref.forEach(val => result.delete(val))
    return result;
}

function getMapping(inputs: Digit[], lineNumber: number): DigitMap {
    const digitMap: DigitMap = {}
    const numToDigit: { [key: number]: Digit } = {}
    inputs.forEach(digit => {
        const found = [[2, 1], [3, 7], [4, 4], [7, 8]].find(size => size[0] === digit.size)
        if (found) {
            digitMap[digitToString(digit)] = found[1]
            numToDigit[found[1]] = digit
        }
    })
    //9
    const found9s = inputs.filter(digit => digit.size === 6 && contains(digit, numToDigit[4]))
    const found9 = found9s[0]
    digitMap[digitToString(found9)] = 9
    numToDigit[9] = found9
    //6
    const found6s = inputs.filter(digit => digit.size === 6 && !areSame(found9, digit) && !contains(digit, numToDigit[1]))
    const found6 = found6s[0]
    digitMap[digitToString(found6)] = 6
    numToDigit[6] = found6
    //0
    const found0s = inputs.filter(digit => digit.size === 6 && !areSame(found9, digit) && contains(digit, numToDigit[1]))
    const found0 = found0s[0]
    digitMap[digitToString(found0)] = 0
    numToDigit[0] = found0
    //3
    const found3s = inputs.filter(digit => digit.size === 5 && contains(digit, numToDigit[1]))
    const found3 = found3s[0]
    digitMap[digitToString(found3)] = 3
    numToDigit[3] = found3

    //2
    const segmentC = remove(numToDigit[8], found6)
    const segmentE = remove(numToDigit[8], found9)
    const found2s = inputs.filter(digit => digit.size === 5 && !areSame(found3, digit) && contains(digit, segmentC) && contains(digit, segmentE))
    const found2 = found2s[0]
    digitMap[digitToString(found2)] = 2
    numToDigit[2] = found2

    //5
    const found5s = inputs.filter(digit => digit.size === 5 && !areSame(found3, digit) && !areSame(found2, digit))
    const found5 = found5s[0]
    digitMap[digitToString(found5)] = 5
    numToDigit[5] = found5

    return digitMap
}

const result = parts.map((line, index) => {
    const map = getMapping(line.inputs, index);
    const result = line.outputs.map(digit => map[digitToString(digit)]).join("")
    return parseInt(result)
})

const total = result.reduce((sum, val) => sum + val)
console.log(`Result ${total} : ${result} `)