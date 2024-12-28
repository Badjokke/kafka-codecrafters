#![allow(unused_imports)]
use std::{io::Write, net::TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
     for stream in listener.incoming() {
         match stream {
             Ok(mut _stream) => {
                let a: i32 = 4;
               let _bytes_sent= _stream.write(&a.to_be_bytes()).unwrap();  
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}
