fn check_valid_number(preamble: &Vec<u64>, number: &u64) -> bool {
    for a in preamble.iter() {
        for b in preamble.iter() {
            if a == b {
                continue;
            }
            if a + b == *number {
                return true;
            }
        }
    }
    false
}

fn find_first_invalid(input: &str, preamble_length: usize) -> u64 {
    let numbers: Vec<u64> = input
        .lines()
        .map(|s| s.parse::<u64>().expect("should be numbers"))
        .collect();
    for i in preamble_length..numbers.len() {
        let number = numbers.get(i).unwrap();
        let to_check: Vec<u64> = numbers
            .clone()
            .into_iter()
            .skip(i - preamble_length)
            .take(preamble_length)
            .collect();
        if !check_valid_number(&to_check, number) {
            return *number;
        }
    }
    0
}

fn find_sum_of_first_invalid(input: &str, number: u64) -> Vec<u64> {
    let mut result = Vec::new();
    for line in input.lines() {
        let n = line.parse::<u64>().unwrap();
        println!("len={}, n={}", result.len(), n);
        result.insert(result.len(), n);
        let mut current_sum = result.iter().sum::<u64>();
        if current_sum == number {
            return result;
        }
        while current_sum > number {
            result.remove(0);
            current_sum = result.iter().sum::<u64>();
        }
    }
    result
}

fn minimum(set: &Vec<u64>) -> u64 {
    *set.get(set.len() - 1).unwrap()
}

fn maximum(set: &Vec<u64>) -> u64 {
    *(set.get(0).unwrap())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::*;

    #[test]
    fn check_example_day_09() {
        let file = read_file("./src/day_09/example.txt");
        let result = find_first_invalid(&file, 5);
        assert_eq!(127, result);
    }

    #[test]
    fn check_input_day_09() {
        let file = read_file("./src/day_09/input.txt");
        let result = find_first_invalid(&file, 25);
        assert_eq!(675280050, result);
    }

    #[test]
    fn check_example_day_09_star2() {
        let file = read_file("./src/day_09/example.txt");
        let first_invalid = find_first_invalid(&file, 5);
        let summed_numbers = find_sum_of_first_invalid(&file, first_invalid);
        println!("numbers: {:?}", summed_numbers);
        let min = minimum(&summed_numbers);
        let max = maximum(&summed_numbers);
        assert_eq!(62, min + max);
    }
}
