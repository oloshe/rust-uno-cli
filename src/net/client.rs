use std::{io::{BufReader, BufWriter, Read, Write}, net::{TcpStream, ToSocketAddrs}};

use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};

pub enum ConnectionStatus {
    Succ,
    Fail,
}
pub struct Client {
    steam: Option<TcpStream>,
    status: ConnectionStatus,
}

impl Client {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Client {
        let result = TcpStream::connect(addr);
        if let Ok(s) = result {
            println!("> 连接成功\n");
            Client{
                status: ConnectionStatus::Succ,
                steam: Some(s),
            }
        } else {
            println!("> 连接失败\n");
            Client {
                status: ConnectionStatus::Fail,
                steam: None,
            }
        }
    }
    pub async fn send<'a, T, R, F>(&mut self, data: &'a T, buf: &'a mut [u8], f: F)
    where
        T: ?Sized + Serialize,
        R: Deserialize<'a>,
        F: FnOnce(R),
    {
        match self.steam.as_mut() {
            Some(steam) => {
                let result = serde_json::to_string(&data);
                if let Ok(json) = result {
                    steam.write(json.as_bytes()).unwrap();
                    // let mut buf = [0u8;1024];
                    steam.read(buf).unwrap();
                    let ret: R = serde_json::from_slice(buf).unwrap();
                    f(ret);
                } else {
                    // 转化为 json 错误
                    eprintln!("> 数据错误");
                }
            },
            None => {
                eprintln!("> 未连接到服务器");
            }
        }
    }
}
