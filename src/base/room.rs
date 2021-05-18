use std::rc::Rc;

use serde::{Serialize, Deserialize};

use super::player::Player;

#[derive(Serialize, Deserialize, Debug)]
pub enum RoomState {
    Ready,
    Start,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Room {
    id: String,
    name_cn: String,
    players: Vec<Player>,
    status: RoomState,
    pwd: bool,
}
struct RoomList (Vec<Room>);

impl RoomList {
    pub fn new(list: Vec<Room>) -> RoomList{
        RoomList(list)
    }
}