use regex::Regex;
use once_cell::sync::Lazy;

const DATA: &str = include_str!("../../data/day_02.txt");

pub fn main() -> anyhow::Result<()> {

    let data = DATA.lines()
        .map(|line| parse_line(line))
        .collect::<Vec<_>>();

    let count = data.iter()
        .filter(|(min, max, expected, pwd)|
            valid_password_old_policy(*min, *max, *expected, pwd).is_some()
        )
        .count();

    println!("Old policy: there are {} valid passwords", count);

    let count = data.iter()
        .filter(|(min, max, expected, pwd)|
            valid_password_new_policy(*min, *max, *expected, pwd).is_some()
        )
        .count();

    println!("New policy: there are {} valid passwords", count);

    Ok(())
}

// Example: 1-7 j: vrfjljjwbsv
static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d+)-(\d+) (.): (.*)$").unwrap());

fn parse_line(text: &str) -> (usize, usize, char, String) {

    let caps = RE.captures(text).unwrap();
    let min: usize = caps[1].parse().unwrap();
    let max: usize = caps[2].parse().unwrap();
    let checked: char = caps[3].chars().next().unwrap();
    let pwd = caps[4].to_owned();

    (min, max, checked, pwd)
}


fn valid_password_old_policy(min: usize, max: usize, checked: char, pwd: &str) -> Option<&str> {
    let count = pwd.chars().filter(|c| checked == *c).count();
    if count < min || count > max {
        None
    } else {
        Some(pwd)
    }
}

fn valid_password_new_policy(pos1: usize, pos2: usize, checked: char, pwd: &str) -> Option<&str> {
    let count = [pos1, pos2].iter().filter_map(|pos| {
        if let Some(c) = pwd.chars().nth(*pos-1) {
            if c == checked {
                Some(())
            } else {
                None
            }
        } else {
            None
        }
    }).count();

    if count == 1 {
        Some(pwd)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!((1, 3, 'a', "abcde"), parse_line("1-3 a: abcde"));
    }

    #[test]
    fn test_valid_password() {
        assert_eq!(Some("abcde"), valid_password_old_policy(1, 3, 'a', "abcde"));
        assert_eq!(None, valid_password_old_policy(1, 3, 'b', "cdefg"));
        assert_eq!(Some("ccccccccc"), valid_password_old_policy(2, 9, 'c', "ccccccccc"));
    }

    #[test]
    fn test_valid_password2() {
        assert_eq!(Some("abcde"), valid_password_new_policy(1, 3, 'a', "abcde"));
        assert_eq!(None, valid_password_new_policy(1, 3, 'b', "cdefg"));
        assert_eq!(None, valid_password_new_policy(2, 9, 'c', "ccccccccc"));
    }

}
