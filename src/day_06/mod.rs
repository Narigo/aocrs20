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
}
