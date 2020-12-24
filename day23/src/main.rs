fn main() {
    let mut cups = Cups::new(vec![2, 4, 7, 8, 1, 9, 3, 5, 6], 2);

    for _move in 0..100 {
        cups.execute_move();
    }

    let mut cur = 0;
    for _ in 0..cups.next_cups.len() - 1 {
        cur = cups.next_cups[cur];
        print!("{}", cur + 1);
    }
    println!();

    let mut vec = vec![2, 4, 7, 8, 1, 9, 3, 5, 6];
    for i in 0..1_000_000 - 9 {
        vec.push(i + 10);
    }

    let mut cups = Cups::new(vec, 2);
    for _move in 0..10_000_000 {
        cups.execute_move();
    }

    let a = cups.next_cups[0];
    let b = cups.next_cups[a];
    println!("{} * {} = {}", a + 1, b + 1, (a + 1) * (b + 1));
}

struct Cups {
    next_cups: Vec<usize>,
    current_cup_label: usize,
}

impl Cups {
    pub fn new(cups: Vec<usize>, first_label: usize) -> Self {
        let mut next_cups = vec![0; cups.len()];
        for (a, b) in cups.iter().cycle().zip(cups.iter().cycle().skip(1)) {
            next_cups[a - 1] = b - 1;

            if next_cups.iter().filter(|&&a| a == 0).count() == 1 {
                break;
            }
        }

        Self {
            next_cups,
            current_cup_label: first_label - 1,
        }
    }

    pub fn execute_move(&mut self) {
        let (a, b, c) = self.pick_next_3();

        self.next_cups[self.current_cup_label] = self.next_cups[c];
        let mut destination_cup =
            (self.next_cups.len() + self.current_cup_label - 1) % self.next_cups.len();
        while [a, b, c].contains(&destination_cup) {
            destination_cup = (self.next_cups.len() + destination_cup - 1) % self.next_cups.len();
        }

        self.next_cups[c] = self.next_cups[destination_cup];
        self.next_cups[destination_cup] = a;
        self.current_cup_label = self.next_cups[self.current_cup_label];
    }

    fn pick_next_3(&self) -> (usize, usize, usize) {
        let a = self.next_cups[self.current_cup_label];
        let b = self.next_cups[a];
        let c = self.next_cups[b];
        (a, b, c)
    }
}
