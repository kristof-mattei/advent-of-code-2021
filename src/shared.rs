pub trait Day {
    fn part_1(&self) -> PartSolution;
    fn part_2(&self) -> PartSolution;
}

#[derive(PartialEq, Eq, Debug)]
pub enum PartSolution {
    I32(i32),
    U32(u32),
    U64(u64),
    USize(usize),
    Vec(Vec<String>),
    None,
}

impl std::fmt::Display for PartSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match *self {
            PartSolution::I32(x) => x.to_string(),
            PartSolution::U32(x) => x.to_string(),
            PartSolution::U64(x) => x.to_string(),
            PartSolution::USize(x) => x.to_string(),
            PartSolution::Vec(ref x) => format!("\n{}", x.join("\n")),
            PartSolution::None => "None".to_owned(),
        };

        write!(f, "{}", string)
    }
}
