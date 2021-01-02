use std::collections::HashSet;

#[derive(Debug, Clone)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

fn line_to_instruction(input: &str) -> Instruction {
    let command = &input[0..3];
    let arg = &input[4..];
    match command {
        "acc" => Instruction::Acc(arg.parse::<i32>().expect("Should have an integer")),
        "jmp" => Instruction::Jmp(arg.parse::<i32>().expect("Should have an integer")),
        "nop" => Instruction::Nop(arg.parse::<i32>().expect("Should have an integer")),
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

fn replace_instruction(old_instruction: Instruction) -> Instruction {
    match old_instruction {
        Instruction::Jmp(count) => Instruction::Nop(count),
        Instruction::Nop(count) => Instruction::Jmp(count),
        other => other,
    }
}

fn find_corrupt_instruction(instructions: Vec<Instruction>) -> Result<i32, i32> {
    let mut current_mutated_line: usize = 0;
    let max_line = instructions.len();
    loop {
        let mut copied = instructions.clone();
        let old_instruction = copied[current_mutated_line].clone();
        std::mem::replace(
            &mut copied[current_mutated_line],
            replace_instruction(old_instruction),
        );
        let result = run_instructions_until_loop(copied);
        if result.is_ok() || max_line <= current_mutated_line {
            return result;
        }
        current_mutated_line += 1;
    }
}

fn run_instructions_until_loop(instructions: Vec<Instruction>) -> Result<i32, i32> {
    let mut state: i32 = 0;
    let mut visited_lines: HashSet<usize> = HashSet::new();
    let mut current_line: usize = 0;
    while current_line < instructions.len() {
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
            Instruction::Nop(_) => {
                current_line = current_line + 1;
            }
        }
        if visited_lines.contains(&current_line) {
            return Err(state);
        }
    }
    Ok(state)
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
        assert_eq!(Err(5), last_state);
    }

    #[test]
    fn check_input_star_1() {
        let file = read_file("./src/day_08/input.txt");
        let instructions = file_to_instructions(&file);
        let last_state = run_instructions_until_loop(instructions);
        assert_eq!(Err(1489), last_state);
    }

    #[test]
    fn check_example_star_2() {
        let file = read_file("./src/day_08/example.txt");
        let instructions = file_to_instructions(&file);
        let last_state = find_corrupt_instruction(instructions);
        assert_eq!(Ok(8), last_state);
    }
}
