use std::{collections::HashMap, io, ops::Index, thread, time::Duration};

use console::{Emoji, Term, style};
use dialoguer::{Input, Select, theme::ColorfulTheme};
use indicatif::ProgressBar;

use crate::{base::player::Player, net::local_area_network::LAN};

pub struct Loopper<'a> {
    user: Player,
    menu: Vec<&'a str>,
}
impl<'a> Loopper<'a>{
    pub fn new() -> Loopper<'a> {
        Loopper{
            user: Player::new("0", "nick"),
            menu: vec![],
        }
    }
    pub fn bootstrap(&mut self) {
        self.login().expect("error")
    }
    fn set_menu(&mut self,v: Vec<&'a str>) {
        self.menu = v;
    }
    fn get_menu(&self) -> &Vec<&str> {
        &self.menu
    }
    fn login(&mut self) -> std::io::Result<()> {
        let id: String = Input::new()
            .with_prompt("> 请输入id")
            .interact()?;

        let name: String = Input::new()
            .with_prompt("> 请输入昵称")
            .interact()?;
        
        let id = id.trim();
        let name = name.trim();
        self.user = Player::new(id, name);
        println!("> {} ({}) {}", style(name).blue(), id, style("登录成功！").yellow());
        self.main_menu().expect("error");
        Ok(())
    }
    fn main_menu(&mut self) -> std::io::Result<()>{
        // let pb = ProgressBar::new(100);
        // for _ in 0..100 {
        //     pb.inc(1);
        //     thread::sleep(Duration::from_millis(3));
        // }
        // pb.finish_and_clear();
        self.set_menu(vec![
            "局域网",
            "联网大厅",
            "退出"
        ]);
        let items = self.get_menu();
        let selection  = Select::with_theme(&ColorfulTheme::default())
            .items(items)
            .default(0)
            .interact_on_opt(&Term::stderr())?;
        match selection {
            Some(index) => match index {
                0 => self.lan(),
                _ => println!("{}", index),
            },
            None => self.main_menu()?
        }
        Ok(())
    }
    fn lan(&self) {
        LAN::connect();
    }
}