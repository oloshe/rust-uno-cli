use std::fmt::Debug;

#[derive(Debug)]
pub enum CardType {
    Number(NumberCard),
    Functional(FunctionalCard),
    Universal(UniversalCard),
}
/// 数字牌
#[derive(Debug)]
pub enum NumberCard { One,Two,Three,Four,Five,Six,Seven,Eight,Nine }

/// 功能牌
#[derive(Debug)]
pub enum FunctionalCard { Plus2, Reverse, Skip }

/// 万能牌
#[derive(Debug)]
pub enum UniversalCard { Plus4, ColorSwitch }

#[derive(Debug)]
pub enum CardColor {
    Normal(NormalColor),
    All,
}

#[derive(Debug)]
pub enum NormalColor { Yellow, Red, Blue, Green }


#[derive(Debug)]
pub struct Card {
    card_color: CardColor,
    card_type: CardType,
}

impl Card {
    /// 创建一个数字牌
    pub fn new_number_card(num: NumberCard, card_color: NormalColor) -> Card {
        Card {
            card_type: CardType::Number(num),
            card_color: CardColor::Normal(card_color),
        }
    }
    /// 新建一张功能牌
    pub fn new_functional_card(functional_type: FunctionalCard, card_color: NormalColor) -> Card {
        Card{
            card_type: CardType::Functional(functional_type),
            card_color: CardColor::Normal(card_color),
        }
    }
    /// 新建一张万能牌
    pub fn new_universal_card(universal_type: UniversalCard) -> Card {
        Card {
            card_type: CardType::Universal(universal_type),
            card_color: CardColor::All,
        }
    }
    pub fn card_color(&self) -> &CardColor {
        &self.card_color
    }
    pub fn card_type(&self) -> &CardType {
        &self.card_type
    }
}
