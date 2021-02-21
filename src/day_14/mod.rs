use std::collections::HashMap;

type Bitmask = String;

trait BitmaskSystem {
    fn use_mask(&mut self, mask: Bitmask) -> Self;
    fn write_value_to_memory(&mut self, memory_slot: usize, value: usize) -> Self;
    fn get_sum_in_memory(&self) -> usize;
}

#[derive(Clone)]
struct Memory {
    values: HashMap<usize, usize>,
}

impl BitmaskSystem for Memory {
    fn use_mask(&mut self, mask: Bitmask) -> Self {
        self.clone()
    }
    fn write_value_to_memory(&mut self, memory_slot: usize, value: usize) -> Self {
        self.clone()
    }
    fn get_sum_in_memory(&self) -> usize {
        0
    }
}

#[derive(Debug, Clone)]
enum Command {
    Mask(String),
    SetMemory(usize, usize),
}

fn line_to_command(input: &str) -> Command {
    let command = &input[0..4];
    if command == "mask" {
        let mask = &input[7..];
        return Command::Mask(mask.to_owned());
    }
    let mem_split: Vec<&str> = input.split("]").collect();
    let num = mem_split.get(0).unwrap()[4..].to_owned();
    Command::SetMemory(
        num.parse::<usize>().unwrap(),
        mem_split.get(1).unwrap()[3..].parse::<usize>().unwrap(),
    )
}

fn from_input(input: &str) -> Memory {
    let mut mem = Memory {
        values: HashMap::new(),
    };
    for line in input.lines() {}
    mem
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::*;

    #[test]
    fn check_day_14() {}

    #[test]
    fn check_day_14_star1() {}

    #[test]
    fn check_day_14_star2() {}

    #[test]
    fn check_day_14_star1_example() {
        let file = read_file("./src/day_14/example.txt");
    }

    #[test]
    fn check_day_14_star1_input() {
        let file = read_file("./src/day_14/input.txt");
    }

    #[test]
    fn check_day_14_star2_example() {
        let file = read_file("./src/day_14/example.txt");
    }

    #[test]
    fn check_day_14_star2_input() {
        let file = read_file("./src/day_14/input.txt");
    }
}
