fn generate_field(file: &str) -> Vec<Vec<char>> {
    let mut field: Vec<Vec<char>> = Vec::new();
    for line in file.lines() {
        if line.len() > 0 {
            field.push(line.chars().collect());
        }
    }
    field
}

fn get_amount_of_trees_in_slope(field: &Vec<Vec<char>>, right: usize, down: usize) -> u64 {
    let repeat_at = field[0].len();
    let mut x = 0;
    let mut count = 0;
    for y in (0..field.len()).step_by(down) {
        if field[y][x] == '#' {
            count = count + 1;
        }
        x = (x + right) % repeat_at;
    }
    return count;
}

fn check_amount_of_trees_a(file: &str) -> u64 {
    let field = generate_field(file);
    get_amount_of_trees_in_slope(&field, 3, 1)
}

fn check_amount_of_trees_b(file: &str) -> u64 {
    let field = generate_field(file);
    let result_1_1 = get_amount_of_trees_in_slope(&field, 1, 1);
    let result_3_1 = get_amount_of_trees_in_slope(&field, 3, 1);
    let result_5_1 = get_amount_of_trees_in_slope(&field, 5, 1);
    let result_7_1 = get_amount_of_trees_in_slope(&field, 7, 1);
    let result_1_2 = get_amount_of_trees_in_slope(&field, 1, 2);
    result_1_1 * result_3_1 * result_5_1 * result_7_1 * result_1_2
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::read_file;

    #[test]
    fn check_amount_of_trees_a_of_example() {
        let file = read_file("./src/day_03/example.txt");
        let result = check_amount_of_trees_a(file.as_str());
        assert_eq!(7, result, "Should have found seven trees along the way");
    }

    #[test]
    fn check_amount_of_trees_a_of_input() {
        let file = read_file("./src/day_03/input.txt");
        let result = check_amount_of_trees_a(file.as_str());
        assert_eq!(274, result, "Should have found seven trees along the way");
    }

    #[test]
    fn check_amount_of_trees_at_slopes() {
        let file = read_file("./src/day_03/example.txt");
        let field = generate_field(file.as_str());
        let result_1_1 = get_amount_of_trees_in_slope(&field, 1, 1);
        let result_3_1 = get_amount_of_trees_in_slope(&field, 3, 1);
        let result_5_1 = get_amount_of_trees_in_slope(&field, 5, 1);
        let result_7_1 = get_amount_of_trees_in_slope(&field, 7, 1);
        let result_1_2 = get_amount_of_trees_in_slope(&field, 1, 2);
        assert_eq!(
            2, result_1_1,
            "Should have found seven trees along the for result_1_1"
        );
        assert_eq!(
            7, result_3_1,
            "Should have found seven trees along the for result_3_1"
        );
        assert_eq!(
            3, result_5_1,
            "Should have found seven trees along the for result_5_1"
        );
        assert_eq!(
            4, result_7_1,
            "Should have found seven trees along the for result_7_1"
        );
        assert_eq!(
            2, result_1_2,
            "Should have found seven trees along the for result_1_2"
        );
    }

    #[test]
    fn check_amount_of_trees_b_of_example() {
        let file = read_file("./src/day_03/example.txt");
        let result = check_amount_of_trees_b(file.as_str());
        assert_eq!(336, result, "Multiplication of results should be 336");
    }

    #[test]
    fn check_amount_of_trees_b_of_input() {
        let file = read_file("./src/day_03/input.txt");
        let result = check_amount_of_trees_b(file.as_str());
        assert_eq!(6050183040, result, "Multiplication of results should be 6050183040");
    }
}
