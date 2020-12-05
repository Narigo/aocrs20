use regex::Regex;

fn check_password_policy(file: &str, is_valid_fn: &dyn Fn(&str) -> bool) -> u64 {
    let mut count = 0;
    for line in file.lines() {
        let is_valid = is_valid_fn(line);
        if is_valid {
            count = count + 1;
        }
    }
    return count;
}

fn is_valid_password_with_policy_of_old_shop(line: &str) -> bool {
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

fn is_valid_password_with_policy_of_new_shop(line: &str) -> bool {
    let re = Regex::new(r"(\d+)-(\d+) (.): (.*)").unwrap();
    let groups = re.captures(line).unwrap();
    let first_position = groups.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let second_position = groups.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let character = groups.get(3).unwrap().as_str().chars().nth(0).unwrap();
    let password = groups.get(4).unwrap().as_str();

    let a = password.chars().nth(first_position - 1).unwrap();
    let b = password.chars().nth(second_position - 1).unwrap();

    (a == character || b == character) && !(a == character && b == character)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_old_policy_for_example() {
        let file = include_str!("./example.txt");
        let result = check_password_policy(file, &is_valid_password_with_policy_of_old_shop);
        assert_eq!(2, result, "Should have two correct passwords");
    }

    #[test]
    fn check_old_policy_for_input() {
        let file = include_str!("./input.txt");
        let result = check_password_policy(file, &is_valid_password_with_policy_of_old_shop);
        assert_eq!(628, result, "Should have 628 correct passwords");
    }

    #[test]
    fn check_new_policy_for_example() {
        let file = include_str!("./example.txt");
        let result = check_password_policy(file, &is_valid_password_with_policy_of_new_shop);
        assert_eq!(1, result, "Should have one correct password with the new policy");
    }

    #[test]
    fn check_new_policy_for_input() {
        let file = include_str!("./input.txt");
        let result = check_password_policy(file, &is_valid_password_with_policy_of_new_shop);
        assert_eq!(705, result, "Should have 705 correct passwords");
    }
}
