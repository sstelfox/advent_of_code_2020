use std::fs::File;
use std::io::Read;

fn is_valid_password_count(rule_pass_pair: &str) -> bool {
    let rule_pass_split: Vec<&str> = rule_pass_pair.split(':').collect();
    assert_eq!(rule_pass_split.len(), 2);

    let (rule, password) = (rule_pass_split[0], &rule_pass_split[1][1..]);
    let rule_portions: Vec<&str> = rule.split(' ').collect();
    assert_eq!(rule_portions.len(), 2);

    let (rule_count_range, rule_character) = (rule_portions[0], rule_portions[1]);
    assert_eq!(rule_character.len(), 1);
    let rule_character = rule_character.chars().collect::<Vec<char>>()[0];

    let rule_count_range_portions: Vec<&str> = rule_count_range.split('-').collect();
    assert_eq!(rule_count_range_portions.len(), 2);
    let (rule_count_low, rule_count_high) = (rule_count_range_portions[0].parse::<usize>().unwrap(), rule_count_range_portions[1].parse::<usize>().unwrap());

    let matching_character_count: usize = password.chars().map(|c| if c == rule_character { 1 } else { 0 }).sum();

    matching_character_count >= rule_count_low && matching_character_count <= rule_count_high
}

fn main() {
    let mut in_dat_fh = File::open("./data/input1.txt").unwrap();
    let mut in_dat =String::new();

    in_dat_fh.read_to_string(&mut in_dat).unwrap();

    let valid_entries: usize = in_dat.lines().map(|l| if is_valid_password_count(l) { 1 } else { 0 }).sum();
    println!("input had {} valid passwords", valid_entries);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_password() {
        assert!(is_valid_password_count(&"1-3 a: abcde"));
        assert!(!is_valid_password_count(&"1-3 b: cdefg"));
        assert!(is_valid_password_count(&"2-9 c: ccccccccc"));
    }
}
