use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use hashbrown::{HashMap, HashSet};

use crate::shared::{Day, PartSolution};

#[derive(Eq, Default, Debug)]
struct Cave {
    name: String,
    targets: RefCell<Caves>,
}

impl Cave {
    fn is_end(&self) -> bool {
        self.name == "end"
    }

    fn is_small(&self) -> bool {
        self.name.to_lowercase() == self.name
    }
}

impl PartialEq for Cave {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Cave {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

type Caves = HashSet<Rc<Cave>>;

fn get_or_insert_cave(caves: &mut Caves, cave_name: &str) -> Rc<Cave> {
    let cave = Rc::new(Cave {
        name: cave_name.to_owned(),
        ..Cave::default()
    });

    if let Some(found_cave) = caves.get(&cave) {
        Rc::clone(found_cave)
    } else {
        caves.insert(Rc::clone(&cave));

        cave
    }
}

fn add_path(caves: &mut Caves, from: &str, to: &str) {
    let from_cave = get_or_insert_cave(caves, from);
    let to_cave = get_or_insert_cave(caves, to);

    from_cave.targets.borrow_mut().insert(Rc::clone(&to_cave));
    to_cave.targets.borrow_mut().insert(from_cave);
}

fn build_cave_system(lines: &[&str]) -> Caves {
    let mut caves: Caves = HashSet::default();

    for line in lines {
        let pieces: Vec<&str> = line.split('-').collect();

        let left = pieces.first().unwrap();
        let right = pieces.get(1).unwrap();

        add_path(&mut caves, left, right);
    }

    caves
}

fn navigate_caves<F>(
    cave: &Rc<Cave>,
    can_revisit: &F,
    mut visited: Vec<Rc<Cave>>,
) -> Vec<Vec<Rc<Cave>>>
where
    F: Fn(&[Rc<Cave>], &Rc<Cave>) -> bool,
{
    let mut solutions = Vec::new();

    let vc = Rc::clone(cave);
    visited.push(vc);

    if cave.is_end() {
        solutions.push(visited);
    } else {
        for target_cave in &*cave.targets.borrow() {
            if can_revisit(&visited, target_cave) {
                let visited_new = visited.clone();

                println!("Visiting {} -> {}", cave.name, target_cave.name);

                let mut new_solutions = navigate_caves(target_cave, can_revisit, visited_new);

                solutions.append(&mut new_solutions);
            }
        }
    }

    solutions
}

fn calculate_all_paths<F>(cave_system: &Caves, can_revisit: F) -> usize
where
    F: Fn(&[Rc<Cave>], &Rc<Cave>) -> bool,
{
    let start = cave_system.iter().find(|c| c.name == "start").unwrap();

    let solutions = navigate_caves(start, &can_revisit, Vec::new());

    println!("The end, we visited the following paths to get here.");

    let mut debug_lines = solutions
        .iter()
        .map(|solution| {
            solution
                .iter()
                .map(|x| x.name.clone())
                .collect::<Vec<_>>()
                .join(",")
        })
        .collect::<Vec<_>>();

    debug_lines.sort();
    for line in debug_lines {
        println!("{}", line);
    }

    solutions.len()
}

fn can_visit_part_1(visited_caves: &[Rc<Cave>], cave: &Rc<Cave>) -> bool {
    if cave.is_small() {
        !visited_caves.contains(cave)
    } else {
        true
    }
}

fn can_visit_part_2(visited_caves: &[Rc<Cave>], cave: &Rc<Cave>) -> bool {
    let cave_name = cave.name.clone();

    if cave_name == "start" || cave_name == "end" {
        !visited_caves.contains(cave) // only if we haven't visited them yet
    } else if cave.is_small() && visited_caves.contains(cave) {
        // if the cave is small and we haven't visited it, fall through
        // BUT we can visit ONE small cave twice.
        // so let's count the small caves, and see if we visited ANY twice
        // if we didn't, we can visit this one again
        let mut visit_counts: HashMap<Rc<Cave>, u32> = HashMap::new();

        for visited_cave in visited_caves.iter().filter(|x| x.is_small()) {
            let visit_count = visit_counts
                .entry(Rc::clone(visited_cave))
                .and_modify(|c| *c += 1)
                .or_insert(1);

            if *visit_count >= 2 {
                return false;
            }
        }

        true
    } else {
        true // unlimited
    }
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("day_12/input.txt").lines().collect();

        let cave_system = build_cave_system(&lines);

        let paths: usize = calculate_all_paths(&cave_system, can_visit_part_1);

        PartSolution::USize(paths)
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("day_12/input.txt").lines().collect();

        let cave_system = build_cave_system(&lines);

        let paths: usize = calculate_all_paths(&cave_system, can_visit_part_2);

        PartSolution::USize(paths)
    }
}

#[cfg(test)]
mod test {
    fn get_example() -> Vec<&'static str> {
        include_str!("day_12/example.txt").lines().collect()
    }

    fn get_example_slightly_larger() -> Vec<&'static str> {
        include_str!("day_12/example_slightly_larger.txt")
            .lines()
            .collect()
    }

    fn get_example_even_larger() -> Vec<&'static str> {
        include_str!("day_12/example_even_larger.txt")
            .lines()
            .collect()
    }

    mod part_1 {
        use pretty_assertions::assert_eq;

        use super::{get_example, get_example_even_larger, get_example_slightly_larger};
        use crate::day_12::{Solution, build_cave_system, calculate_all_paths, can_visit_part_1};
        use crate::shared::{Day as _, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_1(), PartSolution::USize(4495));
        }

        #[test]
        fn example() {
            let lines = get_example();

            let cave_system = build_cave_system(&lines);

            let paths: usize = calculate_all_paths(&cave_system, can_visit_part_1);

            assert_eq!(paths, 10);
        }

        #[test]
        fn example_slightly_larger() {
            let lines = get_example_slightly_larger();

            let cave_system = build_cave_system(&lines);

            let paths: usize = calculate_all_paths(&cave_system, can_visit_part_1);

            assert_eq!(paths, 19);
        }

        #[test]
        fn example_even_larger() {
            let lines = get_example_even_larger();

            let cave_system = build_cave_system(&lines);

            let paths: usize = calculate_all_paths(&cave_system, can_visit_part_1);

            assert_eq!(paths, 226);
        }
    }

    mod part_2 {
        use pretty_assertions::assert_eq;

        use super::get_example;
        use crate::day_12::test::{get_example_even_larger, get_example_slightly_larger};
        use crate::day_12::{Solution, build_cave_system, calculate_all_paths, can_visit_part_2};
        use crate::shared::{Day as _, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_2(), PartSolution::USize(131_254));
        }

        #[test]
        fn example() {
            let lines = get_example();

            let cave_system = build_cave_system(&lines);

            let paths: usize = calculate_all_paths(&cave_system, can_visit_part_2);

            assert_eq!(paths, 36);
        }

        #[test]
        fn example_slightly_larger() {
            let lines = get_example_slightly_larger();

            let cave_system = build_cave_system(&lines);

            let paths: usize = calculate_all_paths(&cave_system, can_visit_part_2);

            assert_eq!(paths, 103);
        }

        #[test]
        fn example_even_larger() {
            let lines = get_example_even_larger();

            let cave_system = build_cave_system(&lines);

            let paths: usize = calculate_all_paths(&cave_system, can_visit_part_2);

            assert_eq!(paths, 3509);
        }
    }
}
