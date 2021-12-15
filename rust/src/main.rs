mod day1;
mod day2;
mod day12;
mod day15;
mod utils;

fn main() {
    utils::run_all(&1, &day1::puzzle, &utils::Active::False);
    utils::run_all(&2, &day2::puzzle, &utils::Active::False);
    utils::run_all(&12, &day12::puzzle, &utils::Active::False);
    utils::run_all(&15, &day15::puzzle, &utils::Active::True);
}
