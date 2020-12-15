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
    numbers: Vec<u32>,
    turn: u32,
}

impl TurnIterator {
    pub fn new() -> Self {
        let mut last_turns: HashMap<u32, u32> = HashMap::new();
        for (i, &number) in STARTING_NUMBERS.iter().enumerate() {
            if i == STARTING_NUMBERS.len() - 1 {
                break;
            }

            *last_turns.entry(number).or_default() = (i + 1) as u32;
        }

        Self {
            last_turns,
            numbers: STARTING_NUMBERS.to_vec(),
            turn: STARTING_NUMBERS.len() as u32,
        }
    }
}

impl Iterator for TurnIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(&last_turn) = self.last_turns.get(&self.numbers[self.turn as usize - 1]) {
            self.numbers.push(self.turn - last_turn);
        } else {
            self.numbers.push(0);
        }

        *self
            .last_turns
            .entry(self.numbers[self.turn as usize - 1])
            .or_default() = self.turn;

        self.turn += 1;
        self.numbers.last().cloned()
    }
}
