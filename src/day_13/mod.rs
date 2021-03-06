type Bus = usize;
type ArrivalTime = usize;
type StartTime = usize;
type WaitTime = usize;

fn from_input(input: &String) -> (ArrivalTime, Vec<Option<Bus>>) {
    let mut lines = input.lines();
    (
        lines.next().unwrap().parse::<usize>().unwrap(),
        lines
            .next()
            .unwrap()
            .split(",")
            .map(|c| c.parse::<usize>().ok())
            .collect(),
    )
}

fn get_earliest_bus(arrival_time: ArrivalTime, busses: Vec<Option<Bus>>) -> (Bus, WaitTime) {
    let mut best_bus = busses[0].unwrap();
    let mut wait_time = best_bus;
    for bus in busses[1..]
        .iter()
        .filter(|c| c.is_some())
        .map(|c| c.unwrap())
    {
        let min_of_both = best_bus.min(bus);
        for i in 0..min_of_both.min(wait_time) {
            let best_bus_wins = ((arrival_time + i) % best_bus) == 0;
            let bus_wins = ((arrival_time + i) % bus) == 0;
            if best_bus_wins {
                break;
            }
            if bus_wins {
                wait_time = i;
                best_bus = bus;
                break;
            }
        }
    }
    (best_bus, wait_time)
}

fn get_earliest_time(busses: Vec<Option<Bus>>) -> StartTime {
    let mut i = 0;
    let mut min_next_jump = busses.first().unwrap().unwrap_or(1);
    for ptr in 1..busses.len() {
        loop {
            if busses.get(ptr).is_none() {
                break;
            }
            let found = busses[0..(ptr + 1)]
                .iter()
                .enumerate()
                .all(|(index, bus)| match bus {
                    Some(id) => (i + index) % id == 0,
                    None => true,
                });

            if found {
                min_next_jump = busses[0..(ptr + 1)].iter().fold(1, |acc, bus| match bus {
                    Some(id) => acc * id,
                    None => acc,
                });
                break;
            }
            i += min_next_jump;
        }
    }
    i
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
        assert_eq!(
            vec![
                Some(7),
                Some(13),
                None,
                None,
                Some(59),
                None,
                Some(31),
                Some(19)
            ],
            busses
        );
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

    #[test]
    fn check_day_13_star2_example() {
        let file = read_file("./src/day_13/example.txt");
        let (_, busses) = from_input(&file);
        let time = get_earliest_time(busses);
        assert_eq!(1068781, time);
    }

    #[test]
    fn check_day_13_star2_example_2() {
        let file = read_file("./src/day_13/example_star2_2.txt");
        let (_, busses) = from_input(&file);
        let time = get_earliest_time(busses);
        assert_eq!(754018, time);
    }

    #[test]
    fn check_day_13_star2_example_3() {
        let file = read_file("./src/day_13/example_star2_3.txt");
        let (_, busses) = from_input(&file);
        let time = get_earliest_time(busses);
        assert_eq!(779210, time);
    }

    #[test]
    fn check_day_13_star2_example_4() {
        let file = read_file("./src/day_13/example_star2_4.txt");
        let (_, busses) = from_input(&file);
        let time = get_earliest_time(busses);
        assert_eq!(1261476, time);
    }

    #[test]
    fn check_day_13_star2_example_5() {
        let file = read_file("./src/day_13/example_star2_5.txt");
        let (_, busses) = from_input(&file);
        let time = get_earliest_time(busses);
        assert_eq!(1202161486, time);
    }

    #[test]
    fn check_day_13_star2_input() {
        let file = read_file("./src/day_13/input.txt");
        let (_, busses) = from_input(&file);
        let time = get_earliest_time(busses);
        assert_eq!(379786358533423, time);
    }
}
