fn main() {
    println!("{}", compute_encryption_key(5764801, 17807724));
}

fn compute_encryption_key(card_public_key: u64, door_public_key: u64) -> u64 {
    let card_loop_size = find_loop_size(7, card_public_key);
    let door_loop_size = find_loop_size(7, door_public_key);
    println!("{}, {}", card_loop_size, door_loop_size);

    let card_encryption_key = transform_subject_number(door_public_key, card_loop_size);
    let door_encryption_key = transform_subject_number(card_public_key, door_loop_size);
    assert_eq!(
        card_encryption_key, door_encryption_key,
        "Card calculated encryption key != Door calculated encryption key"
    );
    card_encryption_key
}

fn find_loop_size(initial_subject_number: u64, card_public_key: u64) -> u64 {
    let mut loop_size = 1;
    loop {
        let n = transform_subject_number(initial_subject_number, loop_size);
        if n == card_public_key {
            return loop_size;
        }

        loop_size += 1;
    }
}

fn transform_subject_number(subject_number: u64, loop_size: u64) -> u64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value = value * subject_number;
        value = value % 20201227;
    }

    value
}
