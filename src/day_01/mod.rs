pub fn check_expense_report_a(report: String) -> u64 {
    for line in report.lines() {
        let current_number = line.parse::<u64>().unwrap();
        for line in report.lines() {
            let number = line.parse::<u64>().unwrap();
            if current_number + number == 2020 {
                return current_number * number;
            }
        }
    }
    return 0;
}

pub fn check_expense_report_b(report: String) -> u64 {
    for line in report.lines() {
        let a = line.parse::<u64>().unwrap();
        for line in report.lines() {
            let b = line.parse::<u64>().unwrap();
            for line in report.lines() {
                let c = line.parse::<u64>().unwrap();
                if a + b + c == 2020 {
                    return a * b * c;
                }
            }
        }
    }
    return 0;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::read_file;

    #[test]
    fn check_example() {
        let input = read_file("./src/day_01/example.txt");
        let result = check_expense_report_a(input);
        assert_eq!(result, 514579);
    }

    #[test]
    fn check_input() {
        let input = read_file("./src/day_01/input.txt");
        let result = check_expense_report_a(input);
        assert_eq!(result, 928896);
    }

    #[test]
    fn check_example_three() {
        let input = read_file("./src/day_01/example.txt");
        let result = check_expense_report_b(input);
        assert_eq!(result, 241861950);
    }

    #[test]
    fn check_input_three() {
        let input = read_file("./src/day_01/input.txt");
        let result = check_expense_report_b(input);
        assert_eq!(result, 295668576);
    }
}
