
mod base;
mod dealer;
fn main() {
    let mut dealer = dealer::Dealer::new();
    println!("{}", dealer);
    let player1 = base::player::Player::new("1", "张三");
    let player2 = base::player::Player::new("3", "李四");
    dealer.add_player(&player1);
    dealer.add_player(&player2);
    dealer.start_game();
}