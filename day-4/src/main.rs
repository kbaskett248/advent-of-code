use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let passports: Vec<HashMap<String, String>> =
            chunk_lines(lines).map(parse_passport).collect();

        let num_valid_passports = passports.iter().filter(|x| passport_is_valid(x)).count();
        println!("PART 1: There are {} valid passports.", num_valid_passports);

        let num_valid_passports_pt_2 = passports
            .iter()
            .filter(|x| passport_is_valid_pt_2(x))
            .count();
        println!(
            "PART 2: There are {} valid passports.",
            num_valid_passports_pt_2
        );
    }
}

fn read_lines<P>(filename: P) -> io::Result<impl Iterator<Item = String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().filter_map(|x| x.ok()))
}

fn chunk_lines(lines: impl Iterator<Item = String>) -> impl Iterator<Item = String> {
    lines
        .scan(vec![], |container, line| match line.as_str() {
            "" => {
                let chunk = Some(container.join(" "));
                container.clear();
                chunk
            }
            _ => {
                container.push(line);
                Some("".to_string())
            }
        })
        .filter(|x| x != "")
}

fn parse_passport(s: String) -> HashMap<String, String> {
    let mut passport = HashMap::with_capacity(8);
    for part in s.split_whitespace() {
        if let [key, value] = *part.splitn(2, ':').collect::<Vec<&str>>().as_slice() {
            passport.insert(key.to_string(), value.to_string());
        }
    }
    passport
}

fn passport_is_valid(passport: &HashMap<String, String>) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|k| passport.contains_key(&k.to_string()))
}

fn passport_is_valid_pt_2(passport: &HashMap<String, String>) -> bool {
    if passport_is_valid(passport) {
        passport.iter().all(|(key, val)| match key.as_str() {
            "byr" => {
                if let Ok(year) = val.parse::<i16>() {
                    year >= 1920 && year <= 2002
                } else {
                    false
                }
            }
            "iyr" => {
                if let Ok(year) = val.parse::<i16>() {
                    year >= 2010 && year <= 2020
                } else {
                    false
                }
            }
            "eyr" => {
                if let Ok(year) = val.parse::<i16>() {
                    year >= 2020 && year <= 2030
                } else {
                    false
                }
            }
            "hgt" => {
                if let Some((height, units)) = parse_height(val) {
                    match units {
                        "in" => height >= 59 && 76 >= height,
                        "cm" => height >= 150 && 193 >= height,
                        _ => false,
                    }
                } else {
                    false
                }
            }
            "hcl" => {
                if let Ok(re) = Regex::new(r"^#[0-9a-f]{6}$") {
                    re.is_match(val)
                } else {
                    false
                }
            }
            "ecl" => ["amb", "blu", "brn", "grn", "gry", "hzl", "oth"]
                .binary_search(&val.as_str())
                .is_ok(),
            "pid" => {
                if let Ok(re) = Regex::new(r"^[0-9]{9}$") {
                    re.is_match(val)
                } else {
                    false
                }
            }
            _ => true,
        })
    } else {
        false
    }
}

fn parse_height(value: &str) -> Option<(i16, &str)> {
    let re = Regex::new(r"^(?P<height>\d{2,3})(?P<units>(cm|in))$").ok()?;
    let caps = re.captures(value)?;
    let height = caps.name("height")?.as_str().parse().ok()?;
    let units = caps.name("units")?.as_str();
    Some((height, units))
}
