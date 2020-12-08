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

    let mut state = (0i32, 0i32); // pc, acc
    let mut visited_instructions = HashSet::new();
    let mut next_state;
    let mut state_history = vec![];
    loop {
        let instruction = &instructions[state.0 as usize];
        if visited_instructions.contains(&state.0) {
            break;
        }

        visited_instructions.insert(state.0);
        match instruction {
            (InstructionType::Nop, _) => next_state = (state.0 + 1, state.1),
            (InstructionType::Acc, value) => next_state = (state.0 + 1, state.1 + value),
            (InstructionType::Jmp, value) => next_state = (state.0 + value, state.1),
        }

        state_history.push(state);
        state = next_state;
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
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
