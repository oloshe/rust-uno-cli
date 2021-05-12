use std::{collections::HashMap, io, ops::Index};

use console::{Emoji, style};

use crate::base::player::Player;

pub struct Loopper<'a> {
    user: Player,
    menu: Vec<&'a str>,
    pointer_index: u8,
}
impl<'a> Loopper<'a>{
    pub fn new() -> Loopper<'a> {
        Loopper{
            user: Player::new("0", "nick"),
            menu: vec![],
            pointer_index: 0,
        }
    }
    pub fn bootstrap(&mut self) {
        self.login()
    }
    fn set_menu(&mut self,v: Vec<&'a str>) {
        self.menu = v;
        self.pointer_index = 0;
    }
    fn update_menu(&self) {
        for (pos, &str) in self.menu.iter().enumerate() {
            let is_curr = pos == self.pointer_index as usize;
            println!("      {}{}", match is_curr {
                true => Emoji(" 👉", "->"),
                false => Emoji("",""),
            }, str)
        }
    }
    fn login(&mut self) {
        let mut id = String::new();
        let mut name = String::new();
        println!("> 请输入id:");
        loop {
            match io::stdin().read_line(&mut id) {
                Err(_) => continue,
                _ => break,
            }
        }
        println!("> 请输入昵称:");
        loop {
            match io::stdin().read_line(&mut name) {
                Err(_) => continue,
                _ => break,
            }
        }
        let id = id.trim();
        let name = name.trim();
        self.user = Player::new(id, name);
        println!("> {}", style("登录成功！").yellow());
        println!("> {} ({}) 加入了游戏", style(name).blue(), id);
        self.main_menu()
    }
    fn main_menu(&mut self) {
        self.set_menu(vec![
            "开始游戏"
        ]);
        // loop {
        //     self.update_menu()
        // }
    }
}