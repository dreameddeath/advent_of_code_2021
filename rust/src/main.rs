mod day1;
mod day2;
mod day12;
mod day15;
mod day16;
mod day18;
mod day22;
mod day23;
mod day23_2;
mod utils;

fn main() {
    utils::run_all(&1, &day1::puzzle, &utils::Active::False);
    utils::run_all(&2, &day2::puzzle, &utils::Active::False);
    utils::run_all(&12, &day12::puzzle, &utils::Active::False);
    utils::run_all(&15, &day15::puzzle, &utils::Active::True);
    utils::run_all(&16, &day16::puzzle, &utils::Active::False);
    utils::run_all(&18, &day18::puzzle, &utils::Active::False);
    utils::run_all(&22, &day22::puzzle, &utils::Active::False);
    utils::run_all(&23, &day23::puzzle, &utils::Active::False);
    utils::run_all(&23, &day23_2::puzzle, &utils::Active::False);

}
