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
        if *current_joltage_adapter == last_possible_joltage + 2 {
            differences.insert(2, differences.get(&2).unwrap_or(&0) + 2);
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

fn find_possible_arrangements(adapters: &Vec<Jolts>) -> u64 {
    let mut sorted_adapters = adapters.clone();
    sorted_adapters.sort();
    sorted_adapters.push(sorted_adapters.last().unwrap() + 3);
    let mut pow_2 = 0;
    let mut pow_7 = 0;
    let mut b = 0;
    let mut c = *sorted_adapters.get(0).unwrap();
    let mut d = *sorted_adapters.get(1).unwrap();
    let mut e = *sorted_adapters.get(2).unwrap();
    println!(
        "b={},c={},d={},e={}, pow_2 = {}, pow_7 = {}",
        b, c, d, e, pow_2, pow_7
    );
    if e - b == 3 {
        pow_2 += 2;
    } else if e - c == 2 {
        pow_2 += 1;
    } else if d - b == 2 {
        pow_2 += 1;
    }
    let mut a = b;
    b = c;
    c = d;
    d = e;
    for adapter in sorted_adapters[3..].iter() {
        e = *adapter;
        if e - a == 4 {
            pow_7 += 1;
            pow_2 -= 2;
        } else if e - c == 2 {
            pow_2 += 1;
        }
        println!(
            "a={},b={},c={},d={},e={}, pow_2 = {}, pow_7 = {}",
            a, b, c, d, e, pow_2, pow_7
        );
        a = b;
        b = c;
        c = d;
        d = e;
    }
    println!("pow_2 = {}, pow_7 = {}", pow_2, pow_7);
    (2 as u64).pow(pow_2) * (7 as u64).pow(pow_7)
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

    #[test]
    fn check_example1_day_10_star2() {
        let file = read_file("./src/day_10/example_1.txt");
        let adapters = adapters_from_input(&file);
        let result = find_possible_arrangements(&adapters);
        assert_eq!(8, result);
    }

    #[test]
    fn check_example2_day_10_star2() {
        let file = read_file("./src/day_10/example_2.txt");
        let adapters = adapters_from_input(&file);
        let result = find_possible_arrangements(&adapters);
        assert_eq!(19208, result);
    }

    #[test]
    fn check_input_day_10_star2() {
        let file = read_file("./src/day_10/input.txt");
        let adapters = adapters_from_input(&file);
        let result = find_possible_arrangements(&adapters);
        assert_eq!(74049191673856, result);
    }
}
