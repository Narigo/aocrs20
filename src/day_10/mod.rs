use std::collections::HashMap;

type Differences = HashMap<Jolts, u64>;
type Jolts = u64;

fn get_highest_joltage(set: &Vec<Jolts>) -> (Differences, Jolts) {
    let mut differences: Differences = HashMap::new();
    let mut last_possible_joltage = 0;
    let mut sorted_adapters: Vec<Jolts> = set.clone();
    sorted_adapters.sort();
    for current_joltage_adapter in sorted_adapters.iter() {
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
}
