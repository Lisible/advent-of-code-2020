use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let file = File::open("input").map_err(|e| Error::InputFileOpenError(e))?;
    let reader = BufReader::new(file);
    let decks = parse_decks(reader)?;
    let mut game = Game::with_decks(decks);
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

#[derive(Debug)]
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

#[derive(Debug)]
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
struct Game {
    players_decks: Vec<Deck>,
    round: usize,
    state: GameState,
}

impl Game {
    pub fn with_decks(players_decks: Vec<Deck>) -> Self {
        Self {
            players_decks,
            round: 1,
            state: GameState::Continue,
        }
    }

    pub fn play_round(&mut self) -> Result<(), Error> {
        println!("-- Round {} --", self.round);
        for player in 0..self.players_decks.len() {
            println!(
                "Player {}'s deck: {}",
                player + 1,
                self.players_decks[player]
            );
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

        played_cards.sort_by_key(|(_, card)| card.0);
        let winning = &played_cards[played_cards.len() - 1];
        println!("Player {} wins the round!\n", winning.0 + 1);

        self.players_decks[winning.0]
            .0
            .append(&mut played_cards.into_iter().map(|a| a.1).rev().collect());

        self.round += 1;
        if self.players_decks.iter().any(|d| d.0.is_empty()) {
            self.state = GameState::GameOver
        }

        Ok(())
    }

    pub fn state(&self) -> GameState {
        self.state
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
    GameOver,
}

#[derive(Debug)]
enum Error {
    InputFileOpenError(std::io::Error),
    InputFileReadError(std::io::Error),
    ParseCardError(std::num::ParseIntError),
    LastDeckNotFound,
    PlayerHasNoCardToPlay,
}
