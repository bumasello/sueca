use common::{Card, RoomStatus};
use mongodb::Database;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct AppStruct {
    pub db: Database,
    pub collection_name: String,
    pub sessions: Arc<RwLock<HashMap<String, String>>>,
    pub rooms: Arc<RwLock<HashMap<String, RoomStruct>>>,
}

#[derive(Clone, Serialize, PartialEq)]
pub struct RoomStruct {
    pub room_id: String,
    pub players: Vec<String>,
    pub status: RoomStatus,
    pub deck: Vec<common::Card>,
    pub hands: HashMap<String, Vec<Card>>,
    pub trump: common::Suit,
    pub current_trick: Vec<(String, Card)>,
    pub turn_index: usize,
    pub starter_index: usize,
    pub scores: [u32; 2],
    pub created_at: u64,
}

impl RoomStruct {
    pub fn process_move(&mut self, player: String, card: Card) -> Result<(), String> {
        if self.status != RoomStatus::InGame {
            return Err("O jogo não está ativo".to_string());
        }

        if self.players[self.turn_index] != player {
            return Err("Não é sua vez".to_string());
        }

        let player_hand = self.hands.get_mut(&player).ok_or("Mão não encontrada")?;
        let card_pos = player_hand
            .iter()
            .position(|c| c == &card)
            .ok_or("Você não tem essa carta")?;

        if !self.current_trick.is_empty() {
            let lead_suit = self.current_trick[0].1.suit;
            if card.suit != lead_suit && player_hand.iter().any(|c| c.suit == lead_suit) {
                return Err("Você deve seguir o naipe puxado".to_string());
            }
        }
        player_hand.remove(card_pos);
        self.current_trick.push((player, card));

        if self.current_trick.len() == 4 {
            let (winner_name, _) =
                common::Card::calculate_winer(&self.current_trick, self.trump.clone());

            self.turn_index = self.players.iter().position(|p| p == &winner_name).unwrap();

            if self.hands.values().next().map_or(true, |h| h.is_empty()) {
                self.status = RoomStatus::Finished;
            };

            let mut points: u32 = 0;

            for (_, card) in self.current_trick.iter() {
                points += common::Card::point(card) as u32
            }

            let idx_winner = self.players.iter().position(|p| p == &winner_name).unwrap();

            if idx_winner % 2 == 0 {
                self.scores[0] += points;
            } else {
                self.scores[1] += points
            }
            self.current_trick.clear();
        } else {
            self.turn_index = (self.turn_index + 1) % 4;
        }

        Ok(())
    }
}
