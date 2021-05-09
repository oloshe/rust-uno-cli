// use base::{Card,UniversalCard};

mod base;
mod dealer;
fn main() {
    // let a = Card::new_universal_card(UniversalCard::Plus4);
    // println!("{:?}", a);
    // let color  = a.card_color();
    // println!("{:?}", color);
    // println!("{:?}", a);
    let a = dealer::Dealer::new();
    println!("{}", a);
}