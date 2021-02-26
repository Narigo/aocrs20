use std::collections::HashMap;

type Bitmask = String;

trait BitmaskSystem {
    fn use_mask(&mut self, mask: Bitmask) -> &Self;
    fn write_value_to_memory(&mut self, memory_slot: usize, value: usize) -> &Self;
    fn get_sum_in_memory(&self) -> usize;
}

#[derive(Clone)]
struct Memory {
    current_mask: Bitmask,
    values: HashMap<usize, usize>,
}

impl BitmaskSystem for Memory {
    fn use_mask(&mut self, mask: Bitmask) -> &Self {
        self.current_mask = mask;
        self
    }
    fn write_value_to_memory(&mut self, memory_slot: usize, value: usize) -> &Self {
        let in_binary = format!("{:0>36b}", value);
        let mut digits: Vec<char> = Vec::new();
        for (index, c) in self.current_mask.chars().enumerate() {
            let next_number = match c {
                '0' => '0',
                '1' => '1',
                _ => in_binary.chars().nth(index).unwrap(),
            };
            digits.push(next_number);
        }
        let num: String = digits.into_iter().collect();
        let value = usize::from_str_radix(&num, 2).unwrap();
        self.values.insert(memory_slot, value);
        self
    }
    fn get_sum_in_memory(&self) -> usize {
        let mut sum = 0;
        for (k, v) in self.values.iter() {
            sum += v;
        }
        sum
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

#[derive(Clone)]
struct MemoryV2 {
    current_mask: Bitmask,
    values: HashMap<usize, usize>,
}

impl BitmaskSystem for MemoryV2 {
    fn use_mask(&mut self, mask: Bitmask) -> &Self {
        self.current_mask = mask;
        self
    }
    fn write_value_to_memory(&mut self, memory_slot: usize, value: usize) -> &Self {
        let in_binary = format!("{:0>36b}", memory_slot);
        println!("in_binary={}", in_binary);
        println!("mask     ={}", self.current_mask);
        let mut memory_slots: Vec<Vec<char>> = Vec::new();
        memory_slots.push(Vec::new());
        for (index, c) in self.current_mask.chars().enumerate() {
            match c {
                '0' => {
                    for slot in memory_slots.iter_mut() {
                        slot.push(in_binary.chars().nth(index).unwrap());
                    }
                }
                '1' => {
                    for slot in memory_slots.iter_mut() {
                        slot.push('1');
                    }
                }
                _ => {
                    let mut new_slots = memory_slots.clone();
                    for slot in memory_slots.iter_mut() {
                        slot.push('0');
                    }
                    for slot in new_slots.iter_mut() {
                        slot.push('1');
                    }
                    memory_slots.extend(new_slots);
                }
            }
        }
        for slot in memory_slots.iter() {
            let num: String = slot.into_iter().collect();
            let mem_address = usize::from_str_radix(&num, 2).unwrap();
            println!("result   ={} -> {}", mem_address, value);
            self.values.insert(mem_address, value);
        }
        self
    }
    fn get_sum_in_memory(&self) -> usize {
        let mut sum = 0;
        for (k, v) in self.values.iter() {
            println!("k={} v={}", k, v);
            sum += v;
        }
        sum
    }
}

fn process_input_v2(input: &str) -> MemoryV2 {
    let mut mem = MemoryV2 {
        current_mask: "000000000000000000000000000000000000".to_owned(),
        values: HashMap::new(),
    };
    for line in input.lines() {
        match line_to_command(line) {
            Command::Mask(mask) => mem.use_mask(mask),
            Command::SetMemory(slot, value) => mem.write_value_to_memory(slot, value),
        };
    }
    mem
}

fn process_input(input: &str) -> Memory {
    let mut mem = Memory {
        current_mask: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_owned(),
        values: HashMap::new(),
    };
    for line in input.lines() {
        match line_to_command(line) {
            Command::Mask(mask) => mem.use_mask(mask),
            Command::SetMemory(slot, value) => mem.write_value_to_memory(slot, value),
        };
    }
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
        let memory = process_input(&file);
        assert_eq!(165, memory.get_sum_in_memory());
    }

    #[test]
    fn check_day_14_star1_input() {
        let file = read_file("./src/day_14/input.txt");
        let memory = process_input(&file);
        assert_eq!(13496669152158, memory.get_sum_in_memory());
    }

    #[test]
    fn check_day_14_star2_example() {
        let file = read_file("./src/day_14/example_star2.txt");
        let memory = process_input_v2(&file);
        assert_eq!(208, memory.get_sum_in_memory());
    }

    #[test]
    fn check_day_14_star2_input() {
        let file = read_file("./src/day_14/input.txt");
    }
}
