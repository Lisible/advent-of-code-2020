use std::collections::hash_map::DefaultHasher;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let file = File::open("input").map_err(|e| Error::InputFileOpenError(e))?;
    let reader = BufReader::new(file);
    let decks = parse_decks(reader)?;
    let mut game = RecursiveCombatGame::with_decks(decks, 1);
    while let GameState::Continue = game.state() {
        game.play_round()?;
    }
    game.print_post_game_results();

    Ok(())
}

fn parse_decks(reader: impl BufRead) -> Result<Vec<Deck>, Error> {
    Ok(reader.lines().try_fold(vec![], |mut acc, line| {
        let line = line.map_err(|e| Error::InputFileReadError(e))?;

        if line.is_empty() {
            return Ok(acc);
        }

        if line.starts_with("Player") {
            acc.push(Deck(VecDeque::new()));
            return Ok(acc);
        }

        let deck: &mut Deck = acc.last_mut().ok_or(Error::LastDeckNotFound)?;
        deck.0.push_back(line.parse()?);
        Ok(acc)
    })?)
}

struct RecursiveCombatGame {
    players_decks: Vec<Deck>,
    round: usize,
    state: GameState,
    game_number: usize,
    previous_decks_hashes: Vec<u64>,
}

impl RecursiveCombatGame {
    pub fn with_decks(players_decks: Vec<Deck>, game_number: usize) -> Self {
        Self {
            players_decks,
            round: 1,
            state: GameState::Continue,
            game_number,
            previous_decks_hashes: vec![],
        }
    }

    pub fn play_round(&mut self) -> Result<(), Error> {
        self.print_round_infos();
        let mut hasher = DefaultHasher::new();
        self.players_decks.hash(&mut hasher);
        let decks_hash = hasher.finish();
        if self.previous_decks_hashes.contains(&decks_hash) {
            self.state = GameState::GameOver(0);
            return Ok(());
        }

        let mut played_cards = vec![];
        for player in 0..self.players_decks.len() {
            let played_card = self.players_decks[player]
                .0
                .pop_front()
                .ok_or(Error::PlayerHasNoCardToPlay)?;

            println!("Player {} plays: {}", player + 1, played_card);
            played_cards.push((player, played_card));
        }

        let mut winning_player = 0;
        if self
            .players_decks
            .iter()
            .enumerate()
            .all(|(i, d)| d.0.len() >= played_cards[i].1 .0 as usize)
        {
            println!("\nPlaying a sub-game to determine the winner...");
            let mut decks = self.players_decks.clone();
            for deck in 0..decks.len() {
                let card_count = played_cards[deck].1 .0;
                for _ in 0..decks[deck].0.len() - card_count as usize {
                    decks[deck].0.pop_back();
                }
            }

            let mut sub_game = RecursiveCombatGame::with_decks(decks, self.game_number + 1);
            winning_player = loop {
                sub_game.play_round();
                if let GameState::GameOver(winner) = sub_game.state() {
                    break winner;
                }
            };
            println!("...Anyway back to game {}", self.game_number);
        } else {
            winning_player = played_cards
                .iter()
                .enumerate()
                .fold(
                    (0, 0),
                    |acc, (i, c)| if acc.1 < c.1 .0 { (i, c.1 .0) } else { acc },
                )
                .0;
        }

        println!(
            "Player {} wins round {} of game {}!\n",
            winning_player + 1,
            self.round,
            self.game_number
        );
        self.players_decks[winning_player]
            .0
            .push_back(played_cards[winning_player].1.clone());
        self.players_decks[winning_player]
            .0
            .push_back(played_cards[(winning_player + 1) % 2].1.clone());

        self.previous_decks_hashes.push(decks_hash);
        self.round += 1;

        for (player, player_deck) in self.players_decks.iter().enumerate() {
            if player_deck.0.is_empty() {
                self.state = GameState::GameOver((player + 1) % 2)
            }
        }

        Ok(())
    }

    pub fn state(&self) -> GameState {
        self.state
    }

    pub fn print_round_infos(&self) {
        println!("-- Round {} (Game {}) --", self.round, self.game_number);
        for player in 0..self.players_decks.len() {
            println!(
                "Player {}'s deck: {}",
                player + 1,
                self.players_decks[player]
            );
        }
    }

    pub fn print_post_game_results(&self) {
        println!("== Post-game results ==");
        for player in 0..self.players_decks.len() {
            println!(
                "Player {}'s deck: {}",
                player + 1,
                self.players_decks[player]
            );

            if !self.players_decks[player].0.is_empty() {
                println!(
                    "score: {}",
                    self.players_decks[player]
                        .0
                        .iter()
                        .rev()
                        .enumerate()
                        .fold(0u64, |acc, (i, c)| { acc + (i as u64 + 1) * c.0 as u64 })
                );
            }
        }
    }
}

#[derive(Debug)]
struct CombatGame {
    players_decks: Vec<Deck>,
    round: usize,
    state: GameState,
}

impl CombatGame {
    pub fn with_decks(players_decks: Vec<Deck>) -> Self {
        Self {
            players_decks,
            round: 1,
            state: GameState::Continue,
        }
    }

    pub fn play_round(&mut self) -> Result<(), Error> {
        self.print_round_infos();
        let mut played_cards = vec![];
        for player in 0..self.players_decks.len() {
            let played_card = self.players_decks[player]
                .0
                .pop_front()
                .ok_or(Error::PlayerHasNoCardToPlay)?;

            println!("Player {} plays: {}", player + 1, played_card);
            played_cards.push((player, played_card));
        }

        played_cards.sort_by_key(|(_, card)| card.0);
        let winning = &played_cards[played_cards.len() - 1];
        println!("Player {} wins the round!\n", winning.0 + 1);

        self.players_decks[winning.0]
            .0
            .append(&mut played_cards.into_iter().map(|a| a.1).rev().collect());

        self.round += 1;

        for (player, player_deck) in self.players_decks.iter().enumerate() {
            if player_deck.0.is_empty() {
                self.state = GameState::GameOver(player)
            }
        }

        Ok(())
    }

    pub fn state(&self) -> GameState {
        self.state
    }

    pub fn print_round_infos(&self) {
        println!("-- Round {} --", self.round);
        for player in 0..self.players_decks.len() {
            println!(
                "Player {}'s deck: {}",
                player + 1,
                self.players_decks[player]
            );
        }
    }

    pub fn print_post_game_results(&self) {
        println!("== Post-game results ==");
        for player in 0..self.players_decks.len() {
            println!(
                "Player {}'s deck: {}",
                player + 1,
                self.players_decks[player]
            );

            if !self.players_decks[player].0.is_empty() {
                println!(
                    "score: {}",
                    self.players_decks[player]
                        .0
                        .iter()
                        .rev()
                        .enumerate()
                        .fold(0, |acc, (i, c)| { acc + (i as u16 + 1) * c.0 as u16 })
                );
            }
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum GameState {
    Continue,
    GameOver(usize),
}

#[derive(Debug, Hash, Clone)]
struct Card(u8);

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Card(u8::from_str(s).map_err(|e| Error::ParseCardError(e))?))
    }
}

#[derive(Debug, Hash, Clone)]
struct Deck(VecDeque<Card>);

impl Display for Deck {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|c| c.0.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

#[derive(Debug)]
enum Error {
    InputFileOpenError(std::io::Error),
    InputFileReadError(std::io::Error),
    ParseCardError(std::num::ParseIntError),
    LastDeckNotFound,
    PlayerHasNoCardToPlay,
}
