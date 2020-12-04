use regex::Regex;

fn check_password_policy(file: &str) -> u64 {
    let mut count = 0;
    for line in file.lines() {
        let is_valid = is_valid_password_with_policy(line);
        if is_valid {
            count = count + 1;
        }
    }
    return count;
}

fn is_valid_password_with_policy(line: &str) -> bool {
    let re = Regex::new(r"(\d+)-(\d+) (.): (.*)").unwrap();
    let groups = re.captures(line).unwrap();
    let minimum = groups.get(1).unwrap().as_str().parse::<u64>().unwrap();
    let maximum = groups.get(2).unwrap().as_str().parse::<u64>().unwrap();
    let character = groups.get(3).unwrap().as_str().chars().nth(0).unwrap();
    let password = groups.get(4).unwrap().as_str();

    let mut count = 0;
    for c in password.chars() {
        if c == character {
            count = count + 1;
            if maximum < count {
                return false;
            }
        }
    }

    minimum <= count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_example_policy() {
        let file = include_str!("./example.txt");
        let result = check_password_policy(file);
        assert_eq!(2, result, "Should have two correct passwords");
    }

    #[test]
    fn check_policy_for_input() {
        let file = include_str!("./input.txt");
        let result = check_password_policy(file);
        assert_eq!(628, result, "Should have two correct passwords");
    }
}
