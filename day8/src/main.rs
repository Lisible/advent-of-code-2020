use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

type Instruction = (InstructionType, i32);
fn main() -> Result<(), Error> {
    let file = File::open("input").map_err(|_| Error::FileNotFound)?;
    let reader = BufReader::new(file);
    let instructions: Vec<Instruction> = reader.lines().try_fold(Vec::new(), |mut acc, line| {
        let line = line.map_err(|_| Error::FileReadError)?;
        let mut split_line = line.split(" ");
        acc.push((
            split_line.next().ok_or(Error::InstructionNotFound)?.into(),
            i32::from_str(split_line.next().ok_or(Error::ArgumentNotFound)?)
                .map_err(|e| Error::ParseArgumentError(e))?,
        ));
        Ok(acc)
    })?;
    let flippable_instructions =
        instructions
            .iter()
            .enumerate()
            .fold(vec![], |mut acc, (i, &(instruction, _))| {
                if instruction == InstructionType::Jmp || instruction == InstructionType::Nop {
                    acc.push(i);
                }

                acc
            });

    for flippable_instruction in flippable_instructions {
        let result = run(&instructions, flippable_instruction);
        println!("is {} swappable ?", flippable_instruction);
        if result.0 as usize == instructions.len() {
            println!("yes: {}", result.1);
            break;
        } else {
            println!("no\n");
        }
    }

    Ok(())
}

fn run(instructions: &Vec<Instruction>, flippable_instruction: usize) -> (i32, i32) {
    let mut pc = 0;
    let mut acc = 0;
    let mut visited_instructions = HashSet::new();
    loop {
        if pc as usize >= instructions.len() || visited_instructions.contains(&pc) {
            break;
        }

        let instruction = if pc as usize == flippable_instruction {
            swap_instruction(pc, instructions)
        } else {
            instructions[pc as usize]
        };
        visited_instructions.insert(pc);
        match instruction {
            (InstructionType::Nop, _) => pc += 1,
            (InstructionType::Acc, value) => {
                acc += value;
                pc += 1
            }
            (InstructionType::Jmp, value) => pc += value,
        }
    }

    (pc, acc)
}

fn swap_instruction(pc: i32, instructions: &Vec<Instruction>) -> Instruction {
    let instruction = instructions[pc as usize];
    match instruction {
        (InstructionType::Nop, value) => (InstructionType::Jmp, value),
        (InstructionType::Jmp, value) => (InstructionType::Nop, value),
        _ => instruction,
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum InstructionType {
    Nop,
    Jmp,
    Acc,
}

impl Into<InstructionType> for &str {
    fn into(self) -> InstructionType {
        match self {
            "nop" => InstructionType::Nop,
            "jmp" => InstructionType::Jmp,
            "acc" => InstructionType::Acc,
            _ => panic!("Unknown instruction"),
        }
    }
}

#[derive(Debug)]
enum Error {
    FileNotFound,
    FileReadError,
    InstructionNotFound,
    ArgumentNotFound,
    ParseArgumentError(ParseIntError),
}
