import { Part, run, Type } from "../day_utils"
const testData = `Player 1 starting position: 4
Player 2 starting position: 8`

const parser = /^.*(\d+)$/;

function parse(lines: string[]): [number, number] {
    const starts = lines.map(line => line.match(parser)).map(result => parseInt(result?.[1] ?? "0"));
    return [starts[0], starts[1]]
}


type State = {
    pos: number;
    score: number
}

type NextRes = {
    next_score: number,
    next_pos: number
}

function calc_next_state(currState: State, move_by: number): State {
    const next_pos = (currState.pos + move_by) % 10
    return {
        pos: next_pos,
        score: currState.score + next_pos + 1
    }
}


function play_player(state: State, curr_roll: number): number {
    curr_roll += 3;
    const nextStep = calc_next_state(state, curr_roll * 3 - 2 - 1)
    state.pos = nextStep.pos;
    state.score = nextStep.score;
    return curr_roll;
}

function play(starts: [number, number]): [State, number] {
    let current: [State, State] = [{ pos: starts[0] - 1, score: 0 }, { pos: starts[1] - 1, score: 0 }];
    let current_dice_roll = 0
    while (true) {
        current_dice_roll = play_player(current[0], current_dice_roll);
        if (current[0].score >= 1000) {
            return [current[1], current_dice_roll];
        }
        current_dice_roll = play_player(current[1], current_dice_roll);
        if (current[1].score >= 1000) {
            return [current[0], current_dice_roll];
        }
    }
}

const possible_dice = [1, 2, 3]
const possible_dice_rolls = possible_dice.flatMap(val1 => possible_dice.flatMap(val2 => possible_dice.map(val3 => [val1, val2, val3] as [number, number, number])))
const possible_dice_rolls_counts = possible_dice_rolls.reduce((counters: bigint[], dice_roll) => {
    const sum = dice_roll[0] + dice_roll[1] + dice_roll[2];
    counters[sum] = (counters[sum] ?? 0n) + 1n;
    return counters
}, []);
type Player = "player1" | "player2"

type MultiverseStatePlayingState = {
    player1_state: State,
    player2_state: State,
}

type MultiversePlayingState = {
    [key: string]: [MultiverseStatePlayingState, bigint]
}

type WiningResults = {
    "player1": bigint,
    "player2": bigint
}


function addToMultiverse(universe: MultiversePlayingState, state: MultiverseStatePlayingState, count: bigint) {
    const key = JSON.stringify(state);
    if (universe[key] === undefined) {
        universe[key] = [state, count];
    }
    else {
        universe[key][1] += count;
    }
}

function calculate_winining_result_multiverse(player1Pos: number, player2Pos: number): WiningResults {
    let currMultiverseState: MultiversePlayingState = {};

    addToMultiverse(currMultiverseState, {
        player1_state: {
            pos: player1Pos - 1,
            score: 0
        },
        player2_state: {
            pos: player2Pos - 1,
            score: 0
        },
    }, 1n);

    const winingResults: WiningResults = {
        "player1": 0n,
        "player2": 0n
    }
    let nextPlayer: Player = "player1";
    let maxMultiverseSize = 0;
    while (Object.keys(currMultiverseState).length > 0) {
        const newMultiverseState: MultiversePlayingState = {}
        Object.values(currMultiverseState).forEach(old_state => {
            const effectivePlayerState = nextPlayer === "player1" ? old_state[0].player1_state : old_state[0].player2_state;
            possible_dice_rolls_counts.forEach((counter, index) => {
                if (counter === undefined) return;
                const nb_universes = old_state[1] * counter;
                const next_player_state = calc_next_state(effectivePlayerState, index);
                if (next_player_state.score >= 21) {
                    winingResults[nextPlayer] += nb_universes;
                }
                else {
                    if (nextPlayer === "player1") {
                        addToMultiverse(newMultiverseState, {
                            player1_state: next_player_state,
                            player2_state: old_state[0].player2_state,
                        }, nb_universes)
                    }
                    else {
                        addToMultiverse(newMultiverseState, {
                            player1_state: old_state[0].player1_state,
                            player2_state: next_player_state,
                        }, nb_universes)
                    }
                }
            })
        }
        )
        currMultiverseState = newMultiverseState;
        maxMultiverseSize = Math.max(maxMultiverseSize, Object.keys(currMultiverseState).length)
        nextPlayer = nextPlayer === "player1" ? "player2" : "player1"
    }
    console.log("Max multiverse size " + maxMultiverseSize);
    return winingResults;
}

function puzzle(lines: string[], part: Part): void {
    const parseRes = parse(lines);
    if (part === Part.PART_1) {
        const play_res = play(parseRes);
        const result = play_res[0].score * play_res[1];
        console.log(`Results ${result}`);
    }
    else {
        const { player1, player2 } = calculate_winining_result_multiverse(parseRes[0], parseRes[1]);
        console.log(`Results P1 ${player1} / P2 ${player2}`);
    }
}

run(21, testData, [Type.TEST, Type.RUN], puzzle, [Part.PART_1, Part.PART_2])