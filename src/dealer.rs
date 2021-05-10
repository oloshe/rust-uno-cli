use base::*;

use crate::base::NormalColor;

/// 发牌家
/// 108张牌
#[derive(Debug)]
pub struct Dealer {
    /// 牌堆
    card_stack: Vec<Card>,
    /// 已出的牌
    card_used: Vec<Card>,
}
impl Dealer {
     /// 新建一个发牌家
     /// 数字牌（76张）、功能牌（24张）、万能牌（8张）
     pub fn new() -> Dealer {
        // 颜色
        let color_enums: [NormalColor;4] = [
            NormalColor::Blue,
            NormalColor::Red,
            NormalColor::Yellow,
            NormalColor::Green,
        ];
        // 数字
        let number_enums: [NumberCard;10] = [
            NumberCard::Zero,
            NumberCard::One,
            NumberCard::Two,
            NumberCard::Three,
            NumberCard::Four,
            NumberCard::Five,
            NumberCard::Six,
            NumberCard::Seven,
            NumberCard::Eight,
            NumberCard::Nine
        ];
        let functional_enum: [FunctionalCard;3] = [
            FunctionalCard::Plus2,
            FunctionalCard::Reverse,
            FunctionalCard::Skip,
        ];
        let mut card_stack: Vec<Card> = Vec::new();

        // 0~9 数字牌
        for num in &number_enums {
            let is_double: bool = match num {
                NumberCard::Zero => false,
                _ => true,
            };
            for &color in &color_enums {
                let card = Card::new_number_card(num.clone(), color);
                println!("{}", card);
                card_stack.push(card);
                if is_double {
                    card_stack.push(card.clone());
                }
            }
        }

        for &functional_type in &functional_enum {
            for &color in &color_enums {
                let card = Card::new_functional_card(functional_type, color);
                card_stack.push(card);
                card_stack.push(card.clone());
            }
        }

        Dealer{
            card_stack,
            card_used: vec![],
        }
     }
 }
