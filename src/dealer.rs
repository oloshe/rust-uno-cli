use std::{collections::HashMap, fmt::Display, usize};
use console::style;
use crate::base::card::*;
use crate::base::player::*;
use crate::base::GameData;
#[derive(Debug)]
enum GameState {
    Ready,
    Started,
}
/// 发牌家
/// 108张牌
#[derive(Debug)]
pub struct Dealer<'a> {
    /// 牌堆
    card_stack: Vec<Card>,
    /// 已出的牌
    card_used: Vec<Card>,
    /// 游戏状态
    game_state: GameState,
    /// 玩家
    players: HashMap<String, &'a Player<'a>>,
    game_data: HashMap<String, GameData<'a>>,
}
impl<'a> Dealer<'a> {
     /// 新建一个发牌家
     /// 数字牌（76张）、功能牌（24张）、万能牌（8张）
     pub fn new() -> Dealer<'a> {
        let mut card_stack: Vec<Card> = Vec::new();
        
        Dealer::new_card(&mut card_stack);
        
        Dealer{
            card_stack,
            card_used: vec![],
            game_state: GameState::Ready,
            players: HashMap::new(),
            game_data: HashMap::new(),
        }
     }

     /// 新建牌堆
     fn new_card(card_stack: &mut Vec<Card>) {
        // 颜色
        let color_enums: [NormalColor;4] = [
            NormalColor::Blue,
            NormalColor::Red,
            NormalColor::Yellow,
            NormalColor::Green,
        ];

        // 0~9 数字牌
        for num in 0..10 {
            let is_double: bool = match num {
                0 => false,
                _ => true,
            };
            for &color in &color_enums {
                let card = Card::new_number_card(num.clone(), color);
                // println!("{}", card);
                card_stack.push(card);
                if is_double {
                    card_stack.push(card.clone());
                }
            }
        }

        // 功能牌
        let functional_enum: [FunctionalCard;3] = [
            FunctionalCard::Plus2,
            FunctionalCard::Reverse,
            FunctionalCard::Skip,
        ];
        for &functional_type in &functional_enum {
            for &color in &color_enums {
                let card = Card::new_functional_card(functional_type, color);
                // println!("{}", card);
                card_stack.push(card);
                card_stack.push(card.clone());
            }
        }

        // 万能牌
        for _ in 0..4 {
            let card = Card::new_universal_card(UniversalCard::ColorSwitch);
            card_stack.push(card);
            let card = Card::new_universal_card(UniversalCard::Plus4);
            card_stack.push(card);
        }
     }


     pub fn add_player(&mut self, p: &'a Player) {
        let id = p.get_id();
        if self.players.contains_key(id) {
            println!("{}", style("该玩家已存在").red());
        } else {
            let id = id.to_string();
            println!(
                "> {} {} 加入了游戏",
                 style(p.get_name()).blue(), 
                 style(format!("({})", id)).yellow()
            );
            self.players.insert(id, p);
        }
     }

     /// 可用牌堆数量
     pub fn count(&self) -> usize {
        self.card_stack.len()
     }

     /// 已出牌堆数量
     pub fn used_count(&self) -> usize {
         self.card_used.len()
     }

     pub fn player_count(&self) -> usize {
         self.players.len()
     }

     /// 开始游戏
     pub fn start_game(&mut self) {
         self.game_state = GameState::Started;
         println!("
         游戏开始：
         玩家数量：{}
         ", self.player_count());
     }
 }

 impl<'a> Display for Dealer<'a> {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "
        剩余牌数量：{}
        已出牌数量: {}
        ", self.count(), self.used_count())
    }
 }
