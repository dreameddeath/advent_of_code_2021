import * as fs from 'fs';

type BoardItem = { value: number, row: number, col: number }
type BoardDef = {
    content: BoardItem[],
    nbRows: number,
    totalResult: number
}

type CurrResultBoardCheck = {
    rows: number[],
    cols: number[],
    score: number,
    successRank?: number
}



const data = fs.readFileSync('./data/day_4_1.dat', 'utf-8');


type WiningResult = {
    winRank: number,
    winResult: number
}

type ParsingResult = {
    pickedValues: number[],
    currBoard?: BoardDef,
    winner?: WiningResult
}

function checkBoard(currBoard: BoardDef | undefined, pickedValues: number[]): WiningResult | undefined {
    if (currBoard === undefined) {
        return undefined;
    }
    const nbCols = Math.floor(currBoard.content.length / currBoard.nbRows);
    const nbRows = currBoard.nbRows;
    const checkResult = pickedValues.reduce((currState, pickedValue, index) => {
        if (currState.successRank !== undefined) {
            return currState
        }
        const match = currBoard.content.find(item => item.value === pickedValue)
        if (match === undefined) {
            return currState
        }
        currState.cols[match.col] = (currState.cols[match.col] ?? 0) + 1
        currState.rows[match.row] = (currState.rows[match.row] ?? 0) + 1
        currState.score -= pickedValue
        if (currState.cols[match.col] === nbCols || currState.rows[match.row] == nbRows) {
            currState.successRank = index
            currState.score *= pickedValue
        }
        return currState
    }, <CurrResultBoardCheck>{ rows: [], cols: [], score: currBoard.totalResult })
    if (checkResult.successRank === undefined) {
        return undefined
    }
    return { winRank: checkResult.successRank, winResult: checkResult.score }
}

function updateResult(result: ParsingResult): ParsingResult {
    const { currBoard, ...newResult } = result;
    const checkRes = checkBoard(currBoard, result.pickedValues)
    if (checkRes === undefined) {
        return newResult;
    }
    if (result.winner === undefined || result.winner.winRank < checkRes.winRank) {
        return { ...newResult, winner: checkRes }
    }
    return newResult
}

const parsedLines: ParsingResult = data.split(/\r?\n/)
    .reduce((result: ParsingResult, line: string, index: number, array) => {
        if (index === 0) {
            return { ...result, pickedValues: line.split(/,/).map(item => parseInt(item)) }
        }
        if (line.trim().length === 0) {
            return updateResult(result)
        }
        const currBoard: BoardDef = result.currBoard ?? { content: [], nbRows: 0, totalResult: 0 }
        const newLine = line.trim().split(/\s+/).map((val, index) => <BoardItem>{ value: parseInt(val), row: currBoard.nbRows, col: index });
        const newBoard: BoardDef = {
            ...currBoard,
            content: currBoard.content.concat(newLine),
            nbRows: currBoard.nbRows + 1,
            totalResult: currBoard.totalResult + newLine.reduce((sum, it) => sum + it.value, 0)
        }
        const newResult = {
            ...result,
            currBoard: newBoard
        }
        if (index === (array.length - 1)) {
            return updateResult(newResult)
        }
        return newResult;
    },
        <ParsingResult>{ pickedValues: [] }
    )

console.log("Result " + parsedLines.winner?.winResult)