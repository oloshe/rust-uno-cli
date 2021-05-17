use std::fmt::Display;

use super::card::Card;
#[derive(Debug)]
pub struct GameData {
    /// 玩家标识
    owner: String,
    /// 玩家手上的卡
    cards: Vec<Card>
}

impl GameData {
    pub fn new(id: String) -> GameData {
        GameData {
            owner: id,
            cards: vec![],
        }
    }
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }
}

impl Display for GameData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ret = self.cards.iter().fold(String::new(), |_, &card| {
            format!("{:?} {}", card.card_type(), card.card_color())
        });

        write!(f, "{}", ret)
    }
}