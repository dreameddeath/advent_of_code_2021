use crate::utils::{Part};

pub fn parse(lines:&Vec<String>) -> Vec<u16> {
    return lines.into_iter()
                .filter_map(|l| l.parse::<u16>().ok())
                .collect()
        
}

pub fn puzzle(part: &Part, lines: &Vec<String>) {
    let values = parse(lines);

    match part {
        Part::Part1 => {
            let count = values
                .into_boxed_slice()
                .windows(2)
                .filter(|list| list[0] < list[1])
                .count();

            println!("Result {}", count)
        }
        Part::Part2 => {
            let count = values
                .into_boxed_slice()
                .windows(3)
                .map(|list| list[0] + list[1] + list[2])
                .collect::<Vec<u16>>()
                .into_boxed_slice()
                .windows(2)
                .filter(|list| list[0] < list[1])
                .count();
            println!("Result {}", count)
        }
    }
}
