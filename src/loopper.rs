use std::{collections::HashMap, io::{self, Result}, ops::Index, thread, time::Duration};

use console::{Emoji, Term, style};
use dialoguer::{Input, Select, theme::ColorfulTheme};
use indicatif::ProgressBar;

use crate::{base::player::Player, net::client::Client};

struct GlobalData {

}

pub struct Loopper {
    user: Player,
    client: Client,
}
impl Loopper{
    pub fn new() -> Loopper {
        println!("> 本地地址：{}", style(local_ipaddress::get().unwrap()).blue());
        let addr = "127.0.0.1:24404";
        println!("> 连接服务器: {}", style(addr).blue());
        Loopper{
            user: Player::new("0", "nick"),
            client: Client::connect(addr),
        }
    }
    pub fn bootstrap(&mut self) {
        self.login().expect("error")
    }
    fn login(&mut self) -> Result<()> {
        self.main_menu().expect("error");
        Ok(())
    }
    fn main_menu(&self) -> Result<()>{
        let items = vec![
            "房间列表",
            "帮助",
            "设置",
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
                    2 => self.setting()?,
                    _ => break,
                },
                None => continue
            }
        }
        Ok(())
    }
    fn setting(&mut self) -> Result<()> {
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