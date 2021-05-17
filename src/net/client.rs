use std::net::{TcpStream, ToSocketAddrs};

use console::style;
use indicatif::{ProgressBar, ProgressStyle};

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
}
