use std::collections::HashMap;
use std::collections::HashSet;

fn amount_of_questions_answered(input: Vec<String>) -> u32 {
    let mut result = 0;
    for part in input {
        let answers = part.replace("\n", "");
        let mut unique_chars = HashSet::new();
        for c in answers.chars() {
            unique_chars.insert(c);
        }
        result = result + unique_chars.len();
    }
    result as u32
}

fn amount_of_questions_answered_by_all(input: Vec<String>) -> u32 {
    let mut result = 0;
    for part in input {
        let mut answers = HashMap::<char, usize>::new();
        let mut amount_of_people = 0;
        for answer in part.lines() {
            amount_of_people += 1;
            for a in answer.chars() {
                *answers.entry(a).or_insert(0) += 1;
            }
        }
        answers.retain(|_k, v| *v == amount_of_people);
        result += answers.len();
    }
    result as u32
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::*;

    #[test]
    fn check_example() {
        let file = read_file("./src/day_06/example.txt");
        let splitted = split_into_parts(&file);
        let expected = vec!["abc", "a\nb\nc", "ab\nac", "a\na\na\na", "b"];
        assert_eq!(expected, splitted);
        let result = amount_of_questions_answered(splitted);
        let expected = 3 + 3 + 3 + 1 + 1;
        assert_eq!(expected, result);
    }

    #[test]
    fn check_input() {
        let file = read_file("./src/day_06/input.txt");
        let splitted = split_into_parts(&file);
        let result = amount_of_questions_answered(splitted);
        assert_eq!(6551, result);
    }

    #[test]
    fn check_example_for_all() {
        let file = read_file("./src/day_06/example.txt");
        let splitted = split_into_parts(&file);
        let result = amount_of_questions_answered_by_all(splitted);
        assert_eq!(6, result);
    }
}
