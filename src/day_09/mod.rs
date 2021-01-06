fn check_valid_number(preamble: &Vec<u32>, number: &u32) -> bool {
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

fn find_first_invalid(input: &str, preamble_length: usize) -> u32 {
    let numbers: Vec<u32> = input
        .lines()
        .map(|s| s.parse::<u32>().expect("should be numbers"))
        .collect();
    for i in preamble_length..numbers.len() {
        let number = numbers.get(i).unwrap();
        let to_check: Vec<u32> = numbers
            .clone()
            .into_iter()
            .skip(i - preamble_length)
            .take(preamble_length)
            .collect();
        if check_valid_number(&to_check, number) {
            println!("{} is okay", number);
        } else {
            println!("{} is NOT okay", number);
            return *number;
        }
    }
    0
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
}
