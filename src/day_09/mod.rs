fn check_valid_number(preamble: &Vec<u32>, next_number: u32) -> bool {
    false
}

fn find_first_invalid(input: &str, preamble_length: usize) -> u32 {
    127
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::*;

    #[test]
    fn check_example() {
        let file = read_file("./src/day_08/example.txt");
        let result = find_first_invalid(&file, 5);
        assert_eq!(127, result);
    }
}
