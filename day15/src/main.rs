use std::collections::HashMap;

const STARTING_NUMBERS: [u32; 6] = [1, 20, 8, 12, 0, 14];

fn main() {
    let mut turn_iterator = TurnIterator::new();

    println!(
        "{}",
        turn_iterator
            .nth(30000000 - STARTING_NUMBERS.len() - 1)
            .expect("No next")
    );
}

struct TurnIterator {
    last_turns: HashMap<u32, u32>,
    turn: u32,
    last_number: u32,
}

impl TurnIterator {
    pub fn new() -> Self {
        let mut last_turns = HashMap::new();
        for (i, &number) in STARTING_NUMBERS.iter().enumerate() {
            if i == STARTING_NUMBERS.len() - 1 {
                break;
            }

            last_turns.insert(number, (i + 1) as u32);
        }

        Self {
            last_turns,
            turn: STARTING_NUMBERS.len() as u32,
            last_number: STARTING_NUMBERS[STARTING_NUMBERS.len() - 1],
        }
    }
}

impl Iterator for TurnIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let l = self.last_number;
        if let Some(&last) = self.last_turns.get(&self.last_number) {
            self.last_number = self.turn - last;
        } else {
            self.last_number = 0;
        }

        self.last_turns.insert(l, self.turn);
        self.turn += 1;
        Some(self.last_number)
    }
}
