use crate::utils::{read_lines, Dataset, Part};

type ParseResult = u16;
pub fn parse_line(line: &String) -> Option<ParseResult> {
    let parse_res = line.parse::<u16>();
    return match parse_res {
        Ok(val) => Some(val),
        Err(_) => None,
    };
}

pub fn parse(data_set: &Dataset) -> Vec<u16> {
    let l = read_lines(1, data_set);
    return match l {
        Some(lines) => lines
            .filter_map(|l| match l {
                Ok(line) => parse_line(&line),
                Err(_) => None,
            })
            .collect(),
        None => vec![],
    };
}

pub fn puzzle(part: &Part, data_set: &Dataset) {
    let values = parse(data_set);

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
