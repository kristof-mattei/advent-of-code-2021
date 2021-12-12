use crate::utils::read_file;

#[derive(Default, Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
        // && self.cid.is_some() // to allow our own...
    }
}

fn parse_passport(passport_pieces: Vec<&str>) -> Passport {
    let mut passport: Passport = Default::default();

    for piece in passport_pieces {
        let split: Vec<&str> = piece.split(':').collect();

        let identifier = split[0];
        let value = split[1];

        match identifier {
            "byr" /*(Birth Year)*/ => passport.byr = Some(value.into()),
            "iyr" /*(Issue Year)*/ => passport.iyr = Some(value.into()),
            "eyr" /*(Expiration Year)*/ => passport.eyr = Some(value.into()),
            "hgt" /*(Height)*/ => passport.hgt = Some(value.into()),
            "hcl" /*(Hair Color)*/ => passport.hcl = Some(value.into()),
            "ecl" /*(Eye Color)*/ => passport.ecl = Some(value.into()),
            "pid" /*(Passport ID)*/ => passport.pid = Some(value.into()),
            "cid" /*(Country ID)*/ => passport.cid = Some(value.into()),
            _ => panic!("WTF DID YOU GIVE ME?")
        }
    }

    println!("{:?}, valid: {}", passport, passport.is_valid());
    passport
}

fn get_passports(lines: &Vec<String>) -> Vec<Passport> {
    let mut passports: Vec<Passport> = Vec::new();

    let groups = lines.split(|l| l.len() == 0);

    for group in groups {
        let mut passport_pieces = Vec::new();

        for line in group {
            line.split(' ').for_each(|p| passport_pieces.push(p))
        }

        passports.push(parse_passport(passport_pieces));
    }

    passports
}

// https://adventofcode.com/2020/day/2
pub fn find_solution() -> Result<u32, Box<dyn std::error::Error>> {
    let split = read_file("./src/day_4/input.txt".into())?;

    let valid_passports = get_passports(&split)
        .into_iter()
        .map(|p| p.is_valid())
        .count();

    Ok(valid_passports as u32)
}

#[test]
fn outcome() {
    assert_eq!(191, find_solution().unwrap());
}
