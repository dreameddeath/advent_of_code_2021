import { generator, Part, run, Type } from "./day_utils"
const testData = `9C0141080250320F1802104A08`

const bitMap: { [key: string]: string } = {
    "0": "0000",
    "1": "0001",
    "2": "0010",
    "3": "0011",
    "4": "0100",
    "5": "0101",
    "6": "0110",
    "7": "0111",
    "8": "1000",
    "9": "1001",
    "A": "1010",
    "B": "1011",
    "C": "1100",
    "D": "1101",
    "E": "1110",
    "F": "1111"
}

type Packet = {
    version: number,
    type: "literal" | "operator",
    typeId: string,
    value: bigint | Packet[]
}

type ParserState = {
    currPos: number,
    value: string
}

function read_next(nb: number, state: ParserState): string {
    const result = state.value.slice(state.currPos, state.currPos + nb)
    if (result.length < nb) {
        throw "NOT_ENOUGHT";
    }
    state.currPos += nb;
    return result;
}

function strToNum(val: string): number {
    return val.split("").reverse().reduce(
        (sum, val, index) => sum + (parseInt(val) * (2 ** index)),
        0)
}

function strToBitInt(val: string): bigint {
    return val.split("").reverse().reduce((sum, val, index) => sum + (BigInt(parseInt(val)) * (2n ** BigInt(index))), 0n)
}
function parsePacket(state: ParserState): Packet {
    const version = strToNum(read_next(3, state))
    const typeId = read_next(3, state);
    if (typeId == "100") {
        let literal_value = "";
        while (true) {
            const content = read_next(5, state);
            literal_value += content.slice(1);
            if (content[0] === "0") {
                break;
            }
        }
        return {
            version,
            type: "literal",
            typeId,
            value: strToBitInt(literal_value)
        }
    }
    else {
        const lengthType = read_next(1, state);
        if (lengthType === "0") {
            const subPacketContentSize = read_next(15, state);
            const length = strToNum(subPacketContentSize);
            const subContent = read_next(length, state);
            const subState: ParserState = { currPos: 0, value: subContent };
            return {
                type: 'operator',
                version,
                typeId,
                value: parsePackets(subState)
            }
        }
        else {
            const nbSubPacketsStr = read_next(11, state)
            const nbSubPacket = strToNum(nbSubPacketsStr);
            const packets = [...generator(nbSubPacket)].map((index) => parsePacket(state));
            return {
                type: 'operator',
                version,
                typeId,
                value: packets
            }
        }
    }
}

function parsePackets(state: ParserState): Packet[] {
    const result = [];
    try {
        while (true) {
            result.push(parsePacket(state))
        }
    }
    catch (e) {

    }
    return result;
}

function parse(line: string): string {
    return line.split("").map(chr => bitMap[chr]).join("")
}

function sum_version(packets: Packet[]): number {
    return packets.reduce((sum, packet) => sum + packet.version + (Array.isArray(packet.value) ? sum_version(packet.value) : 0), 0)
}

enum OPERATORS {
    SUM = "000",
    PRODUCT = "001",
    MIN = "010",
    MAX = "011",
    GT = "101",
    LT = "110",
    EQ = "111"
}

function minBigInt(a:bigint,b:bigint):bigint{
    return a>b?b:a
}


function maxBigInt(a:bigint,b:bigint):bigint{
    return a<b?b:a;
}

function calc(packet: Packet): bigint {
    if (typeof packet.value === "bigint") {
        return packet.value;
    }
    switch (packet.typeId) {
        case OPERATORS.SUM: return packet.value.reduce((sum, packet) => sum + calc(packet), 0n);
        case OPERATORS.PRODUCT: return packet.value.reduce((sum, packet) => sum * calc(packet), 1n);
        case OPERATORS.MIN: return packet.value.reduce((min, packet) => (min === -1n) ? calc(packet) : minBigInt(min, calc(packet)), -1n);
        case OPERATORS.MAX: return packet.value.reduce((max, packet) => (max === -1n) ? calc(packet) : maxBigInt(max, calc(packet)), -1n);
        case OPERATORS.EQ: return calc(packet.value[0]) === calc(packet.value[1]) ? 1n : 0n;
        case OPERATORS.LT: return calc(packet.value[0]) < calc(packet.value[1]) ? 1n : 0n;
        case OPERATORS.GT: return calc(packet.value[0]) > calc(packet.value[1]) ? 1n : 0n;
    }
    throw "NOT_MANAGED";
}

function puzzle(lines: string[], part: Part): void {
    const parsed = parse(lines[0]);
    const packets = parsePackets({ currPos: 0, value: parsed });
    const sum = sum_version(packets)
    const calc_res = calc(packets[0]);
    console.log("Parsed:" + sum +"  calc result "+calc_res);
}

run(16, testData, [Type.TEST, Type.RUN], puzzle, [Part.PART_1, Part.PART_2])