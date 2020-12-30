enum Instruction {
    Acc,
    Jmp,
    Nop,
}

fn line_to_instruction(input: &str) -> Instruction {
    Instruction::Nop
}

fn file_to_instructions(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in input.lines() {
        let instruction = line_to_instruction(line);
        instructions.push(instruction);
    }
    instructions
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::*;

    #[test]
    fn check_example() {
        let file = read_file("./src/day_08/example.txt");
        let instructions = file_to_instructions(&file);
        assert_eq!(8, instructions.len());
    }
}
