use std::fmt;

#[derive(Debug, PartialEq)]
enum Command {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    Forward(usize),
    Left(usize),
    Right(usize),
}

fn input_to_commands(input: &String) -> Vec<Command> {
    let mut commands: Vec<Command> = Vec::new();
    use Command::*;
    for line in input.lines() {
        commands.push(Forward(10));
    }
    commands
}

enum Direction {
    North,
    South,
    East,
    West,
}

struct Ship {
    facing: Direction,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::*;

    #[test]
    fn check_day_12_star1_example_1_commands() {
        let file = read_file("./src/day_12/example.txt");
        let commands = input_to_commands(&file);
        assert_eq!(5, commands.len());
    }

    #[test]
    fn check_day_12_star1_input() {
        let file = read_file("./src/day_12/input.txt");
        let commands = input_to_commands(&file);
        assert_eq!(768, commands.len());
    }
}
