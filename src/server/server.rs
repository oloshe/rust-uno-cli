use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, thread::{self, spawn}};

pub struct Server {

}

impl Server {
    pub fn serve(port: u16) {
        let addr = ("127.0.0.1", port);
        // println!("> 启动服务器...");
        let server = TcpListener::bind(addr).unwrap();
        // println!("> 启动成功");
        for ret in server.incoming() {
            match ret {
                Ok(stream) => handle_stream(stream),
                Err(e) => eprintln!("{}", e),
            }
        }
    }
    pub fn fork(port: u16) -> thread::JoinHandle<()> {
        let handle = spawn(move || {
            Self::serve(port)
        });
        handle
    }
}   

fn handle_stream(mut stream: TcpStream) {
    let mut buf = [0u8;1024];
    stream.read(&mut buf).unwrap();
    stream.write(&buf).unwrap();
}