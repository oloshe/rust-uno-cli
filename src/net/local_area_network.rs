use std::{io::{self, Read}, net::{IpAddr, TcpListener, TcpStream, UdpSocket}, thread::spawn};

use console::style;

static PORT: u16 = 24680;

pub struct LAN {

}

impl LAN {
    pub fn connect() -> io::Result<()> {
        let addr = format!("0.0.0.0:{}", PORT);
        // let socket = UdpSocket::bind(addr);
        GetAdaptersAddresses();
        let socket = UdpSocket::bind(addr.clone()).expect("fail to bind");
        socket.set_broadcast(true).expect("fail to set boardcast");
        let aaa = socket.broadcast().expect("fail to broadcast");
        println!("{}", aaa);
        socket.connect(format!("255.255.255.255:{}", PORT)).expect("fail to connect");
        socket.send(&[0,1,2]).expect("fail to send");
        // if let Ok(mut stream) = TcpStream::connect(addr) {
        //     // 连接成功
        //     println!("连接成功");
        //     stream.set_nonblocking(true).expect("set_nonblocking call failed");

        //     let mut buf = vec![];
        //     loop {
        //         match stream.read_to_end(&mut buf) {
        //             Ok(_) => break,
        //             // Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
        //             //     // wait until network socket is ready, typically implemented
        //             //     // via platform-specific APIs such as epoll or IOCP
        //             //     wait_for_fd();
        //             // }
        //             Err(e) => panic!("encountered IO error: {}", e),
        //         };
        //     };
        //     println!("bytes: {:?}", buf);
        // } else {
        //     // 连接失败，本地创建服务器
        //     // println!("连接失败")
        //     let server = TcpListener::bind(format!("127.0.0.1:{}", PORT)).unwrap();
        //     println!("> {}", style("创建局域网服务器成功！").green().bold());

        //     for stream in server.incoming() {
        //         let handle = spawn(move || {
        //             match stream {
        //                 Ok(stream) => println!("{:?}", stream),
        //                 Err(e) => eprintln!("{}", e),
        //             }
        //         });
        //         // handle.join().unwrap();
        //     }
        //     println!("连接完毕");

        // }
        Ok(())
    }
}