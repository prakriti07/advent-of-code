use crate::input::Input;

type Word = i32;

#[derive(Copy, Clone)]
enum Instruction {
    Acc(Word),
    Jmp(Word),
    Nop(Word),
}

struct Computer {
    instructions: Vec<Instruction>,
    accumulator: Word,
    instruction_pointer: Word,
}

impl Computer {
    fn parse(program_text: &str) -> Result<Self, String> {
        let instructions = program_text
            .lines()
            .enumerate()
            .map(|(line_idx, line)| {
                if line.len() < 6 {
                    return Err(format!(
                        "Line {}: Too short line ({})",
                        line_idx + 1,
                        line.len()
                    ));
                }

                let argument = line[4..]
                    .parse::<Word>()
                    .map_err(|e| format!("Line {}: Cannot parse argument - {}", line_idx + 1, e))?;

                Ok(match &line[0..3] {
                    "acc" => Instruction::Acc(argument),
                    "jmp" => Instruction::Jmp(argument),
                    "nop" => Instruction::Nop(argument),
                    _ => {
                        return Err(format!(
                            "Line {}: Invalid line not starting with acc/jmp/nop",
                            line_idx + 1
                        ));
                    }
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            instructions,
            accumulator: 0,
            instruction_pointer: 0,
        })
    }

    fn execute_instruction(&mut self) -> Result<(), String> {
        if self.has_exited() {
            return Err("Cannot executed an exited program".to_string());
        }
        match self.instructions[self.instruction_pointer as usize] {
            Instruction::Acc(parameter) => {
                self.accumulator += parameter;
            }
            Instruction::Jmp(parameter) => {
                self.instruction_pointer += parameter;
                return Ok(());
            }
            Instruction::Nop(_) => {}
        };
        self.instruction_pointer += 1;
        Ok(())
    }

    fn has_exited(&self) -> bool {
        self.instruction_pointer < 0 || self.instruction_pointer >= self.instructions.len() as Word
    }
}

fn check_if_exits(computer: &mut Computer) -> Result<bool, String> {
    let mut executed_instructions = vec![false; computer.instructions.len()];
    while !computer.has_exited() && !executed_instructions[computer.instruction_pointer as usize] {
        executed_instructions[computer.instruction_pointer as usize] = true;
        computer.execute_instruction()?;
    }
    Ok(computer.has_exited())
}

pub fn solve(input: &mut Input) -> Result<Word, String> {
    let mut computer = Computer::parse(input.text)?;
    if input.is_part_one() {
        check_if_exits(&mut computer)?;
        Ok(computer.accumulator)
    } else {
        for i in 0..computer.instructions.len() {
            let instruction = computer.instructions[i];
            match instruction {
                Instruction::Jmp(parameter) | Instruction::Nop(parameter) => {
                    computer.instructions[i] = if matches!(instruction, Instruction::Jmp(_)) {
                        Instruction::Nop(parameter)
                    } else {
                        Instruction::Jmp(parameter)
                    };

                    if check_if_exits(&mut computer)? {
                        return Ok(computer.accumulator);
                    }

                    computer.instruction_pointer = 0;
                    computer.accumulator = 0;
                    computer.instructions[i] = instruction;
                }
                _ => {
                    continue;
                }
            }
        }
        Err("No instruction modification causes program to exit".to_string())
    }
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let example = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    test_part_one!(example => 5);
    test_part_two!(example => 8);

    let real_input = include_str!("day08_input.txt");
    test_part_one!(real_input => 1684);
    test_part_two!(real_input => 2188);
}
