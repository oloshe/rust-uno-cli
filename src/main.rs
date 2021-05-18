#![warn(unused)]

use std::thread;

use client::loopper::Loopper;
use server::server::Server;

mod net;
mod base;
mod client;
mod server;
mod dto;
#[async_std::main]
async fn main() {
    // let args: Vec<String> = std::env::args().collect();
    let port: u16 = 24404;
    let server_handle = Server::fork(port);
    let client_handle = thread::spawn(|| {
        let mut looper = Loopper::new();
        looper.bootstrap();
    });
    server_handle.join().unwrap();
    client_handle.join().unwrap();
}