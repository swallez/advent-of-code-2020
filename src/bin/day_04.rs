use std::collections::HashMap;
use std::collections::HashSet;
use once_cell::sync::Lazy;
use std::iter::FromIterator;
use std::ops::Deref;
use regex::Regex;

fn main() {
    println!("Passports with valid keys: {}", count_passports_with_valid_keys(DATA));
    println!("Passports with valid values: {}", count_passports_with_valid_values(DATA));
}

//--------------------------------------------------------------------------------------------------
// Problem 1

const DATA: &str = include_str!("../../data/day_04.txt");

static KNOWN_KEYS: Lazy<HashSet<&str>> = Lazy::new(|| HashSet::from_iter(
    vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"].into_iter()));

/// "cid" isn't required
static REQUIRED_KEYS: Lazy<HashSet<&str>> = Lazy::new(|| {
    let mut r = KNOWN_KEYS.deref().clone();
    r.remove("cid");
    r
});

type Passport = HashMap<String, String>;

fn count_passports_with_valid_keys(data: &'static str) -> usize {
    read_passports(data).iter().map(has_valid_keys).filter(|valid| *valid).count()
}

fn has_valid_keys(p: &Passport) -> bool {
    let keys: HashSet<&str> = HashSet::from_iter(p.keys().map(|s| s.as_str()));

    let valid_keys = keys.difference(KNOWN_KEYS.deref()).next().is_none();
    let has_required_keys = REQUIRED_KEYS.difference(&keys).next().is_none();

    valid_keys && has_required_keys
}

fn read_passports(s: &str) -> Vec<Passport> {
    // Normalize key/value so that a passport's k/v pairs are *all* separated by a space
    let s = s.replace("\n\n", "|").replace("\n", " ");

    s.split("|").map(|keys| {
        keys.split(' ')
            .filter(|kv| *kv != "") // Last line reported as empty string
            .map(|kv| {
                let mut split = kv.split(':');
                let (k, v) = (split.next().unwrap(), split.next().unwrap());
                (k.to_owned(), v.to_owned())
            })
            .collect::<Passport>()
    }).collect::<Vec<_>>()
}

//--------------------------------------------------------------------------------------------------
// Problem 2

/// A validator takes a string and returns a bool. 'static + Send + Sync needed for the global
/// validator map.
type Validator = Box<(dyn Fn(& str) -> bool + 'static + Send + Sync)>;

/// Validators by passport key
static VALIDATORS: Lazy<HashMap<&str, Validator>> = Lazy::new(|| {
    let mut r: HashMap<&str, Validator> = HashMap::new();

    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    r.insert("byr", Box::new(|s| {
        s.parse::<i32>().map(|y| y >= 1920 && y <= 2002).unwrap_or(false)
    }));

    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    r.insert("iyr", Box::new(|s| {
        s.parse::<i32>().map(|y| y >= 2010 && y <= 2020).unwrap_or(false)
    }));

    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    r.insert("eyr", Box::new(|s| {
        s.parse::<i32>().map(|y| y >= 2020 && y <= 2030).unwrap_or(false)
    }));

    // hgt (Height) - a number followed by either cm or in:
    // - If cm, the number must be at least 150 and at most 193.
    // - If in, the number must be at least 59 and at most 76.
    let re = Regex::new("^([0-9]+)(cm|in)$").unwrap();
    r.insert("hgt", Box::new(move |s| {

        // Pure functional approach
        // re.captures(s).map(|cap| {
        //     cap[1].parse::<i32>().map(|n| {
        //         match &cap[2] {
        //             "cm" if n >= 150 && n <= 193 => true,
        //             "in" if n >= 59 && n <= 76 => true,
        //             _ => false
        //         }
        //     }).unwrap_or(false)
        // }).unwrap_or(false)

        // Use errors to have a linear happy path
        (|| -> anyhow::Result<bool> {
            let cap = re.captures(s).ok_or(anyhow::anyhow!(""))?;
            let n: i32 = cap[1].parse()?;
            Ok(match &cap[2] {
                "cm" => n >= 150 && n <= 193,
                "in" => n >= 59 && n <= 76,
                _ => false
            })
        })().unwrap_or(false)
    }));

    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    let re = Regex::new("^#[0-9a-f]{6}$").unwrap();
    r.insert("hcl", Box::new(move |s| re.is_match(s)));

    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    let re = Regex::new("^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    r.insert("ecl", Box::new(move |s| re.is_match(s)));

    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    let re = Regex::new("^[0-9]{9}$").unwrap();
    r.insert("pid", Box::new(move |s| re.is_match(s)));

    // cid (Country ID) - ignored, missing or not.
    r.insert("cid", Box::new(|_| true));

    r
});

fn has_valid_values(p: &Passport) -> bool {
    if !has_valid_keys(p) {
        return false;
    }

    p.iter().fold(true, |acc, (k, v)| {
        acc & VALIDATORS[k.as_str()](v)
    })
}

fn count_passports_with_valid_values(data: &'static str) -> usize {
    read_passports(data).iter().map(has_valid_values).filter(|valid| *valid).count()
}

//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_DATA: &str = include_str!("../../data/day_04_test.txt");

    #[test]
    fn test_parse_passports() {
        let p = read_passports(TEST_DATA);
        println!("{:?}", p);
        assert_eq!(4, p.len());
    }

    #[test]
    fn test_validate_passport_keys() {
        let p = read_passports(TEST_DATA);

        assert_eq!(4, p.len());
        assert_eq!(true, has_valid_keys(&p[0]));
        assert_eq!(false, has_valid_keys(&p[1]));
        assert_eq!(true, has_valid_keys(&p[2]));
        assert_eq!(false, has_valid_keys(&p[3]));

        assert_eq!(2, count_passports_with_valid_keys(TEST_DATA));
    }

    #[test]
    fn test_validate_value() {

        fn validate(expected: bool, k: &str, v: &str) {
            assert_eq!(expected, VALIDATORS[k](v), "{} = {}", k, v);
        }

        validate(true, "byr", "2002");
        validate(true, "hgt", "60in");
        validate(true, "hgt", "190cm");
        validate(true, "hcl", "#123abc");
        validate(true, "ecl", "brn");
        validate(true, "pid", "000000001");

        validate(false, "byr", "2003");
        validate(false, "hgt", "190in");
        validate(false, "hgt", "190");
        validate(false, "hcl", "#123abz");
        validate(false, "hcl", "123abc");
        validate(false, "ecl", "wat");
        validate(false, "pid", "0123456789");
    }
}
