use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, Lines};
use std::path::Path;
use std::time::Instant;
fn read_lines_internal<P>(filename: P) -> Result<Lines<BufReader<File>>, Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_lines(day: &u8, is_test: &Dataset) -> Option<Lines<BufReader<File>>> {
    let f = read_lines_internal(format!(
        "./data/day_{}{}.txt",
        day,
        match is_test {
            Dataset::Test => "_test",
            _ => "",
        }
    ));

    return match f {
        Ok(lines) => Some(lines),
        Err(_) => {
            println!("No file found");
            None
        }
    };
}

#[derive(Debug)]
pub enum Part {
    Part1,
    Part2,
}

#[derive(Debug)]
pub enum Dataset {
    Test,
    Real,
}

#[allow(dead_code)]
pub enum Active {
    True,
    False,
}

pub fn run<F: Fn(&Part, &Vec<String>)>(day: &u8,fct: &F,part: &Part,data_set: &Dataset,lines: &Vec<String>) {
    let start = Instant::now();
    println!("[Day {}][{:?}][{:?}] Starting ", day, part, data_set);
    fct(part, lines);
    println!(
        "[Day {}][{:?}][{:?}] Duration {} ms ",
        day,
        part,
        data_set,
        start.elapsed().as_millis()
    )
}

pub fn to_lines(day: &u8, data_set: &Dataset) -> Vec<String> {
    return read_lines(day, data_set)
        .map(|lines| lines.filter_map(|l| l.ok()).collect())
        .unwrap_or(vec![]);
}

pub fn run_all<F: Fn(&Part, &Vec<String>)>(day: &u8, fct: &F, active: &Active) {
    if let Active::False = active {
        return;
    }

    let test_lines = to_lines(day, &Dataset::Test);
    let real_lines = to_lines(day, &Dataset::Real);
    run(day, fct, &Part::Part1, &Dataset::Test, &test_lines);
    println!("");
    run(day, fct, &Part::Part1, &Dataset::Real, &real_lines);
    println!("");
    run(day, fct, &Part::Part2, &Dataset::Test, &test_lines);
    println!("");
    run(day, fct, &Part::Part2, &Dataset::Real, &real_lines);
}

#[allow(dead_code)]
pub fn merge<A, B, C>(first: Option<A>, second: Option<B>, merger: fn(A, B) -> C) -> Option<C> {
    let first = first?;
    let second = second?;
    Some(merger(first, second))
}
