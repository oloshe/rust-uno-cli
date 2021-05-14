use std::{io::{self, Read}, net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream, UdpSocket}, str, thread::spawn};

use console::{Emoji, style};
use dialoguer::Input;

static PORT: u16 = 24680;

pub struct LAN {

}

impl LAN {
    /// 局域网起一个服务
    pub fn serve() {
        let local_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), PORT));
        let socket = UdpSocket::bind(local_addr).expect("bind error");
        println!("> {}创建局域网主机成功", Emoji::new("🔈 ", ""));
        loop {
            let mut buf = [0u8;1024];
            println!("监听返回...");
            let (_len, _src) = socket.recv_from(&mut buf).expect("recv error");
            // let buf = &mut buf[..len];
            let str = str::from_utf8(&buf).expect("str from uft8 err");
            println!("recv: {}", str);
            let _ = socket.send_to(&[1], _src).expect("send to error");
        }
    }
    /// 连接至局域网
    pub fn connect() -> io::Result<()> {
        let local_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 10029));
        let host_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), PORT));
        let socket = UdpSocket::bind(local_addr).expect("error bind");
        socket.connect(host_addr).expect("connect error");
        println!("> {}连接至局域网", Emoji::new("📌 ", ""));
        loop {
            let mut input = String::new();
            println!("请输入：");
            io::stdin().read_line(&mut input)?;
            socket.send(input.as_bytes())?;
            println!("服务器返回：");
            let mut buf = [0u8; 1024];
            socket.recv_from(&mut buf)?;
            println!("{}", str::from_utf8(&buf).expect("error"));
        }
    }

    pub fn bootstrap() -> io::Result<()> {
        // 检查局域网是否有主机存在
        let host_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), PORT));
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        // 本地地址
        let local_addr = socket.local_addr().unwrap();
        println!("> 本地地址：{}", local_addr);
        socket.connect(host_addr).expect("连接失败");
        println!("> {:?}" , socket.peer_addr());
        let str = local_addr.to_string();
        let str = format!("{} 想要上线了", str);
        socket.send(str.as_bytes()).unwrap();
        Ok(())
    }
}