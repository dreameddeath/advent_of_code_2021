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

pub fn read_lines(day: i8, is_test: &Dataset) -> Option<Lines<BufReader<File>>> {
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

pub fn run<F: Fn(&Part, &Dataset)>(fct: &F, part: &Part, dataset: &Dataset) {
    let start = Instant::now();
    println!("[{:?}][{:?}] Starting ", part, dataset);
    fct(part, dataset);
    println!(
        "[{:?}][{:?}] Duration {} ms ",
        part,
        dataset,
        start.elapsed().as_millis()
    )
}

pub fn run_all<F: Fn(&Part, &Dataset)>(fct: &F) {
    run(fct, &Part::Part1, &Dataset::Test);
    run(fct, &Part::Part1, &Dataset::Real);
    run(fct, &Part::Part2, &Dataset::Test);
    run(fct, &Part::Part2, &Dataset::Real);
}
