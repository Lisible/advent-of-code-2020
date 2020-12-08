use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

type Instruction = (InstructionType, i32);

/// This solution is my first attempt to solve the challenge
/// I went for a backtracking solution but when I realized I had to find a way to flip nop
/// it was late and I was tired so I just gave up and wrote
/// the bruteforce solution.
/// This solution will only work if the input file requires a jmp to be flipped to nop.
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

    let part1 = false;
    let mut pc = 0;
    let mut acc = 0;
    let mut visited_instructions = HashSet::new();
    let mut swapped = HashSet::new();
    let mut state_history = vec![];
    let mut step = 0;
    loop {
        if pc as usize >= instructions.len() || (part1 && visited_instructions.contains(&pc)) {
            break;
        }

        step += 1;

        let mut instruction = instructions[pc as usize];
        if !part1 && visited_instructions.contains(&pc) {
            let swappable_state = restore_to_swappable_instruction_state(
                &mut state_history,
                &instructions,
                &mut swapped,
            );
            pc = swappable_state.0;
            acc = swappable_state.1;
            instruction = swap_instruction(pc, &instructions);
        }

        visited_instructions.insert(pc);
        match instruction {
            (InstructionType::Nop, _) => pc += 1,
            (InstructionType::Acc, value) => {
                acc += value;
                pc += 1
            }
            (InstructionType::Jmp, value) => pc += value,
        }
        state_history.push((pc, acc));
    }

    println!("{}", acc);

    Ok(())
}

fn is_instruction_swappable(instruction_type: InstructionType) -> bool {
    instruction_type == InstructionType::Jmp || instruction_type == InstructionType::Nop
}

fn swap_instruction(pc: i32, instructions: &Vec<Instruction>) -> Instruction {
    let instruction = instructions[pc as usize];
    match instruction {
        (InstructionType::Nop, value) => (InstructionType::Jmp, value),
        (InstructionType::Jmp, value) => (InstructionType::Nop, value),
        _ => instruction,
    }
}

fn restore_to_swappable_instruction_state(
    state_history: &mut Vec<(i32, i32)>,
    instructions: &Vec<Instruction>,
    swapped: &mut HashSet<i32>,
) -> (i32, i32) {
    let mut state = state_history.pop().unwrap();
    let mut instruction = instructions[state.0 as usize];
    while !is_instruction_swappable(instruction.0) || swapped.contains(&state.0) {
        state = state_history.pop().unwrap();
        instruction = instructions[state.0 as usize];
    }

    swapped.insert(state.0);
    (
        state_history.last().unwrap().0,
        state_history.last().unwrap().1,
    )
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
