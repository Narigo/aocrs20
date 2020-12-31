use std::collections::HashSet;

#[derive(Debug)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop,
}

fn line_to_instruction(input: &str) -> Instruction {
    let command = &input[0..3];
    let arg = &input[4..];
    match command {
        "acc" => Instruction::Acc(arg.parse::<i32>().expect("Should have an integer")),
        "jmp" => Instruction::Jmp(arg.parse::<i32>().expect("Should have an integer")),
        "nop" => Instruction::Nop,
        _ => panic!("Got a non acc, jmp or nop command!"),
    }
}

fn file_to_instructions(input: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in input.lines() {
        let instruction = line_to_instruction(line);
        instructions.push(instruction);
    }
    instructions
}

fn run_instructions_until_loop(instructions: Vec<Instruction>) -> i32 {
    let mut state: i32 = 0;
    let mut visited_lines: HashSet<usize> = HashSet::new();
    let mut current_line: usize = 0;
    loop {
        let instruction = instructions
            .get(current_line as usize)
            .expect("Should have a line");
        visited_lines.insert(current_line);
        match instruction {
            Instruction::Acc(count) => {
                state = state + count;
                current_line = current_line + 1;
            }
            Instruction::Jmp(count) => {
                current_line = (count + (current_line as i32)) as usize;
            }
            Instruction::Nop => {
                current_line = current_line + 1;
            }
        }
        if visited_lines.contains(&current_line) {
            break;
        }
    }
    state
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::*;

    #[test]
    fn check_example() {
        let file = read_file("./src/day_08/example.txt");
        let instructions = file_to_instructions(&file);
        let last_state = run_instructions_until_loop(instructions);
        assert_eq!(5, last_state);
    }
}
