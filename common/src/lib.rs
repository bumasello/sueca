use std::{fmt::Debug, usize};

use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Suit {
    Ouros,
    Espadas,
    Copas,
    Paus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RoomStatus {
    Waiting,
    InGame,
    Finished,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoomSummaryStruct {
    pub room_id: String,
    pub players: Vec<String>,
    pub status: RoomStatus,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameRoomStateStruct {
    pub player: String,
    pub trump: Suit,
    pub status: RoomStatus,
    pub current_trick: Vec<(String, Card)>,
    pub scores: [u32; 2],
    pub players: Vec<String>,
    pub hand: Vec<Card>,
    pub current_turn: String,
    pub lead: Option<Suit>,
}

impl Suit {
    const SUITS: [Suit; 4] = [Suit::Copas, Suit::Espadas, Suit::Ouros, Suit::Paus];

    pub fn as_vec() -> Vec<Suit> {
        Self::SUITS.to_vec()
    }

    pub fn get_trump() -> Suit {
        let suits = Suit::SUITS;
        let mut rng = thread_rng();

        *suits.choose(&mut rng).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Card {
    pub value: u8,
    pub suit: Suit,
}

impl Card {
    pub fn return_shuffled_deck() -> Vec<Card> {
        const EXCL_NUM: [u8; 3] = [8, 9, 10];
        let suits = Suit::as_vec();
        let mut cards: Vec<Card> = Vec::with_capacity(40);

        for suit in suits.iter() {
            for n in 1..=13 {
                if EXCL_NUM.contains(&n) {
                    continue;
                }
                cards.push(Card {
                    value: n,
                    suit: *suit,
                });
            }
        }
        let mut rng = thread_rng();
        cards.shuffle(&mut rng);

        //        println!("{:#?}", cards);
        cards
    }

    pub fn strength(&self) -> u8 {
        match self.value {
            1 => 10, // Ás
            7 => 9,  // Sete
            13 => 8, // Rei
            11 => 7, // Valete (na Sueca, Valete ganha da Dama)
            12 => 6, // Dama
            6 => 5,
            5 => 4,
            4 => 3,
            3 => 2,
            2 => 1,
            _ => 0,
        }
    }

    pub fn point(&self) -> u8 {
        match self.value {
            1 => 11, // Ás
            7 => 10, // Sete
            13 => 4, // Rei
            11 => 3, // Valete (na Sueca, Valete ganha da Dama)
            12 => 2, // Dama
            _ => 0,
        }
    }

    pub fn calculate_winer(trick: &[(String, Card)], trump_suit: Suit) -> (String, Card) {
        let first_card = trick[0].1;
        let lead_suit = first_card.suit;

        trick
            .iter()
            .max_by_key(|(_, card)| {
                let mut score = card.strength();

                if card.suit == trump_suit {
                    score += 20;
                } else if card.suit == lead_suit {
                    score += 10;
                }

                score
            })
            .unwrap()
            .clone()
    }

    pub fn distribute_cards(mut deck: Vec<Card>, players: Vec<String>) -> Vec<(String, Vec<Card>)> {
        let mut player_cards: Vec<(String, Vec<Card>)> = Vec::new();

        for p in players {
            let mut hand: Vec<Card> = Vec::new();
            for _ in 0..10 {
                if let Some(card) = deck.pop() {
                    hand.push(card);
                }
            }
            player_cards.push((p, hand));
        }
        player_cards
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_print() {
        Card::return_shuffled_deck();
    }
}
