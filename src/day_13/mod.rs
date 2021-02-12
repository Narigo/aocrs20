type Bus = usize;
type ArrivalTime = usize;

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

fn get_earliest_bus(arrival_time: ArrivalTime, busses: Vec<Bus>) -> Bus {
    let mut best_bus = busses[0];
    for bus in busses[1..].iter() {
        let time_before = best_bus * (arrival_time / best_bus + 1) % best_bus;
        let time_this = bus * (arrival_time / bus) % bus;
        println!("best_bus = {}, time = {}", best_bus, time_before);
        println!("     bus = {}, time = {}", bus, time_this);
        if time_this < time_before {
            best_bus = *bus;
        }
    }
    best_bus
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
        let earliest = get_earliest_bus(arrival_time, busses);
        assert_eq!(59, earliest);
    }
}
