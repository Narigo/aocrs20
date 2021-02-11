type Bus = usize;
type ArrivalTime = usize;

fn from_input(input: &str) -> (ArrivalTime, Vec<Bus>) {
    let lines = input.lines();
    (lines.get(0).unwrap().parse::<usize>(), lines.get(1).unwrap().split(",").filter(|c| c != 'x').parse::<usize>())
}

fn get_earliest_bus(arrival_time: ArrivalTime, busses: Vec<Bus>) -> Bus {
    59
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::*;

    #[test]
    fn check_day_13() {}

    #[test]
    fn check_day_13_star1() {}

    #[test]
    fn check_day_13_star2() {}

    #[test]
    fn check_day_13_star1_from_input() {
        let file = read_file("./src/day_13/example.txt");
        let (arrival_time, busses) = from_input(file);
        assert_eq(939, arrival_time);
        assert_eq(vec![7,13,59,31,19], busses);
    }

    fn check_day_13_star1_example() {
        let file = read_file("./src/day_13/example.txt");
    }

}
