import { generator, Part, run, Type } from "../day_utils"
const testData = `6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5`


type Paper = {
    dots: boolean[][] //y then x
    maxX: number
    maxY: number,
}

type Fold = {
    type: "x" | "y",
    position: number
}
const FoldRegex = /^fold along (x|y)=(\d+)$/

function addDot(paper: Paper, pos: { x: number, y: number }): Paper {
    if (paper.dots[pos.y] === undefined) {
        paper.dots[pos.y] = []
    }
    paper.dots[pos.y][pos.x] = true
    paper.maxX = Math.max(paper.maxX, pos.x)
    paper.maxY = Math.max(paper.maxY, pos.y)
    return paper
}

function parse(lines: string[]): { paper: Paper, folds: Fold[] } {
    return lines
        .filter(line => line.trim().length !== 0)
        .reduce((state, line) => {
            const parsingResult = line.match(FoldRegex)
            if (parsingResult !== null && parsingResult.length > 2) {
                return {
                    ...state,
                    folds: state.folds.concat([
                        {
                            type: parsingResult[1] as "x" | "y",
                            position: parseInt(parsingResult[2])
                        }
                    ])
                }
            }
            else {
                const values = line.split(",").map(val => parseInt(val))
                return {
                    ...state,
                    paper: addDot(state.paper, { x: values[0], y: values[1] })
                }
            }
        }
            , <{ paper: Paper, folds: Fold[] }>{ paper: { dots: [], maxX: 0, maxY: 0 }, folds: [] })

}

function target(x: number, y: number, fold: Fold): { x: number, y: number } | undefined {
    if ((fold.type === "x" && x <= fold.position) || (fold.type === "y" && y <= fold.position)) {
        return { x, y }
    }
    if (fold.type === "x") {
        const newX = 2 * fold.position - x;
        if (newX < 0) {
            return undefined
        }
        return { x: newX, y }
    }
    else {
        const newY = 2 * fold.position - y;
        if (newY < 0) {
            return undefined
        }
        return { x, y: newY }
    }
}

function foldPaper(paper: Paper, fold: Fold): Paper {
    const foldedPaper: Paper = {
        dots: [],
        maxX: (fold.type === "x") ? fold.position - 1 : paper.maxX,
        maxY: (fold.type === "y") ? fold.position - 1 : paper.maxY,
    }

    paper.dots.forEach(
        (line, y) =>
            (line ?? []).forEach((dot, x) => {
                const targetPos = target(x, y, fold)
                if (dot === true && targetPos !== undefined) addDot(foldedPaper, targetPos)
            })
    )

    return foldedPaper
}

function printPaper(paper: Paper) {
    console.log(
        [...generator(paper.maxY + 1)].map(y =>
            [...generator(paper.maxX + 1)].map(x => ((paper.dots[y]?.[x]) ?? false) ? "#" : " ").join("")
        ).join("\n")
    )
}

function puzzle(lines: string[], part: Part): void {
    const { paper, folds } = parse(lines)
    if (part === Part.PART_1) {
        const folded = foldPaper(paper, folds[0])
        //printPaper(folded)
        const nbDots = folded.dots.reduce((sumTotal, line) => line.reduce((sumLine, val) => sumLine + 1, 0) + sumTotal, 0)
        console.log(`Dots: ${nbDots}`)
    }
    else {
        const foldedPaper = folds.reduce((newPaper, fold) => foldPaper(newPaper, fold), paper)
        printPaper(foldedPaper)
    }
}

run(13, testData, [Type.TEST, Type.RUN], puzzle, [Part.PART_1, Part.PART_2])
