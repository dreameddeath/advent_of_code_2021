use crate::utils::Part;

enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

struct Position {
    horizontal: u32,
    depth: u32,
}

struct PositionWithAim {
    horizontal: u32,
    depth: u32,
    aim: u32,
}

fn parse_line(line: &String) -> Option<Instruction> {
    let parts: Vec<&str> = line.split(" ").collect();
    let value: u32 = parts
        .get(1)
        .and_then(|val_str| val_str.parse::<u32>().ok())?;
    return parts.get(0).map(|dir_str| match dir_str.as_ref() {
        "forward" => Instruction::Forward(value),
        "up" => Instruction::Up(value),
        _ => Instruction::Down(value),
    });
}

fn parse(lines: &Vec<String>) -> Vec<Instruction> {
    return lines.into_iter().filter_map(|l| parse_line(&l)).collect();
}

pub fn puzzle(part: &Part, lines: &Vec<String>) {
    let instructions = parse(lines);

    match part {
        Part::Part1 => {
            let result: Position = instructions.iter().fold(
                Position {
                    horizontal: 0,
                    depth: 0,
                },
                |result, intruction| match intruction {
                    Instruction::Up(value) => Position {
                        depth: result.depth - value,
                        ..result
                    },
                    Instruction::Down(value) => Position {
                        depth: result.depth + value,
                        ..result
                    },
                    Instruction::Forward(value) => Position {
                        horizontal: result.horizontal + value,
                        ..result
                    },
                },
            );

            println!("Result {}", result.horizontal * result.depth)
        }
        Part::Part2 => {
            let result: PositionWithAim = instructions.iter().fold(
                PositionWithAim {
                    horizontal: 0,
                    depth: 0,
                    aim: 0,
                },
                |result, intruction| match intruction {
                    Instruction::Up(value) => PositionWithAim {
                        aim: result.aim - value,
                        ..result
                    },
                    Instruction::Down(value) => PositionWithAim {
                        aim: result.aim + value,
                        ..result
                    },
                    Instruction::Forward(value) => PositionWithAim {
                        horizontal: result.horizontal + value,
                        depth: result.depth + result.aim * value,
                        ..result
                    },
                },
            );

            println!("Result {}", result.horizontal * result.depth)
        }
    }
}
