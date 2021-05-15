// use std::{fmt::Display, io::{self, Result, Read}, net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream, ToSocketAddrs, UdpSocket}, str, thread::{self, spawn}, time::Duration};

// use console::{Emoji, style};
// use dialoguer::Input;

// static PORT: u16 = 24680;

// pub struct LAN {

// }

// impl LAN {
//     pub fn get_local_addr() -> String {
//         let addr = local_ipaddress::get().unwrap();
//         addr
//     }
//     /// 局域网起一个服务
//     pub fn serve() -> Result<()> {
//         let addr = ("127.0.0.1", PORT); //SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), PORT));
//         let socket = UdpSocket::bind(addr).expect("cannot bind this port");
//         let addr = LAN::get_local_addr();
//         println!("> {}创建局域网房间成功\nip地址: {}:{}", 
//         Emoji::new("🔈 ", ""),
//         style(&addr).blue().on_white(),
//         style(&PORT).blue().on_white(),
//         );
//         // let buf = addr.as_bytes();
//         // match socket.send(&buf) {
//         //     Ok(n) => {
//         //         println!("sent {} times", n);
//         //     },
//         //     Err(e) => return Err(e)
//         // }

//         // thread::spawn(move || {
//             loop {
//                 let mut buf = [0u8;1024];
//                 println!("监听返回...");
//                 let (_len, _src) = socket.recv_from(&mut buf).expect("recv error");
//                 // let buf = &mut buf[..len];
//                 let str = str::from_utf8(&buf).expect("str from uft8 err");
//                 println!("recv: {}", str);
//                 let _ = socket.send_to(&[1], _src).expect("send to error");
//             }
//         // });
//     }
//     /// 连接至局域网
//     pub fn connect<A: ToSocketAddrs + Display>(host_addr: A) -> Result<()> {
//         let socket = UdpSocket::bind("127.0.0.1:8080").expect("error bind");
//         println!("> 连接 {} ...", host_addr);
//         let _conn = socket.connect(host_addr);
//         if let Err(e) = _conn {
//             println!("errorrrrrr");
//             eprintln!("{}", e);
//             return Ok(())
//         }
//         println!("> {}连接至局域网 ip地址: {}", 
//         Emoji::new("📌 ", ""), 
//         style(socket.local_addr().unwrap()).red().on_white());

//         loop {
//             let mut input = String::new();
//             println!("请输入：");
//             io::stdin().read_line(&mut input)?;
//             socket.send(input.as_bytes())?;
//             println!("服务器返回：");
//             let mut buf = [0u8; 1024];
//             socket.recv_from(&mut buf)?;
//             println!("{}", str::from_utf8(&buf).expect("error"));
//         }
//     }

//     // pub fn bootstrap() -> Result<()> {
//     //     // 检查局域网是否有主机存在
//     //     let host_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), PORT)); // ("0.0.0.0", PORT);
//     //     let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
//     //     socket.set_read_timeout(Some(Duration::new(5, 0)))?;
//     //     // 本地地址
//     //     let local_addr = socket.local_addr().unwrap();
//     //     println!("> 本地地址：{}", local_addr);
//     //     socket.connect(host_addr).expect("连接失败");
//     //     println!("> {:?}" , socket.peer_addr());
//     //     let str = local_addr.to_string();
//     //     let str = format!("{} 想要上线了", str);
//     //     socket.send(str.as_bytes()).unwrap();
//     //     let mut buf = [0u8;1024];
//     //     let a = socket.recv(&mut buf);
//     //     if let Ok(usize) = a {
//     //         println!("> 响应: {}", usize);
//     //     } else {
//     //         println!(" 超时了")
//     //     }
//     //     Ok(())
//     // }


// }