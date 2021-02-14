type Bus = usize;
type ArrivalTime = usize;
type WaitTime = usize;

fn from_input(input: &String) -> (ArrivalTime, Vec<Bus>) {
    let mut lines = input.lines();
    (
        lines.next().unwrap().parse::<usize>().unwrap(),
        lines
            .next()
            .unwrap()
            .split(",")
            .filter(|c| *c != "x")
            .map(|c| c.parse::<usize>().unwrap())
            .collect(),
    )
}

fn get_earliest_bus(arrival_time: ArrivalTime, busses: Vec<Bus>) -> (Bus, WaitTime) {
    let mut best_bus = busses[0];
    let mut wait_time = best_bus;
    for bus in busses[1..].iter() {
        let min_of_both = best_bus.min(*bus);
        for i in 0..min_of_both.min(wait_time) {
            let best_bus_wins = ((arrival_time + i) % best_bus) == 0;
            let bus_wins = ((arrival_time + i) % bus) == 0;
            if best_bus_wins {
                break;
            }
            if bus_wins {
                wait_time = i;
                best_bus = *bus;
                break;
            }
        }
    }
    (best_bus, wait_time)
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
    fn check_day_13_star1_example_input_values() {
        let file = read_file("./src/day_13/example.txt");
        let (arrival_time, busses) = from_input(&file);
        assert_eq!(939, arrival_time);
        assert_eq!(vec![7, 13, 59, 31, 19], busses);
    }

    #[test]
    fn check_day_13_star1_example() {
        let file = read_file("./src/day_13/example.txt");
        let (arrival_time, busses) = from_input(&file);
        let (earliest, wait_time) = get_earliest_bus(arrival_time, busses);
        assert_eq!(59, earliest);
        assert_eq!(5, wait_time);
        assert_eq!(295, earliest * wait_time);
    }

    #[test]
    fn check_day_13_star1_input() {
        let file = read_file("./src/day_13/input.txt");
        let (arrival_time, busses) = from_input(&file);
        let (earliest, wait_time) = get_earliest_bus(arrival_time, busses);
        assert_eq!(601, earliest);
        assert_eq!(6, wait_time);
        assert_eq!(3606, earliest * wait_time);
    }
}
