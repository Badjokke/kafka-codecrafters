#![allow(unused_imports)]
use std::{io::{Read, Write}, net::TcpListener};
pub mod util;
use util::kafka_header_util;
use util::byte_util::stream_input_to_bytes;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
     for stream in listener.incoming() {
         match stream {
             Ok(mut stream) => {
                let buf = match stream_input_to_bytes(&mut stream){
                    Some(buffer) => buffer,
                    None => panic!("No input!")
                };
                println!("extracted buffer: {:?}", buf);
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}
