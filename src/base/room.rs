use super::player::Player;

#[derive(Debug)]
pub enum RoomState {
    Ready,
    Start,
}

#[derive(Debug)]
pub struct Room {
    id: String,
    name_cn: String,
    players: Vec<Player>,
    status: RoomState,
    pwd: bool,
}