import { getData, Type } from "../day_utils"
const testData = `[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]`
const lines = getData(10, Type.RUN, testData)


const linesOfChars = lines
    .map(line => line.split(""))
const stack = []
const defs: { [key: string]: { opener: string, points: number, pointsToComplete: number } } = {
    ")": { opener: "(", points: 3, pointsToComplete: 1 },
    "]": { opener: "[", points: 57, pointsToComplete: 2 },
    "}": { opener: "{", points: 1197, pointsToComplete: 3 },
    ">": { opener: "<", points: 25137, pointsToComplete: 4 }
}

const closureDef: { [key: string]: { pointsToComplete: number } } = {
    "(": { pointsToComplete: 1 },
    "[": { pointsToComplete: 2 },
    "{": { pointsToComplete: 3 },
    "<": { pointsToComplete: 4 }
}
type ParsingResult = {
    issueChar?: string,
    stack: string[]
}

type ParsingResultIssues = Required<ParsingResult>

const parsingResults = linesOfChars.map(line =>
    line.reduce((context, current) => {
        if (context.issueChar) return context
        const closingDef = defs[current]
        if (closingDef) {
            if (context.stack.slice(-1)[0] !== closingDef.opener) {
                return <ParsingResult>{ ...context, issueChar: current }
            }
            return <ParsingResult>{ ...context, stack: context.stack.slice(0, -1) }
        }
        return { ...context, stack: context.stack.concat([current]) };
    }, <ParsingResult>{ stack: [] })
)

const issues = parsingResults.filter(it => it.issueChar !== undefined).map(it => it as ParsingResultIssues)

const issuesScore = issues.map(it => defs[it.issueChar].points).reduce((sum, val) => sum + val)
const incompleteLines = parsingResults.filter(it => it.issueChar === undefined)

const incompleteScores = incompleteLines.map(
    line => line.stack.reverse().reduce((lineScore, char) => lineScore * 5 + closureDef[char].pointsToComplete, 0)
)
incompleteScores.sort((a, b) => a - b)

console.log(`Result : ${issuesScore} / ${incompleteScores[Math.floor(incompleteScores.length / 2)]}`)