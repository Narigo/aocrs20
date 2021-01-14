use std::collections::HashMap;

type Differences = HashMap<Jolts, u64>;
type Jolts = u64;

fn get_highest_joltage(set: &Vec<Jolts>) -> (Differences, Jolts) {
    let mut differences: Differences = HashMap::new();
    let mut last_possible_joltage: u64 = 0;
    let mut sorted_adapters: Vec<Jolts> = set.clone();
    sorted_adapters.sort();
    for current_joltage_adapter in sorted_adapters.iter() {
        if *current_joltage_adapter == last_possible_joltage + 1 {
            differences.insert(1, differences.get(&1).unwrap_or(&0) + 1);
        }
        if *current_joltage_adapter == last_possible_joltage + 3 {
            differences.insert(3, differences.get(&3).unwrap_or(&1) + 1);
        }
        last_possible_joltage = *current_joltage_adapter;
    }
    (differences, last_possible_joltage + 3)
}

fn adapters_from_input(input: &String) -> Vec<Jolts> {
    input
        .lines()
        .map(|line| {
            line.parse::<u64>()
                .expect("should only have numbers in input")
        })
        .collect()
}

fn get_multiplication_value(adapter_differences: Differences) -> u64 {
    adapter_differences.get(&1).unwrap_or(&0) * adapter_differences.get(&3).unwrap_or(&0)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::*;

    #[test]
    fn check_example1_day_10() {
        let file = read_file("./src/day_10/example_1.txt");
        let adapters = adapters_from_input(&file);
        let result = get_highest_joltage(&adapters);
        assert_eq!(22, result.1);
    }

    #[test]
    fn check_example2_day_10() {
        let file = read_file("./src/day_10/example_2.txt");
        let adapters = adapters_from_input(&file);
        let result = get_highest_joltage(&adapters);
        // 22 differences of 1 jolt
        assert_eq!(Some(&22), result.0.get(&1));
        // 10 differences of 3 jolt
        assert_eq!(Some(&10), result.0.get(&3));
        assert_eq!(220, get_multiplication_value(result.0));
    }

    #[test]
    fn check_input_day_10() {
        let file = read_file("./src/day_10/input.txt");
        let adapters = adapters_from_input(&file);
        let result = get_highest_joltage(&adapters);
        assert_eq!(2738, get_multiplication_value(result.0));
    }
}
