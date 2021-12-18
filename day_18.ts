import { assert } from "console"
import exp from "constants"
import { Part, run, Type } from "./day_utils"
const testData = `[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]`

const token_regexp = /\[|\]|\d+/g

type Expr = number | [Expr, Expr]

function parse_line(line: string): Expr {
    const stack: ({ val: number } | Expr)[][] = []
    for (const token of line.matchAll(token_regexp)) {
        switch (token[0]) {
            case "[": stack.push([]); break;
            case "]": {
                if (stack.length === 1) break;
                const item = stack.pop();
                const parent = stack[stack.length - 1];
                parent.push(item as Expr);
                break;
            }
            default: {
                stack[stack.length - 1].push(parseInt(token[0]))
            }
        }
    }
    return stack[0] as Expr;
}


function parse(lines: string[]): Expr[] {
    return lines.map(line => parse_line(line))
}

type ActionLeftAdd = {
    action: "left",
    value: number,
    done: boolean,
}

type ActionRightAdd = {
    action: "right",
    value: number,
    done: boolean,
}

type ActionSplit = {
    action: "split",
    done: true;
}

type Action = ActionLeftAdd | ActionRightAdd | ActionSplit

type ReduceState = {
    actions: Action[],
    itemPos: number,
    items: Expr[]
}

function applyAction(act: Action, item: Expr): Expr {
    if (act.done) {
        return item;
    }
    if (typeof item === "number") {
        act.done = true
        return item + act.value;

    }
    else if (act.action === "right") {
        const newLeft = applyAction(act, item[0]);
        const newRight = applyAction(act, item[1]);
        return [newLeft, newRight];
    }
    else {
        const newRight = applyAction(act, item[1]);
        const newLeft = applyAction(act, item[0]);
        return [newLeft, newRight];
    }
}

function applyActions(actions: Action[], origExpr: Expr, type: "left" | "right"): Expr {
    return actions.filter(act => act.action === type).reduce((modifiedExpr, act) => applyAction(act, modifiedExpr), origExpr);
}


function explode(expr: Expr, depth: number, actionsAlreadyMade: boolean): [Action[], Expr] {
    if (actionsAlreadyMade || typeof expr === "number") {
        return [[], expr]
    }

    let [actionsFromLeft, left] = explode(expr[0], depth + 1, actionsAlreadyMade);
    let [actionsFromRight, right] = explode(expr[1], depth + 1, actionsFromLeft.length > 0);
    if (actionsFromLeft.length > 0) {
        right = applyActions(actionsFromLeft, right, "right");
    }
    if (actionsFromRight.length > 0) {
        left = applyActions(actionsFromRight, left, "left")
    }
    const allActions = actionsFromLeft.concat(actionsFromRight);
    if (allActions.length > 0) {
        return [allActions, [left, right]];
    }
    else if (typeof left === "number" && typeof right === "number" && depth == 4) {
        return [
            [
                { action: 'left', value: left, done: false },
                { action: 'right', value: right, done: false }
            ],
            0
        ]
    }
    else {
        return [[], [left, right]];
    }
}

function split(expr: Expr, operationAlreadyMade: boolean): [boolean, Expr] {
    if (operationAlreadyMade) {
        return [operationAlreadyMade, expr]
    }
    if (typeof expr === "number") {
        if (expr < 10) {
            return [operationAlreadyMade, expr];
        }
        return [
            true,
            [Math.floor(expr / 2), Math.ceil(expr / 2)]
        ]
    }
    else {
        const [splitAlreadyMadeInLeft, left] = split(expr[0], operationAlreadyMade);
        const [splitAlreadyMadeInRight, right] = split(expr[1], splitAlreadyMadeInLeft);
        return [splitAlreadyMadeInRight, [left, right]]
    }
}

function reduce_all(expr: Expr): Expr {
    let currExpr = expr
    while (true) {
        const [actions, afterExplode] = explode(currExpr, 0, false);
        const [operationDone, afterSplit] = split(afterExplode, actions.length > 0)
        currExpr = afterSplit;
        if (!operationDone) {
            break;
        }
    }

    return currExpr;
}

function sum(exprA: Expr, exprB: Expr): Expr {
    return reduce_all([exprA, exprB])
}

function magnitude(expr: Expr): number {
    if (typeof expr === "number") {
        return expr;
    }
    else {
        return magnitude(expr[0]) * 3 + magnitude(expr[1]) * 2;
    }
}

function print_exprs(exprs: Expr[], expected: string | undefined = undefined) {
    const parsed = exprs.map(expr => JSON.stringify(expr)).join("\n");
    console.log(`Expressions ${parsed}` + ((expected) ? ' for expected ' + expected : ""));
}

function test_reduce_all(source: string, expected: string): void {
    const test = parse_line(source)
    const result = reduce_all(test);
    print_exprs([test, result], expected);
    const stringResult = JSON.stringify(result);
    assert(stringResult === expected);
}

function test_explode(source: string, expected: string): void {
    const test = parse_line(source)
    const [_, result] = explode(test, 0, false);
    print_exprs([test, result], expected);
    const stringResult = JSON.stringify(result);
    assert(stringResult === expected);
}

function test_sum(sourceA: string, sourceB: string, expected: string) {
    const test = parse_line(sourceA);
    const testB = parse_line(sourceB);
    const result = sum(test, testB);
    print_exprs([test, result], expected);
    const stringResult = JSON.stringify(result);
    assert(stringResult === expected);
}

function test_engine(): void {
    test_reduce_all("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
    test_reduce_all("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
    test_reduce_all("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
    test_explode("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
    test_explode("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    test_sum("[[[[4,3],4],4],[7,[[8,4],9]]]", "[1,1]", "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

    test_sum("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]", "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]", "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]")
    test_sum("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
        , "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]", "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]")
}

function puzzle(lines: string[], part: Part): void {
    //test_engine();
    const exprs = parse(lines);

    if (part === Part.PART_1) {
        const reducedSum = exprs.reduce(
            (added, item) => sum(added, item)
        );
        const magnitudeRes = magnitude(reducedSum);

        console.log(`Results ${magnitudeRes}`);
    }
    else {
        const maxMag = exprs
            .flatMap(expr =>
                exprs.flatMap(expr2 => (expr2 === expr) ? [] : [[expr, expr2], [expr2, expr]])
            )
            .map(part => sum(part[0], part[1]))
            .reduce((currMax: number, sumExpr) => Math.max(currMax, magnitude(sumExpr)), 0);
        console.log(`Results ${maxMag}`);

    }
}

run(18, testData, [Type.TEST, Type.RUN], puzzle, [Part.PART_1, Part.PART_2])