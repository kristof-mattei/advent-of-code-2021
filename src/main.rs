#![warn(clippy::all, clippy::nursery, clippy::cargo)]

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod error;
mod utils;

fn print_answer<T: std::fmt::Display, E: std::fmt::Display>(
    day: &u32,
    part: &u32,
    result: &Result<T, E>,
) {
    match result {
        Ok(r) => {
            println!("Answer to Day {}, part {} is ... {}", day, part, r);
        }
        Err(e) => {
            println!(
                "Whoops! Day {}, part {} gave an error of ... {}",
                day, part, e
            );
        }
    }
}
fn main() {
    let days = [
        (
            day_1::part_1::find_solution(),
            day_1::part_2::find_solution(),
        ),
        (
            day_2::part_1::find_solution(),
            day_2::part_2::find_solution(),
        ),
        (
            day_3::part_1::find_solution(),
            day_3::part_2::find_solution(),
        ),
        (
            day_4::part_1::find_solution(),
            day_4::part_2::find_solution(),
        ),
    ];

    for (i, (part1, part2)) in days.iter().enumerate() {
        print_answer(&(i as u32 + 1), &1, part1);
        print_answer(&(i as u32 + 1), &2, part2);
    }
}
