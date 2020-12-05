fn check_amount_of_trees(file: &str) -> u64 {
    let mut field: Vec<Vec<char>> = Vec::new();
    for line in file.lines() {
        if line.len() > 0 {
            field.push(line.chars().collect());
        }
    }
    let repeat_at = field[0].len();
    let mut x = 0;
    let mut count = 0;
    for y in 0..field.len() {
        if field[y][x] == '#' {
            count = count + 1;
        }
        x = (x + 3) % repeat_at;
    }
    return count;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::read_file;

    #[test]
    fn check_amount_of_trees_of_example() {
        let file = read_file("./src/day_03/example.txt");
        let result = check_amount_of_trees(file.as_str());
        assert_eq!(7, result, "Should have found seven trees along the way");
    }

    #[test]
    fn check_amount_of_trees_of_input() {
        let file = read_file("./src/day_03/input.txt");
        let result = check_amount_of_trees(file.as_str());
        assert_eq!(274, result, "Should have found seven trees along the way");
    }
}
