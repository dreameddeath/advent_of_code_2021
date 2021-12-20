
import { Part, run, Type } from "./day_utils"
const testData = `--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14`

type Vector = [number, number, number]

type RotationMatrix = [Vector, Vector, Vector];

const angles = [0, 1/*90°*/, 2/*180°*/, 3/*270°*/];
const trigoForAngle: [number /*cos*/, number /*sin*/][] = [[1, 0], [0, 1], [-1, 0], [0, -1]];
function xRotation(angle: number): RotationMatrix {
    const [cosA, sinA] = trigoForAngle[angle];
    return [
        [1, 0, 0],
        [0, cosA, -sinA],
        [0, sinA, cosA]
    ];
}

function yRotation(angle: number): RotationMatrix {
    const [cosA, sinA] = trigoForAngle[angle];
    return [
        [cosA, 0, sinA],
        [0, 1, 0],
        [-sinA, 0, cosA]
    ]
}

function zRotation(angle: number): RotationMatrix {
    const [cosA, sinA] = trigoForAngle[angle];
    return [
        [cosA, -sinA, 0],
        [sinA, cosA, 0],
        [0, 0, 1]
    ]
}

function multiplyMatrices(matrixA: RotationMatrix, matrixB: RotationMatrix): RotationMatrix {
    const mulResult = [0, 1, 2].map(line =>
        [0, 1, 2].map(
            col => matrixA[line]
                .reduce((sum, val, index) => sum + val * matrixB[col][index], 0)
        ) as Vector
    ) as [Vector, Vector, Vector];
    return mulResult
}


function deduplicate(matrices: RotationMatrix[]): RotationMatrix[] {
    const map: { [key: string]: RotationMatrix } = {};
    matrices.forEach(matrice => map[JSON.stringify(matrice)] = matrice);
    return Object.values(map);
}
const rotationMatrices: RotationMatrix[] = deduplicate(
    angles.flatMap(x =>
        angles.flatMap(y =>
            angles.map(z =>
                multiplyMatrices(multiplyMatrices(xRotation(x), yRotation(y)), zRotation(z))
            )
        )
    )
)

type BeaconTuple = {
    posA: number,
    pointA: Vector,
    posB: number,
    pointB: Vector,
    vectorAB: Vector,
    vectorBA: Vector
}

type ProbeData = {
    name: string,
    beaconsSet: { [key: string]: Vector }
    distanceMap: { [key: string]: BeaconTuple[] }
}

const X = 0;
const Y = 1;
const Z = 2;

function calc_vector(pointA: Vector, pointB: Vector): Vector {
    return [X, Y, Z].map(coord => pointA[coord] - pointB[coord]) as Vector
}

function manhattanDistance(a: Vector, b: Vector): number {
    return [X, Y, Z].reduce((sum, coord) => sum + Math.abs(a[coord] - b[coord]), 0);
}


function finalizeData(data: ProbeData): ProbeData {
    const resultMap: { [key: string]: boolean } = {};
    const beacons = Object.values(data.beaconsSet);
    data.distanceMap = {};
    beacons.forEach(
        (beacon1, index1) => {
            beacons.forEach((beacon2, index2) => {
                const key = `${index1}:${index2}`;
                const key2 = `${index2}:${index1}`;
                if (index1 != index2 && !resultMap[key]) {
                    const distance = manhattanDistance(beacon1, beacon2) + "";
                    if (data.distanceMap[distance] === undefined) { data.distanceMap[distance] = [] };
                    data.distanceMap[distance].push({
                        posA: index1,
                        posB: index2,
                        pointA: beacon1,
                        pointB: beacon2,
                        vectorAB: calc_vector(beacon1, beacon2),
                        vectorBA: calc_vector(beacon2, beacon1),
                    });
                    resultMap[key] = true;
                    resultMap[key2] = true
                }
            })
        })
    return data;
}

function addToBeaconSet(probe: ProbeData, beacon: Vector) {
    probe.beaconsSet[JSON.stringify(beacon)] = beacon;
}

function parse_lines(lines: string[]): ProbeData[] {
    return lines.reduce((probes, line) => {
        if (line.startsWith("---")) { return probes.concat({ name: line, distanceMap: {}, beaconsSet: {} }) }
        if (line.trim().length === 0) { return probes }
        const beacon = line.split(",").map(val => parseInt(val)) as Vector;
        addToBeaconSet(probes[probes.length - 1], beacon);
        return probes
    }, [] as ProbeData[])
        .map(probeData => finalizeData(probeData))
}

function applyRotation(rotation: RotationMatrix, vector: Vector): Vector {
    return [0, 1, 2].map(line => rotation[line].reduce((sum, val, index) => sum + vector[index] * val, 0)) as Vector;
}

function applyOffset(origVector: Vector, offsetVector: Vector): Vector {
    return [origVector[0] + offsetVector[0], origVector[1] + offsetVector[1], origVector[2] + offsetVector[2]]
}

function vectorEqual(vectorA: Vector, vectorB: Vector): boolean {
    return vectorA.every((val, index) => vectorB[index] === val);
}

type MatchingTuple = {
    tupleA: BeaconTuple,
    tupleB: BeaconTuple,
    rotationB: RotationMatrix,
    isAB: boolean,
    commonVector: Vector
}

function findMatchingRotation(rotationB: RotationMatrix, tupleA: BeaconTuple, tupleB: BeaconTuple): MatchingTuple[] {
    if (vectorEqual(tupleA.vectorAB, applyRotation(rotationB, tupleB.vectorAB))) {
        return [{ tupleA, tupleB, rotationB, isAB: true, commonVector: tupleA.vectorAB }]
    }
    else if (vectorEqual(tupleA.vectorAB, applyRotation(rotationB, tupleB.vectorBA))) {
        return [{ tupleA, tupleB, rotationB, isAB: false, commonVector: tupleA.vectorAB }]
    }
    else { return [] }
}

function findMatchingRotations(tupleA: BeaconTuple, tupleB: BeaconTuple): MatchingTuple[] {
    return rotationMatrices.flatMap(rotation => findMatchingRotation(rotation, tupleA, tupleB));
}


function findMatchingVectors(probe1: ProbeData, probe2: ProbeData, distance: string): MatchingTuple[] {
    const probe1TupleData = probe1.distanceMap[distance];
    const probe2TupleData = probe2.distanceMap[distance];
    return probe1TupleData.flatMap(tuple1 => probe2TupleData.flatMap(tuple2 => findMatchingRotations(tuple1, tuple2)));
}
type Transform = {
    rotation: RotationMatrix,
    offset: Vector
}

function applyTransform(vector: Vector, transform: Transform): Vector {
    return applyOffset(applyRotation(transform.rotation, vector), transform.offset)
}

function findMatchingTransforms(matchingTuple: MatchingTuple[]): Transform[] {
    const map: { [key: string]: Transform } = {};

    matchingTuple.forEach((tuple) => {
        const sourcePointTupleB = tuple.isAB ? tuple.tupleB.pointA : tuple.tupleB.pointB;
        const rotatedBPoint = applyRotation(tuple.rotationB, sourcePointTupleB);
        const transform: Transform = {
            rotation: tuple.rotationB,
            offset: calc_vector(tuple.tupleA.pointA, rotatedBPoint)
        }
        map[JSON.stringify(transform)] = transform;
    });
    return Object.values(map);
}

function isTranformApplicable(probe1: ProbeData, probe2: ProbeData, transform: Transform): boolean {
    const commonBeacons = Object.values(probe2.beaconsSet).filter(beacon => probe1.beaconsSet[JSON.stringify(applyTransform(beacon, transform))] !== undefined);
    return commonBeacons.length >= 12;
}

function getApplicableTransform(probe1: ProbeData, probe2: ProbeData): Transform | undefined {
    const listSameDistances = Object.keys(probe1.distanceMap).filter(dist => probe2.distanceMap[dist] && findMatchingVectors(probe1, probe2, dist));
    if (listSameDistances.length < 12) {
        return undefined;
    }
    console.log(`Finding common distance ${listSameDistances.length} between ${probe1.name} and ${probe2.name}`);
    const allPotentialMatchingTuples = listSameDistances.flatMap(distance => findMatchingVectors(probe1, probe2, distance));
    console.log(`Finding common tuple vectors ${allPotentialMatchingTuples.length} between ${probe1.name} and ${probe2.name}`);
    const allMatchingTransform = findMatchingTransforms(allPotentialMatchingTuples);
    console.log(`Finding potential transforms ${allMatchingTransform.length} between ${probe1.name} and ${probe2.name}`);
    const allApplicableTransform = allMatchingTransform.filter(transform => isTranformApplicable(probe1, probe2, transform));
    console.log(`Finding all applicable transforms ${allApplicableTransform.length} between ${probe1.name} and ${probe2.name}`);

    return allApplicableTransform[0];
}

type ScannersPos = {
    [key: string]: Vector
}

function mergeProbeDatas(probes: ProbeData[]): [ProbeData, ScannersPos] {
    const finalProbe = probes.shift() as ProbeData;
    const scannersPos: ScannersPos = {}
    scannersPos[finalProbe.name] = [0, 0, 0];
    while (probes.length > 0) {
        const probeToMerge = probes.shift() as ProbeData;
        const match = getApplicableTransform(finalProbe, probeToMerge);
        if (!match) {
            probes.push(probeToMerge)
        }
        else {
            scannersPos[probeToMerge.name] = match.offset;
            const beaconsToMerge = Object.values(probeToMerge.beaconsSet).map(beacon => applyTransform(beacon, match));
            beaconsToMerge.forEach(beaconToMerge => {
                addToBeaconSet(finalProbe, beaconToMerge)
            })
            finalizeData(finalProbe)
        }
    }
    return [finalProbe, scannersPos];
}

function calcMaxDistance(scannersPos: ScannersPos): number {
    const scannerPosCoords = Object.values(scannersPos);

    return scannerPosCoords
        .flatMap(
            posA => scannerPosCoords.map(
                posB => manhattanDistance(posA, posB)
            )
        )
        .reduce((min, val) => Math.max(min, val))
}

function puzzle(lines: string[], part: Part): void {
    const probesData = parse_lines(lines);
    const [mergeProbeData, scannerPos] = mergeProbeDatas(probesData);
    if (part === Part.PART_1) {
        const uniqueValues = Object.keys(mergeProbeData.beaconsSet).length
        console.log(`Results ${uniqueValues}`);
    }
    else {
        const maxDistance=calcMaxDistance(scannerPos);
        console.log(`Results ${maxDistance}`);

    }
}

run(19, testData, [Type.TEST, Type.RUN], puzzle, [Part.PART_1, Part.PART_2])