import { generator, Part, run, Type } from "../day_utils"
const testData = `..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###`


type Rules = boolean[];

type Image = {
    defaultValue: boolean,
    content: boolean[][]
}

function parseRules(line: string): Rules {
    return line.split("").map(char => char === "#")
}

function parseImage(lines: string[]): Image {
    return {
        content: lines.map(line => line.split("").map(char => char === "#")),
        defaultValue: false
    }
}

function printImage(image: Image, loop: number) {
    //const imageStr = image.content.map(line => line.map(val => val ? "#" : ".").join("")).join("\n")
    //console.log(`After loop ${loop}\n${imageStr}`);
}

function getEnhancedValue(pixels: boolean[], rules: Rules): boolean {
    const rulePos = pixels.reduce((num, bit) => (num * 2) + (bit ? 1 : 0), 0)
    return rules[rulePos];
}

function getBitsToEnhance(x: number, y: number, image: Image): boolean[] {
    const bits = [-1, 0, 1].flatMap(offsetY =>
        [-1, 0, 1].flatMap(offsetX => {
            const line = image.content[offsetY + y] ?? []
            return line[offsetX + x] ?? image.defaultValue
        })
    )
    return bits
}

const extentionSize = 1;
const fullMatrixArray = [...generator(9)];

function applyLoop(srcImage: Image, rules: Rules): Image {
    //const image = extendImage(srcImage);
    const newWidth = srcImage.content[0].length + 2 * extentionSize;
    const newHeight = srcImage.content.length + 2 * extentionSize;
    const allX = [...generator(newWidth)];
    const allY = [...generator(newHeight)];


    const content = allY.map(y => allX.map(x => getEnhancedValue(getBitsToEnhance(x - extentionSize, y - extentionSize, srcImage), rules)));
    const defaultValue = getEnhancedValue(fullMatrixArray.map(_ => srcImage.defaultValue), rules);
    return {
        content,
        defaultValue
    };
}

function applyLoops(src: Image, loops: number, rules: Rules): Image {
    printImage(src, 0);
    return [...generator(loops)].reduce((image, loop) => {
        const newImage = applyLoop(image, rules);
        printImage(newImage, loop + 1);
        return newImage;
    }, src)
}

function calcNbLit(srcImage: Image): number {
    return srcImage.content.reduce((total, line) => line.reduce((subTotal, val) => subTotal + (val ? 1 : 0), total), 0)
}


function puzzle(lines: string[], part: Part): void {
    const rules = parseRules(lines[0]);
    const startImage = parseImage(lines.slice(2))
    if (part === Part.PART_1) {
        const finalImage = applyLoops(startImage, 2, rules);
        const nbLit = calcNbLit(finalImage);
        console.log(`Results ${nbLit}`);
    }
    else {
        const finalImage = applyLoops(startImage, 50, rules);
        const nbLit = calcNbLit(finalImage);
        console.log(`Results ${nbLit}`);

    }
}

run(20, testData, [Type.TEST, Type.RUN], puzzle, [Part.PART_1, Part.PART_2])