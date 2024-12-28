#![allow(unused_imports)]
use std::{io::{Read, Write}, net::TcpListener};



fn parse_correlation_id_v2(buffer:&[u8]) -> i32{
    println!("{:?}",buffer);
   let byte_offset = 8;
   let correlation_id = i32::from_be_bytes(buffer[byte_offset..byte_offset+4].try_into().unwrap());
   println!("Extracted correlation id: {correlation_id}");
   return correlation_id; 
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
     for stream in listener.incoming() {
         match stream {
             Ok(mut _stream) => {
                let mut buf: [u8;20] = [0;20];
                let msg = _stream.read(&mut buf).unwrap();
                println!("recieved {msg} bytes");
                let correlation_id:i32 = parse_correlation_id_v2(&buf);
                let message_size:i32 = 4;
                let response = [message_size.to_be_bytes(), correlation_id.to_be_bytes()].concat(); 
                let bytes_send = _stream.write(&response).expect("Failed to send");
                println!("Sending response: {:?}", response);
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}
