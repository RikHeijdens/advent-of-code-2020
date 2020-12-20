use std::collections::HashSet;
use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    // Increases, or decreases the program's accumulator with the given argument.
    ACC,
    // Updates the program counter to a new offset, where the offset is given by the argument.
    JMP,
    // No operation.
    NOP,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Instruction {
    operation: Operation,
    argument: i32,
}

impl Instruction {
    pub fn from_reader<R: BufRead>(reader: &mut R) -> Vec<Instruction> {
        let mut instructions: Vec<Instruction> = Vec::new();
        loop {
            let mut buffer = String::new();
            reader
                .read_line(&mut buffer)
                .expect("Expected to read data from reader.");

            let line = buffer.trim();
            if line.is_empty() {
                break;
            }

            match Instruction::from_str(line) {
                Ok(instruction) => instructions.push(instruction),
                Err(_) => {
                    eprintln!("Failed to parse '{}' as an instruction.", line)
                }
            }
        }
        instructions
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split(' ');
        let operation;
        let argument: i32;

        // Attempt to parse the operation
        match splits.next() {
            Some("acc") => operation = Operation::ACC,
            Some("jmp") => operation = Operation::JMP,
            Some("nop") => operation = Operation::NOP,
            Some(op) => return Err(format!("Unknown instruction: '{}'", op)),
            _ => {
                return Err(format!("Unable to parse instruction: '{}'", s));
            }
        }

        // Attempt to parse the argument
        match splits.next() {
            Some(a) => match a.parse::<i32>() {
                Ok(value) => argument = value,
                Err(_) => return Err(format!("Failed to parse argument '{}' as i32.", a)),
            },
            _ => return Err(format!("Failed to parse argument from '{}'", s)),
        }

        Ok(Instruction {
            operation,
            argument,
        })
    }
}

pub struct Processor {
    accumulator: i32,
    program_counter: i32,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            accumulator: 0,
            program_counter: 0,
        }
    }

    /// Processes a list of instructions until a cycle is detected, as soon as a cycle has
    /// been detected, the current value in the accumulator will be returned.
    ///
    /// If no loop will be found -1 will be returned instead of the value in the accumulator.
    pub fn find_cycle(&mut self, instructions: &[Instruction]) -> (bool, i32) {
        // Keep track of which instructions we've executed in order to detect the cycle.
        let mut executed_instructions: HashSet<i32> = HashSet::new();

        // Reset the program counter before we begin.
        self.program_counter = 0;

        // Execute the instructions.
        loop {
            if self.program_counter as usize >= instructions.len() {
                // Program terminated without finding a loop.
                return (false, self.accumulator);
            }

            // Fetch an instruction
            let instruction = &instructions[self.program_counter as usize];

            // Check if the instruction has been executed previously.
            if executed_instructions.contains(&self.program_counter) {
                // We've found the cycle, return the value of the accumulator.
                return (true, self.accumulator);
            }

            // Track which instructions were executed.
            executed_instructions.insert(self.program_counter);

            // Execute the instruction.
            self.execute_instruction(&instruction);
        }
    }

    /// Executes a single instruction.
    fn execute_instruction(&mut self, instruction: &Instruction) {
        // Execute the instruction.
        match instruction.operation {
            Operation::ACC => {
                self.accumulator += instruction.argument;
                self.program_counter += 1
            }
            Operation::JMP => self.program_counter += instruction.argument,
            Operation::NOP => self.program_counter += 1,
        }
    }
}

impl Default for Processor {
    fn default() -> Self {
        Self::new()
    }
}

/// Finds the fix in the boot code by testing updating instructions.
///
/// Returns -1 if a solution could not be found, or the value of the accumulator after program
/// termination if a solution could be found.
pub fn find_fix(instructions: &[Instruction]) -> i32 {
    // In order to find the fix, we need to find an instruction that needs to be changed from
    // "jmp" to "nop" or vice versa. In order to do this we'll simply convert a nop statement
    // into a jmp statement, or vice versa and test whether the program works as expected.
    for (idx, instruction) in instructions.iter().enumerate() {
        if instruction.operation == Operation::ACC {
            // ACC operations were not corrupted.
            continue;
        }
        // Clone the list of instructions
        let mut modified_instructions = instructions.to_owned();
        // Modify the instruction at index.
        let instruction = &modified_instructions[idx];
        match instruction.operation {
            Operation::JMP => {
                modified_instructions[idx] = Instruction {
                    operation: Operation::NOP,
                    argument: instruction.argument,
                }
            }
            Operation::NOP => {
                modified_instructions[idx] = Instruction {
                    operation: Operation::JMP,
                    argument: instruction.argument,
                }
            }
            _ => {}
        }

        // Test whether the program works as expected.
        let mut processor = Processor::new();
        if let (false, acc) = processor.find_cycle(&modified_instructions) {
            return acc;
        }
    }
    // No solution
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    fn read_instructions() -> Vec<Instruction> {
        let instructions_input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";
        let mut reader = BufReader::new(instructions_input.as_bytes());
        let instructions = Instruction::from_reader(&mut reader);
        return instructions;
    }

    #[test]
    fn test_parsing() {
        let instructions = read_instructions();
        assert_eq!(instructions.len(), 9);
        assert_eq!(
            instructions[0],
            Instruction {
                operation: Operation::NOP,
                argument: 0
            }
        );
        assert_eq!(
            instructions[5],
            Instruction {
                operation: Operation::ACC,
                argument: -99
            }
        );
        assert_eq!(
            instructions[7],
            Instruction {
                operation: Operation::JMP,
                argument: -4
            }
        );
    }

    #[test]
    fn test_instruction() {
        assert_eq!(
            Instruction::from_str("acc +1").unwrap(),
            Instruction {
                operation: Operation::ACC,
                argument: 1
            }
        );
    }

    #[test]
    fn test_find_cycle() {
        let instructions = read_instructions();
        let mut processor = Processor::new();
        assert_eq!(processor.find_cycle(&instructions), (true, 5));
    }

    #[test]
    fn test_find_fix() {
        let instructions = read_instructions();
        assert_eq!(find_fix(&instructions), 8);
    }
}
