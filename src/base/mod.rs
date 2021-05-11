pub mod card;
pub mod player;

#[derive(Debug)]
pub struct GameData<'a> {
    /// 玩家标识
    owner: &'a str,
    /// 玩家手上的卡
    cards: Vec<&'a card::Card>
}