use std::{net::{TcpListener, ToSocketAddrs}, thread::spawn};

pub struct LocalServer {
    server: TcpListener
}

impl LocalServer {
    pub fn new(port: u16) -> LocalServer {
        let addr = format!("127.0.0.1:{}", port);
        let server = TcpListener::bind(addr).unwrap();
        LocalServer{ server }
    }
    pub fn listen(&self, task_callback: fn()) {
        for steam in self.server.incoming() {
            spawn(move || {
                match steam {
                    Ok(_) => task_callback(),
                    Err(e) => eprintln!("{}", e),
                }
            });
        }
    }
}