use std::{collections::HashMap, io::{self, Result}, ops::Index, thread, time::Duration};

use console::{Emoji, Term, style};
use dialoguer::{Input, Select, theme::ColorfulTheme};
use indicatif::ProgressBar;

use crate::{base::player::Player, net::local_area_network::LAN};

pub struct Loopper {
    user: Player,
}
impl Loopper{
    pub fn new() -> Loopper {
        Loopper{
            user: Player::new("0", "nick"),
        }
    }
    pub fn bootstrap(&mut self) {
        self.login().expect("error")
    }
    fn login(&mut self) -> Result<()> {
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
    fn main_menu(&self) -> Result<()>{
        let items = vec![
            "局域网",
            "帮助",
            "退出"
        ];
        loop {
            let selection  = Select::with_theme(&ColorfulTheme::default())
                .items(&items)
                .default(0)
                .interact_on_opt(&Term::stderr())?;
            match selection {
                Some(index) => match index {
                    0 => self.lan_menu()?,
                    1 => println!("暂无帮助"),
                    _ => break,
                },
                None => continue
            }
        }
        Ok(())
    }
    fn lan_menu(&self) -> Result<()> {
        let items = vec![
            "加入房间",
            "创建房间",
            "返回"
        ];
        loop {
            let selectcion = Select::with_theme(&ColorfulTheme::default())
                .items(&items)
                .default(0)
                .interact_on_opt(&Term::stderr())?;
            match selectcion {
                Some(index) => match index {
                    0 => self.lan_join()?,
                    1 => self.lan_create()?,
                    _ => break,
                },
                None => continue,
            }
        }
        Ok(())
    }
    fn lan_join(&self) -> Result<()> {
        // LAN::connect().expect("asda");
        // let addr: String = Input::new()
        //     .with_prompt("> 请输入房间ip地址")
        //     .interact()?;
        // let addr = addr.trim();
        // LAN::connect(addr)
        Ok(())
    }
    fn lan_create(&self) -> Result<()> {
        // LAN::serve()?;
        // // self.main_menu()
        Ok(())
    }
}