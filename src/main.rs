use loopper::Loopper;

mod net;
mod base;
mod dealer;
mod loopper;
fn main() {
//     let mut dealer = dealer::Dealer::new();
//     println!("{}", dealer);
//     let player1 = base::player::Player::new("1", "张三");
//     let player2 = base::player::Player::new("3", "李四");
//     dealer.add_player(&player1);
//     dealer.add_player(&player2);
//     // dealer.add_player(&player2);
//     match dealer.start_game() {
//         Err(str) => println!("{}", str),
//         _ => (),
//     }
//     println!("{}", dealer);
    let mut looper = Loopper::new();
    looper.bootstrap();
}