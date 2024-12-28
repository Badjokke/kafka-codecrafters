#![allow(unused_imports)]
use std::{io::Write, net::TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
     for stream in listener.incoming() {
         match stream {
             Ok(mut _stream) => {
               let bytes_sent= _stream.write(&[0,0,7]).unwrap();  
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}
