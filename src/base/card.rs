use std::fmt::{Debug, Display};

#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum CardType {
    Number(u8),
    Functional(FunctionalCard),
    Universal(UniversalCard),
}

/// 功能牌
#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum FunctionalCard { Plus2, Reverse, Skip }

/// 万能牌
#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum UniversalCard { Plus4, ColorSwitch }

#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum CardColor {
    Normal(NormalColor),
    All,
}

impl Display for CardColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "GIAO")
        write!(f, "{:?}", match self {
            CardColor::All => "AA",
            CardColor::Normal(color) => color.to_string(),
        })
    }
}

#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum NormalColor { Yellow, Red, Blue, Green }

impl NormalColor {
    fn to_string(&self) -> &str {
        match self {
            NormalColor::Yellow => "Y",
            NormalColor::Green => "G",
            NormalColor::Blue => "B",
            NormalColor::Red => "R",
        }
    }
}


#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Card {
    card_color: CardColor,
    card_type: CardType,
}

impl Card {
    /// 创建一个数字牌
    pub fn new_number_card(num: u8, card_color: NormalColor) -> Card {
        if num >= 10 {
            panic!("Number Card is out of bound(0~9)!")
        }
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

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}-{}]", self.card_type(), self.card_color())
        // write!(f, "GIAO!!")
    }
}