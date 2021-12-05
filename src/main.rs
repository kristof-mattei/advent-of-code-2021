#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]
use std::error::Error;

mod shared;
mod utils;

mod day_1;
mod day_2;
mod day_3;

fn print_answer(day: u32, part: u32, result: &Result<String, Box<dyn Error>>) {
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
    let solutions = vec![
        (
            day_1::part_1::find_solution().map(|r| r.to_string()),
            day_1::part_2::find_solution().map(|r| r.to_string()),
        ),
        (
            day_2::part_1::find_solution().map(|r| r.to_string()),
            day_2::part_2::find_solution().map(|r| r.to_string()),
        ),
        (
            day_3::part_1::find_solution().map(|r| r.to_string()),
            day_3::part_2::find_solution().map(|r| r.to_string()),
        ),
    ];

    let mut day: u32 = 1;

    for (day_solution_part_1, day_solution_part_2) in solutions {
        print_answer(day, 1, &day_solution_part_1);
        print_answer(day, 2, &day_solution_part_2);

        day += 1;
    }
}
