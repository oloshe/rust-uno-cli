use std::{collections::HashMap, fmt::Display, usize};
use core::result::Result;
use console::style;
use rand::{prelude::SliceRandom, thread_rng};
use crate::base::card::*;
use crate::base::player::*;
use crate::base::game_data::*;
#[derive(Debug)]
enum GameState {
    Ready,
    Started,
}

#[derive(Debug)]
enum Direction {
    /// 顺时针
    Clockwise,
    /// 逆时针
    Counterclockwise
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
    players: HashMap<String, &'a Player>,
    game_data: HashMap<String, GameData>,
    direction: Direction,
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
            direction: Direction::Clockwise,
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

     /// 添加玩家
     pub fn add_player(&mut self, p: &'a Player) {
        let id = p.get_id();
        if self.players.contains_key(id) {
            // println!("> {} {}", style(p.get_name()).blue(), style("试图重复加入房间").black());
        } else {
            let _id_str = || id.to_string();
            println!(
                "> {} {} 加入了游戏",
                 style(p.get_name()).blue(), 
                 style(format!("({})", _id_str())).yellow()
            );
            self.players.insert(_id_str(), p);
            self.game_data.insert(_id_str(), GameData::new(_id_str()));
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

     /// 玩家数量
     pub fn player_count(&self) -> usize {
         self.players.len()
     }

     /// 开始游戏
     ///
     /// 玩家们随意指定一个玩家作为庄家。随后每人取牌7张，其余作为牌堆。庄家从牌堆中取出首张数字牌（若不是数字牌则再取），并依据此牌出牌，游戏开始。出牌的初始顺序是顺时针。
     pub fn start_game(&mut self) -> Result<(), &str> {
         let count = self.player_count();
         if count < 2 {
            return Err("> 人数不足无法开始")
         }
         self.game_state = GameState::Started;
         self.reset_game_data();
         self.shuffle_card();
         self.distribute_card();
         println!("
         游戏开始：
         玩家数量：{}
         ", count);
         Ok(())
     }

     /// 初始化游戏数据
     fn reset_game_data(&mut self) {
        self.game_data.clear();
        for (id, _) in self.players.iter_mut() {
            self.game_data.insert(id.to_string(), GameData::new(id.to_string()));
        }
     }

     /// 洗牌
     pub fn shuffle_card(&mut self) {
        self.card_stack.shuffle(&mut thread_rng());
     }

     /// 发牌
     fn distribute_card(&mut self) {
        // let mut players = &self.players;
        for (_, p) in self.game_data.iter_mut() {
            for _ in 0..7 {
                p.add_card(
                    self.card_stack.pop().expect("error")
                );
            }
        }
     }

     /// 抽卡
     pub fn get_a_card(&mut self) -> Option<Card> {
        self.card_stack.pop()
     }

     pub fn print_game_data(&self) {
         for (id, data) in self.game_data.iter() {
            println!("
            id: {}
            牌: {}
            ", id, data);
         }
     }
 }

 impl<'a> Display for Dealer<'a> {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "
        剩余牌数量：{}
        已出牌数量: {}

{:#?}
        ", self.count(), self.used_count(), self.game_data)
    }
 }
